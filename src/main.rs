use std::{
	fs,
	path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use clap::Parser;

use minedmap::{resource, world};

#[derive(Debug, Parser)]
struct Args {
	/// Minecraft save directory
	input_dir: PathBuf,
	/// MinedMap data directory
	output_dir: PathBuf,
}

type RegionCoords = (i32, i32);

/// Type with methods for processing the regions of a Minecraft save directory
struct RegionProcessor {
	block_types: resource::BlockTypeMap,
	processed_dir: PathBuf,
}

impl RegionProcessor {
	fn new(output_dir: &Path) -> Self {
		RegionProcessor {
			block_types: resource::block_types(),
			processed_dir: [output_dir, Path::new("processed")].iter().collect(),
		}
	}

	/// Parses a filename in the format r.X.Z.mca into the contained X and Z values
	fn parse_region_filename(path: &Path) -> Option<RegionCoords> {
		let file_name = path.file_name()?.to_str()?;
		let parts: Vec<_> = file_name.split('.').collect();
		let &["r", x, z, "mca"] = parts.as_slice() else {
			return None;
		};

		Some((x.parse().ok()?, z.parse().ok()?))
	}

	/// Processes a single region file
	fn process_region(&self, path: &Path, coords: RegionCoords) -> Result<()> {
		println!("Processing region r.{}.{}.mca", coords.0, coords.1);

		minedmap::io::region::from_file(path)?.foreach_chunk(
			|chunk_coords, data: world::de::Chunk| {
				(|| -> Result<()> {
					let chunk = world::chunk::Chunk::new(&data)?;

					let _top_layer = world::layer::top_layer(&chunk, &self.block_types)?;

					Ok(())
				})()
				.with_context(|| format!("Failed to process chunk {:?}", chunk_coords))
			},
		)
	}

	/// Iterates over all region files of a Minecraft save directory
	fn process_region_dir(&self, regiondir: &Path) -> Result<()> {
		let read_dir = regiondir
			.read_dir()
			.with_context(|| format!("Failed to read directory {}", regiondir.display()))?;

		fs::create_dir_all(&self.processed_dir).with_context(|| {
			format!(
				"Failed to create directory {}",
				self.processed_dir.display(),
			)
		})?;

		for entry in read_dir.filter_map(|entry| entry.ok()).filter(|entry| {
			// We are only interested in regular files
			entry
				.file_type()
				.map(|file_type| file_type.is_file())
				.unwrap_or_default()
		}) {
			let path = entry.path();
			let Some(coords) = Self::parse_region_filename(&path) else {
				continue;
			};

			if let Err(err) = self.process_region(&path, coords) {
				eprintln!(
					"Failed to process region r.{}.{}.mca: {}",
					coords.0, coords.1, err,
				);
			}
		}

		Ok(())
	}
}

fn main() -> Result<()> {
	let args = Args::parse();

	let regiondir: PathBuf = [&args.input_dir, Path::new("region")].iter().collect();

	let region_processor = RegionProcessor::new(&args.output_dir);
	region_processor.process_region_dir(&regiondir)?;

	Ok(())
}
