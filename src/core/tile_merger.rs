//! Mipmap-style merging of tiles

use std::{
	fs::File,
	io::BufWriter,
	path::{Path, PathBuf},
	time::SystemTime,
};

use anyhow::Result;
use tracing::warn;

use super::common::*;
use crate::io::fs;

/// [TileMerger::merge_tiles] return
#[derive(Debug, Clone, Copy)]
pub enum Stat {
	/// None of the input files were found
	NotFound,
	/// The output file is up-to-date
	Skipped,
	/// The output file is regenerated
	Regenerate,
}

/// A source file for the [TileMerger]
///
/// The tuple elements are X and Z coordinate offsets in the range [0, 1],
/// the file path and the time of last change of the input.
pub type Source = ((i32, i32), PathBuf, SystemTime);

/// Reusable trait for mipmap-style tile merging with change tracking
pub trait TileMerger {
	/// [fs::FileMetaVersion] of input and output files
	///
	/// The version in the file metadata on disk must match the returned
	/// version for the a to be considered up-to-date.
	fn file_meta_version(&self) -> fs::FileMetaVersion;

	/// Returns the paths of input and output files
	fn tile_path(&self, level: usize, coords: TileCoords) -> PathBuf;

	/// Can be used to log the processing status
	fn log(&self, _output_path: &Path, _stat: Stat) {}

	/// Handles the actual merging of source files
	fn write_tile(&self, file: &mut BufWriter<File>, sources: &[Source]) -> Result<()>;

	/// Generates a tile at given coordinates and mipmap level
	fn merge_tiles(&self, level: usize, coords: TileCoords, prev: &TileCoordMap) -> Result<Stat> {
		let version = self.file_meta_version();
		let output_path = self.tile_path(level, coords);
		let output_timestamp = fs::read_timestamp(&output_path, version);

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

				let source_path = self.tile_path(level - 1, source_coords);
				let timestamp = match fs::modified_timestamp(&source_path) {
					Ok(timestamp) => timestamp,
					Err(err) => {
						warn!("{}", err);
						return None;
					}
				};
				Some(((dx, dz), source_path, timestamp))
			})
			.collect();

		let Some(input_timestamp) = sources.iter().map(|(_, _, ts)| *ts).max() else {
			self.log(&output_path, Stat::NotFound);
			return Ok(Stat::NotFound);
		};

		if Some(input_timestamp) <= output_timestamp {
			self.log(&output_path, Stat::Skipped);
			return Ok(Stat::Skipped);
		}

		self.log(&output_path, Stat::Regenerate);

		fs::create_with_timestamp(&output_path, version, input_timestamp, |file| {
			self.write_tile(file, &sources)
		})?;

		Ok(Stat::Regenerate)
	}
}
