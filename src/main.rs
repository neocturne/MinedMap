use std::{
	fs,
	path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use clap::Parser;

use minedmap::{io::storage, resource, types::*, world};

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

	fn processed_path(&self, coords: RegionCoords, temp: bool) -> PathBuf {
		let filename = format!(
			"r.{}.{}.bin{}",
			coords.0,
			coords.1,
			if temp { ".tmp" } else { "" },
		);
		[&self.processed_dir, Path::new(&filename)].iter().collect()
	}

	/// Processes a single chunk
	fn process_chunk(&self, data: world::de::Chunk) -> Result<Box<world::layer::BlockInfoArray>> {
		let chunk = world::chunk::Chunk::new(&data)?;
		world::layer::top_layer(&chunk, &self.block_types)
	}

	fn save_region(
		&self,
		region: RegionCoords,
		processed_data: &ChunkArray<Option<Box<world::layer::BlockInfoArray>>>,
	) -> Result<()> {
		let tmp_path = self.processed_path(region, true);
		storage::write(&tmp_path, processed_data)?;

		let output_path = self.processed_path(region, false);
		fs::rename(&tmp_path, &output_path).with_context(|| {
			format!(
				"Failed to rename {} to {}",
				tmp_path.display(),
				output_path.display(),
			)
		})?;

		Ok(())
	}

	/// Processes a single region file
	fn process_region(&self, path: &Path, coords: RegionCoords) -> Result<()> {
		println!("Processing region r.{}.{}.mca", coords.0, coords.1);

		let mut processed_data: ChunkArray<Option<Box<world::layer::BlockInfoArray>>> =
			Default::default();

		minedmap::io::region::from_file(path)?.foreach_chunk(
			|chunk_coords, data: world::de::Chunk| {
				let processed_chunk = self
					.process_chunk(data)
					.with_context(|| format!("Failed to process chunk {:?}", chunk_coords))?;
				processed_data[chunk_coords] = Some(processed_chunk);
				Ok(())
			},
		)?;

		self.save_region(coords, &processed_data)?;

		Ok(())
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
