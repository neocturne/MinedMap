//! The [TileRenderer] and related types and functions

use std::{
	num::NonZeroUsize,
	path::PathBuf,
	sync::{Arc, Mutex},
	time::SystemTime,
};

use anyhow::{Context, Result};
use lru::LruCache;
use rayon::prelude::*;
use tokio::sync::OnceCell;
use tracing::{debug, info};

use super::{common::*, region_group::RegionGroup};
use crate::{
	io::{fs, storage},
	resource::{block_color, needs_biome, Colorf},
	types::*,
	util::coord_offset,
};

/// Type for referencing loaded [ProcessedRegion] data
type RegionRef = Arc<ProcessedRegion>;

/// Returns the index of the biome at a block coordinate
///
/// The passed chunk and block coordinates relative to the center of the
/// region group is offset by *dx* and *dz*.
///
/// The returned tuple contains the relative region coordinates the offset coordinate
/// ends up in (in the range -1..1) and the index in that region's biome list.
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

/// The TileRenderer generates map tiles from processed region data
pub struct TileRenderer<'a> {
	/// Common MinedMap configuration from command line
	config: &'a Config,
	/// Runtime for asynchronous region loading
	rt: &'a tokio::runtime::Runtime,
	/// List of populated regions to render tiles for
	regions: &'a [TileCoords],
	/// Set of populated regions for fast existence checking
	region_set: rustc_hash::FxHashSet<TileCoords>,
	/// Cache of previously loaded regions
	region_cache: Mutex<LruCache<PathBuf, Arc<OnceCell<RegionRef>>>>,
}

impl<'a> TileRenderer<'a> {
	/// Constructs a new TileRenderer
	pub fn new(
		config: &'a Config,
		rt: &'a tokio::runtime::Runtime,
		regions: &'a [TileCoords],
	) -> Self {
		let region_cache = Mutex::new(LruCache::new(
			NonZeroUsize::new(6 + 6 * config.num_threads).unwrap(),
		));
		let region_set = regions.iter().copied().collect();
		TileRenderer {
			config,
			rt,
			regions,
			region_set,
			region_cache,
		}
	}

	/// Loads [ProcessedRegion] for a region or returns previously loaded data from the region cache
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
				storage::read_file(&processed_path, storage::Format::Bincode)
					.context("Failed to load processed region data")
			})
			.await
			.cloned()
	}

	/// Loads a 3x3 neighborhood of processed region data
	async fn load_region_group(
		&self,
		processed_paths: RegionGroup<PathBuf>,
	) -> Result<RegionGroup<RegionRef>> {
		processed_paths
			.async_try_map(move |path| self.load_region(path))
			.await
	}

	/// Computes the color of a tile pixel
	fn block_color_at(
		region_group: &RegionGroup<RegionRef>,
		chunk: &ProcessedChunk,
		chunk_coords: ChunkCoords,
		block_coords: LayerBlockCoords,
	) -> Option<Colorf> {
		/// Helper for keys in the weight table
		///
		/// Hashing the value as a single u32 is more efficient than hashing
		/// the tuple elements separately.
		fn biome_key((dx, dz, index): (i8, i8, u16)) -> u32 {
			(dx as u8 as u32) | ((dz as u8 as u32) << 8) | ((index as u32) << 16)
		}

		/// One quadrant of the kernel used to smooth biome edges
		///
		/// The kernel is mirrored in X und Z direction to build the full 5x5
		/// smoothing kernel.
		const SMOOTH: [[f32; 3]; 3] = [[41.0, 26.0, 7.0], [26.0, 16.0, 4.0], [7.0, 4.0, 1.0]];
		/// Maximum X coordinate offset to take into account for biome smoothing
		const X: isize = SMOOTH[0].len() as isize - 1;
		/// Maximum Z coordinate offset to take into account for biome smoothing
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

		let mut color = Colorf::ZERO;
		let mut total = 0.0;

		for ((region_x, region_z, index), w) in weights.into_values() {
			let region = region_group.get(region_x, region_z)?;
			let biome = region.biome_list.get(usize::from(index))?;

			total += w;
			color += w * block_color(block, Some(biome), depth.0 as f32);
		}

		Some(color / total)
	}

	/// Renders a chunk subtile into a region tile image
	fn render_chunk(
		image: &mut image::RgbaImage,
		region_group: &RegionGroup<RegionRef>,
		chunk: &ProcessedChunk,
		chunk_coords: ChunkCoords,
	) {
		/// Width/height of a chunk subtile
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

	/// Renders a region tile image
	fn render_region(image: &mut image::RgbaImage, region_group: &RegionGroup<RegionRef>) {
		for (coords, chunk) in region_group.center().chunks.iter() {
			let Some(chunk) = chunk else {
				continue;
			};

			Self::render_chunk(image, region_group, chunk, coords);
		}
	}

	/// Returns the filename of the processed data for a region and the time of its last modification
	fn processed_source(&self, coords: TileCoords) -> Result<(PathBuf, SystemTime)> {
		let path = self.config.processed_path(coords);
		let timestamp = fs::modified_timestamp(&path)?;
		Ok((path, timestamp))
	}

	/// Returns the filenames of the processed data for a 3x3 neighborhood of a region
	/// and the time of last modification for any of them
	fn processed_sources(&self, coords: TileCoords) -> Result<(RegionGroup<PathBuf>, SystemTime)> {
		let sources = RegionGroup::new(|x, z| {
			Some(TileCoords {
				x: coords.x + (x as i32),
				z: coords.z + (z as i32),
			})
			.filter(|entry| self.region_set.contains(entry))
		})
		.try_map(|entry| self.processed_source(entry))
		.with_context(|| format!("Region {:?} from previous step must exist", coords))?;

		let max_timestamp = *sources
			.iter()
			.map(|(_, timestamp)| timestamp)
			.max()
			.expect("at least one timestamp must exist");

		let paths = sources.map(|(path, _)| path);
		Ok((paths, max_timestamp))
	}

	/// Renders and saves a region tile image
	fn render_tile(&self, coords: TileCoords) -> Result<bool> {
		/// Width/height of a tile image
		const N: u32 = (BLOCKS_PER_CHUNK * CHUNKS_PER_REGION) as u32;

		let (processed_paths, processed_timestamp) = self.processed_sources(coords)?;

		let output_path = self.config.tile_path(TileKind::Map, 0, coords);
		let output_timestamp = fs::read_timestamp(&output_path, MAP_FILE_META_VERSION);

		if Some(processed_timestamp) <= output_timestamp {
			debug!(
				"Skipping unchanged tile {}",
				output_path
					.strip_prefix(&self.config.output_dir)
					.expect("tile path must be in output directory")
					.display(),
			);
			return Ok(false);
		}

		debug!(
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
			MAP_FILE_META_VERSION,
			processed_timestamp,
			|file| {
				image
					.write_to(file, self.config.tile_image_format())
					.context("Failed to save image")
			},
		)?;

		Ok(true)
	}

	/// Runs the tile generation
	pub fn run(self) -> Result<()> {
		fs::create_dir_all(&self.config.tile_dir(TileKind::Map, 0))?;

		info!("Rendering map tiles...");

		// Use par_bridge to process items in order (for better use of region cache)
		let processed = self
			.regions
			.iter()
			.par_bridge()
			.map(|&coords| {
				anyhow::Ok(usize::from(
					self.render_tile(coords)
						.with_context(|| format!("Failed to render tile {:?}", coords))?,
				))
			})
			.try_reduce(|| 0, |a, b| Ok(a + b))?;

		info!(
			"Rendered map tiles ({} processed, {} unchanged)",
			processed,
			self.regions.len() - processed,
		);

		Ok(())
	}
}
