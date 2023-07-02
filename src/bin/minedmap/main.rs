mod common;
mod region_processor;
mod tile_mipmapper;
mod tile_renderer;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use common::Config;
use region_processor::RegionProcessor;
use tile_mipmapper::TileMipmapper;
use tile_renderer::TileRenderer;

#[derive(Debug, Parser)]
pub struct Args {
	/// Minecraft save directory
	pub input_dir: PathBuf,
	/// MinedMap data directory
	pub output_dir: PathBuf,
}

fn main() -> Result<()> {
	let args = Args::parse();
	let config = Config::new(args);

	let regions = RegionProcessor::new(&config).run()?;
	TileRenderer::new(&config).run(regions.iter().copied())?;
	TileMipmapper::new(&config).run(regions)?;

	Ok(())
}
