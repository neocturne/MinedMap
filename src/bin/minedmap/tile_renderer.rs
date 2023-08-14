use std::{
	num::NonZeroUsize,
	path::PathBuf,
	sync::{Arc, Mutex},
	time::SystemTime,
};

use anyhow::{Context, Result};
use glam::Vec3;
use lru::LruCache;
use rayon::prelude::*;
use tokio::sync::OnceCell;

use minedmap::{
	io::{fs, storage},
	resource::{block_color, needs_biome},
	types::*,
	util::coord_offset,
};

use super::{common::*, region_group::RegionGroup};

type RegionRef = Arc<ProcessedRegion>;

fn biome_at(
	region_group: &RegionGroup<RegionRef>,
	chunk: ChunkCoords,
	block: LayerBlockCoords,
	dx: i32,
	dz: i32,
) -> Option<(i8, i8, u16)> {
	let (region_x, chunk_x, block_x) = coord_offset(chunk.x, block.x, dx);
	let (region_z, chunk_z, block_z) = coord_offset(chunk.z, block.z, dz);
	let chunk = ChunkCoords {
		x: chunk_x,
		z: chunk_z,
	};
	let block = LayerBlockCoords {
		x: block_x,
		z: block_z,
	};
	let region = region_group.get(region_x, region_z)?;
	Some((
		region_x,
		region_z,
		region.chunks[chunk].as_ref()?.biomes[block]?.get() - 1,
	))
}

pub struct TileRenderer<'a> {
	config: &'a Config,
	rt: &'a tokio::runtime::Runtime,
	region_cache: Mutex<LruCache<PathBuf, Arc<OnceCell<RegionRef>>>>,
}

impl<'a> TileRenderer<'a> {
	pub fn new(config: &'a Config, rt: &'a tokio::runtime::Runtime) -> Self {
		let region_cache = Mutex::new(LruCache::new(
			NonZeroUsize::new(6 + 6 * config.num_threads).unwrap(),
		));
		TileRenderer {
			config,
			rt,
			region_cache,
		}
	}

	async fn load_region(&self, processed_path: PathBuf) -> Result<RegionRef> {
		let region_loader = {
			let mut region_cache = self.region_cache.lock().unwrap();
			if let Some(region_loader) = region_cache.get(&processed_path) {
				Arc::clone(region_loader)
			} else {
				let region_loader = Default::default();
				region_cache.put(processed_path.clone(), Arc::clone(&region_loader));
				region_loader
			}
		};

		region_loader
			.get_or_try_init(|| async {
				storage::read(&processed_path).context("Failed to load processed region data")
			})
			.await
			.cloned()
	}

	async fn load_region_group(
		&self,
		processed_paths: RegionGroup<PathBuf>,
	) -> Result<RegionGroup<RegionRef>> {
		processed_paths
			.async_try_map(move |path| self.load_region(path))
			.await
	}

	fn block_color_at(
		region_group: &RegionGroup<RegionRef>,
		chunk: &ProcessedChunk,
		chunk_coords: ChunkCoords,
		block_coords: LayerBlockCoords,
	) -> Option<Vec3> {
		fn biome_key((dx, dz, index): (i8, i8, u16)) -> u32 {
			(dx as u8 as u32) | (dz as u8 as u32) << 8 | (index as u32) << 16
		}

		const SMOOTH: [[f32; 3]; 3] = [[41.0, 26.0, 7.0], [26.0, 16.0, 4.0], [7.0, 4.0, 1.0]];
		const X: isize = SMOOTH[0].len() as isize - 1;
		const Z: isize = SMOOTH.len() as isize - 1;

		let block = chunk.blocks[block_coords]?;
		let depth = chunk.depths[block_coords]?;

		if !needs_biome(block) {
			return Some(block_color(block, None, depth.0 as f32));
		}

		let mut weights = rustc_hash::FxHashMap::<u32, ((i8, i8, u16), f32)>::default();
		for dz in -Z..=Z {
			for dx in -X..=X {
				let w = SMOOTH[dz.unsigned_abs()][dx.unsigned_abs()];
				if w == 0.0 {
					continue;
				}

				let Some(biome) = biome_at(
					region_group,
					chunk_coords,
					block_coords,
					dx as i32,
					dz as i32,
				) else {
					continue;
				};

				let value = weights.entry(biome_key(biome)).or_default();
				value.0 = biome;
				value.1 += w;
			}
		}

		if weights.is_empty() {
			return None;
		}

		let mut color = Vec3::ZERO;
		let mut total = 0.0;

		for ((region_x, region_z, index), w) in weights.into_values() {
			let region = region_group.get(region_x, region_z)?;
			let biome = region.biome_list.get_index(index.into())?;

			total += w;
			color += w * block_color(block, Some(biome), depth.0 as f32);
		}

		Some(color / total)
	}

	fn render_chunk(
		image: &mut image::RgbaImage,
		region_group: &RegionGroup<RegionRef>,
		chunk: &ProcessedChunk,
		chunk_coords: ChunkCoords,
	) {
		const N: u32 = BLOCKS_PER_CHUNK as u32;

		let chunk_image = image::RgbaImage::from_fn(N, N, |x, z| {
			let block_coords = LayerBlockCoords {
				x: BlockX::new(x),
				z: BlockZ::new(z),
			};
			let color = Self::block_color_at(region_group, chunk, chunk_coords, block_coords);
			image::Rgba(
				color
					.map(|c| [c[0] as u8, c[1] as u8, c[2] as u8, 255])
					.unwrap_or_default(),
			)
		});
		overlay_chunk(image, &chunk_image, chunk_coords);
	}

	fn render_region(image: &mut image::RgbaImage, region_group: &RegionGroup<RegionRef>) {
		for (coords, chunk) in region_group.center().chunks.iter() {
			let Some(chunk) = chunk else {
				continue;
			};

			Self::render_chunk(image, region_group, chunk, coords);
		}
	}

	fn processed_source(&self, coords: TileCoords) -> Result<(PathBuf, SystemTime)> {
		let path = self.config.processed_path(coords);
		let timestamp = fs::modified_timestamp(&path)?;
		Ok((path, timestamp))
	}

	fn processed_sources(&self, coords: TileCoords) -> Result<(RegionGroup<PathBuf>, SystemTime)> {
		let sources = RegionGroup::new(|x, z| {
			self.processed_source(TileCoords {
				x: coords.x + (x as i32),
				z: coords.z + (z as i32),
			})
		})
		.with_context(|| format!("Region {:?} from previous step must exist", coords))?;

		let max_timestamp = *sources
			.iter()
			.map(|(_, timestamp)| timestamp)
			.max()
			.expect("at least one timestamp must exist");

		let paths = sources.map(|(path, _)| path);
		Ok((paths, max_timestamp))
	}

	fn render_tile(&self, coords: TileCoords) -> Result<()> {
		const N: u32 = (BLOCKS_PER_CHUNK * CHUNKS_PER_REGION) as u32;

		let (processed_paths, processed_timestamp) = self.processed_sources(coords)?;

		let output_path = self.config.tile_path(TileKind::Map, 0, coords);
		let output_timestamp = fs::read_timestamp(&output_path, FILE_META_VERSION);

		if Some(processed_timestamp) <= output_timestamp {
			println!(
				"Skipping unchanged tile {}",
				output_path
					.strip_prefix(&self.config.output_dir)
					.expect("tile path must be in output directory")
					.display(),
			);
			return Ok(());
		}

		println!(
			"Rendering tile {}",
			output_path
				.strip_prefix(&self.config.output_dir)
				.expect("tile path must be in output directory")
				.display(),
		);

		let region_group = self
			.rt
			.block_on(self.load_region_group(processed_paths))
			.with_context(|| format!("Region {:?} from previous step must be loadable", coords))?;
		let mut image = image::RgbaImage::new(N, N);
		Self::render_region(&mut image, &region_group);

		fs::create_with_timestamp(
			&output_path,
			FILE_META_VERSION,
			processed_timestamp,
			|file| {
				image
					.write_to(file, image::ImageFormat::Png)
					.context("Failed to save image")
			},
		)
	}

	pub fn run(self, regions: &[TileCoords]) -> Result<()> {
		fs::create_dir_all(&self.config.tile_dir(TileKind::Map, 0))?;

		// Use par_bridge to process items in order (for better use of region cache)
		regions.iter().par_bridge().for_each(|&coords| {
			if let Err(err) = self.render_tile(coords) {
				eprintln!("Failed to render tile {:?}: {:?}", coords, err);
			}
		});

		Ok(())
	}
}
