use anyhow::{Context, Result};

use minedmap::{io::fs, types::*};

use super::common::*;

pub struct TileMipmapper<'a> {
	config: &'a Config,
}

impl<'a> TileMipmapper<'a> {
	pub fn new(config: &'a Config) -> Self {
		TileMipmapper { config }
	}

	fn done(tiles: &TileCoordMap) -> bool {
		tiles
			.0
			.iter()
			.all(|(z, xs)| (-1..=0).contains(z) && xs.iter().all(|x| (-1..=0).contains(x)))
	}

	fn map_coords(tiles: &TileCoordMap) -> TileCoordMap {
		let mut ret = TileCoordMap::default();

		for (&z, xs) in &tiles.0 {
			for &x in xs {
				let xt = x >> 1;
				let zt = z >> 1;

				ret.0.entry(zt).or_default().insert(xt);
			}
		}

		ret
	}

	fn render_mipmap<P: image::PixelWithColorType>(
		&self,
		kind: TileKind,
		level: usize,
		coords: TileCoords,
		prev: &TileCoordMap,
	) -> Result<()>
	where
		[P::Subpixel]: image::EncodableLayout,
		image::ImageBuffer<P, Vec<P::Subpixel>>: Into<image::DynamicImage>,
	{
		const N: u32 = (BLOCKS_PER_CHUNK * CHUNKS_PER_REGION) as u32;

		let output_path = self.config.tile_path(kind, level, coords);

		println!(
			"Rendering mipmap tile {}",
			output_path
				.strip_prefix(&self.config.output_dir)
				.expect("tile path must be in output directory")
				.display(),
		);

		let mut image: image::DynamicImage =
			image::ImageBuffer::<P, Vec<P::Subpixel>>::new(N, N).into();

		let sources: Vec<_> = [(0, 0), (0, 1), (1, 0), (1, 1)]
			.into_iter()
			.filter_map(|(dx, dz)| {
				let source_coords = TileCoords {
					x: 2 * coords.x + dx,
					z: 2 * coords.z + dz,
				};
				if !prev.contains(source_coords) {
					return None;
				}

				let source_path = self.config.tile_path(kind, level - 1, source_coords);
				let timestamp = match fs::modified_timestamp(&source_path) {
					Ok(timestamp) => timestamp,
					Err(err) => {
						eprintln!("{}", err);
						return None;
					}
				};
				Some(((dx, dz), source_path, timestamp))
			})
			.collect();

		let Some(timestamp) = sources.iter().map(|(_, _, ts)| *ts).max() else {
			return Ok(());
		};

		for ((dx, dz), source_path, _) in sources {
			let source = match image::open(&source_path) {
				Ok(source) => source,
				Err(err) => {
					eprintln!(
						"Failed to read source image {}: {}",
						source_path.display(),
						err,
					);
					continue;
				}
			};
			let resized = source.resize(N / 2, N / 2, image::imageops::FilterType::Triangle);
			image::imageops::overlay(
				&mut image,
				&resized,
				dx as i64 * (N / 2) as i64,
				dz as i64 * (N / 2) as i64,
			);
		}

		fs::create_with_timestamp(&output_path, timestamp, |file| {
			image
				.write_to(file, image::ImageFormat::Png)
				.context("Failed to save image")
		})
	}

	pub fn run(self, tiles: &[TileCoords]) -> Result<Vec<TileCoordMap>> {
		let mut tile_stack = {
			let mut tile_map = TileCoordMap::default();

			for &TileCoords { x, z } in tiles {
				tile_map.0.entry(z).or_default().insert(x);
			}

			vec![tile_map]
		};

		loop {
			let level = tile_stack.len();
			let prev = &tile_stack[level - 1];
			if Self::done(prev) {
				break;
			}

			fs::create_dir_all(&self.config.tile_dir(TileKind::Map, level))?;
			fs::create_dir_all(&self.config.tile_dir(TileKind::Lightmap, level))?;

			let next = Self::map_coords(prev);

			for (&z, xs) in &next.0 {
				for &x in xs {
					let coords = TileCoords { x, z };
					self.render_mipmap::<image::Rgba<u8>>(TileKind::Map, level, coords, prev)?;
					self.render_mipmap::<image::LumaA<u8>>(
						TileKind::Lightmap,
						level,
						coords,
						prev,
					)?;
				}
			}

			tile_stack.push(next);
		}

		Ok(tile_stack)
	}
}
