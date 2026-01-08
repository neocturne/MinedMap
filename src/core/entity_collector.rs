//! The [EntityCollector]

use std::path::Path;

use anyhow::{Context, Result};
use tracing::{info, warn};

use super::{common::*, tile_collector::TileCollector, tile_merger::TileMerger};
use crate::io::{fs, storage};

/// Generates mipmap tiles from full-resolution tile images
pub struct EntityCollector<'a> {
	/// Common MinedMap configuration from command line
	config: &'a Config,
	/// List of populated tiles for base mipmap level (level 0)
	regions: &'a [TileCoords],
}

impl TileMerger for EntityCollector<'_> {
	fn file_meta_version(&self) -> fs::FileMetaVersion {
		ENTITIES_FILE_META_VERSION
	}

	fn tile_path(&self, level: usize, coords: TileCoords) -> std::path::PathBuf {
		self.config.entities_path(level, coords)
	}

	fn write_tile(
		&self,
		file: &mut std::io::BufWriter<std::fs::File>,
		sources: &[super::tile_merger::Source],
	) -> Result<()> {
		Self::merge_entity_lists(file, sources.iter().map(|source| &source.1))
	}
}

impl TileCollector for EntityCollector<'_> {
	type CollectOutput = ();

	fn tiles(&self) -> &[TileCoords] {
		self.regions
	}

	fn prepare(&self, level: usize) -> Result<()> {
		fs::create_dir_all(&self.config.entities_dir(level))
	}

	fn finish(
		&self,
		_level: usize,
		_outputs: impl Iterator<Item = Self::CollectOutput>,
	) -> Result<()> {
		Ok(())
	}

	fn collect_one(
		&self,
		level: usize,
		coords: TileCoords,
		prev: &TileCoordMap,
	) -> Result<Self::CollectOutput> {
		self.merge_tiles(level, coords, prev)?;
		Ok(())
	}
}

impl<'a> EntityCollector<'a> {
	/// Constructs a new EntityCollector
	pub fn new(config: &'a Config, regions: &'a [TileCoords]) -> Self {
		EntityCollector { config, regions }
	}

	/// Merges multiple entity lists into one
	fn merge_entity_lists<P: AsRef<Path>>(
		file: &mut std::io::BufWriter<std::fs::File>,
		sources: impl Iterator<Item = P>,
	) -> Result<()> {
		let mut output = ProcessedEntities::default();

		for source_path in sources {
			let mut source: ProcessedEntities =
				match storage::read_file(source_path.as_ref(), storage::Format::Json) {
					Ok(source) => source,
					Err(err) => {
						warn!(
							"Failed to read entity data file {}: {:?}",
							source_path.as_ref().display(),
							err,
						);
						continue;
					}
				};

			output.block_entities.append(&mut source.block_entities);
		}

		storage::write(file, &output, storage::Format::Json).context("Failed to write entity data")
	}

	/// Runs the mipmap generation
	pub fn run(self) -> Result<()> {
		info!("Collecting entity data...");

		let tile_stack = self.collect_tiles()?;

		// Final merge
		let level = tile_stack.len() - 1;
		let tile_map = &tile_stack[level];
		let sources: Vec<_> = [(-1, -1), (-1, 0), (0, -1), (0, 0)]
			.into_iter()
			.map(|(x, z)| TileCoords { x, z })
			.filter(|&coords| tile_map.contains(coords))
			.map(|coords| self.tile_path(level, coords))
			.collect();

		fs::create_with_tmpfile(&self.config.entities_path_final, |file| {
			Self::merge_entity_lists(file, sources.iter())
		})?;

		info!("Collected entity data.");
		Ok(())
	}
}
