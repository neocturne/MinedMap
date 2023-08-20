//! Core functions of the MinedMap CLI

mod common;
mod metadata_writer;
mod region_group;
mod region_processor;
mod tile_mipmapper;
mod tile_renderer;

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

use common::Config;
use metadata_writer::MetadataWriter;
use region_processor::RegionProcessor;
use tile_mipmapper::TileMipmapper;
use tile_renderer::TileRenderer;

/// Generate map tiles from Minecraft save data
#[derive(Debug, Parser)]
pub struct Args {
	/// Number of parallel threads to use for processing
	///
	/// If not given, only a single thread is used. Pass 0 to
	/// use one thread per logical CPU core.
	#[arg(short, long)]
	pub jobs: Option<usize>,
	/// Minecraft save directory
	pub input_dir: PathBuf,
	/// MinedMap data directory
	pub output_dir: PathBuf,
}

/// Configures the Rayon thread pool for parallel processing
fn setup_threads(num_threads: usize) -> Result<()> {
	rayon::ThreadPoolBuilder::new()
		.num_threads(num_threads)
		.build_global()
		.context("Failed to configure thread pool")
}

/// MinedMap CLI main function
pub fn cli() -> Result<()> {
	let args = Args::parse();
	let config = Config::new(&args);

	setup_threads(config.num_threads)?;

	let rt = tokio::runtime::Builder::new_current_thread()
		.build()
		.unwrap();

	let regions = RegionProcessor::new(&config).run()?;
	TileRenderer::new(&config, &rt, &regions).run()?;
	let tiles = TileMipmapper::new(&config, &regions).run()?;
	MetadataWriter::new(&config, &tiles).run()?;

	Ok(())
}
