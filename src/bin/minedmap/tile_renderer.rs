use std::{
	path::{Path, PathBuf},
	time::SystemTime,
};

use anyhow::{Context, Result};
use glam::Vec3;
use num_integer::div_mod_floor;

use minedmap::{
	io::{fs, storage},
	resource::{block_color, needs_biome},
	types::*,
};

use super::{common::*, region_group::RegionGroup};

/// Offsets a chunk and block coordinate pair by a number of blocks
///
/// As the new coordinate may end up in a different region, a region offset
/// is returned together with the new chunk and block coordinates.
fn coord_offset<const AXIS: u8>(
	chunk: ChunkCoord<AXIS>,
	block: BlockCoord<AXIS>,
	offset: i32,
) -> (i8, ChunkCoord<AXIS>, BlockCoord<AXIS>) {
	const CHUNKS: i32 = CHUNKS_PER_REGION as i32;
	const BLOCKS: i32 = BLOCKS_PER_CHUNK as i32;
	let coord = chunk.0 as i32 * BLOCKS + block.0 as i32 + offset;
	let (region_chunk, block) = div_mod_floor(coord, BLOCKS);
	let (region, chunk) = div_mod_floor(region_chunk, CHUNKS);
	(
		region
			.try_into()
			.expect("the region coordinate should be in the valid range"),
		ChunkCoord::new(chunk),
		BlockCoord::new(block),
	)
}

fn biome_at(
	region_group: &RegionGroup<ProcessedRegion>,
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
}

impl<'a> TileRenderer<'a> {
	pub fn new(config: &'a Config) -> Self {
		TileRenderer { config }
	}

	fn load_region(processed_path: &Path) -> Result<ProcessedRegion> {
		storage::read(processed_path).context("Failed to load processed region data")
	}

	fn load_region_group(
		processed_paths: RegionGroup<PathBuf>,
	) -> Result<RegionGroup<ProcessedRegion>> {
		processed_paths.try_map(|path| Self::load_region(&path))
	}

	fn block_color_at(
		region_group: &RegionGroup<ProcessedRegion>,
		chunk: &ProcessedChunk,
		chunk_coords: ChunkCoords,
		block_coords: LayerBlockCoords,
	) -> Option<Vec3> {
		const SMOOTH: [[f32; 3]; 3] = [[41.0, 26.0, 7.0], [26.0, 16.0, 4.0], [7.0, 4.0, 1.0]];
		const X: isize = SMOOTH[0].len() as isize - 1;
		const Z: isize = SMOOTH.len() as isize - 1;

		let block = chunk.blocks[block_coords]?;
		let depth = chunk.depths[block_coords]?;

		if !needs_biome(block) {
			return Some(block_color(block, None, depth.0 as f32));
		}

		let mut weights = rustc_hash::FxHashMap::<(i8, i8, u16), f32>::default();
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

				*weights.entry(biome).or_default() += w;
			}
		}

		if weights.is_empty() {
			return None;
		}

		let mut color = Vec3::ZERO;
		let mut total = 0.0;

		for ((region_x, region_z, index), w) in weights {
			let region = region_group.get(region_x, region_z)?;
			let biome = region.biome_list.get_index(index.into())?;

			total += w;
			color += w * block_color(block, Some(biome), depth.0 as f32);
		}

		Some(color / total)
	}

	fn render_chunk(
		image: &mut image::RgbaImage,
		region_group: &RegionGroup<ProcessedRegion>,
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

	fn render_region(image: &mut image::RgbaImage, region_group: &RegionGroup<ProcessedRegion>) {
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

		let region_group = Self::load_region_group(processed_paths)
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

		for &coords in regions {
			if let Err(err) = self.render_tile(coords) {
				eprintln!("Failed to render tile {:?}: {:?}", coords, err);
			}
		}

		Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_coord_offset() {
		const CHUNKS: i32 = CHUNKS_PER_REGION as i32;
		const BLOCKS: i32 = BLOCKS_PER_CHUNK as i32;

		for chunk in ChunkX::iter() {
			for block in BlockX::iter() {
				assert_eq!(coord_offset(chunk, block, 0), (0, chunk, block));
				assert_eq!(
					coord_offset(chunk, block, -(CHUNKS * BLOCKS)),
					(-1, chunk, block)
				);
				assert_eq!(
					coord_offset(chunk, block, CHUNKS * BLOCKS),
					(1, chunk, block)
				);

				for offset in -(CHUNKS * BLOCKS)..(CHUNKS * BLOCKS) {
					let (region2, chunk2, block2) = coord_offset(chunk, block, offset);
					assert!((-1..=1).contains(&region2));
					let coord = chunk.0 as i32 * BLOCKS + block.0 as i32 + offset;
					let coord2 =
						((region2 as i32 * CHUNKS) + chunk2.0 as i32) * BLOCKS + block2.0 as i32;
					assert_eq!(coord2, coord);
				}
			}
		}
	}
}
