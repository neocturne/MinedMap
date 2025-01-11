//! Core functions of the MinedMap CLI

mod common;
mod entity_collector;
mod metadata_writer;
mod region_group;
mod region_processor;
mod tile_collector;
mod tile_merger;
mod tile_mipmapper;
mod tile_renderer;

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use git_version::git_version;

use common::{Config, ImageFormat};
use metadata_writer::MetadataWriter;
use region_processor::RegionProcessor;
use tile_mipmapper::TileMipmapper;
use tile_renderer::TileRenderer;

use self::entity_collector::EntityCollector;

/// MinedMap version number
const VERSION: &str = git_version!(
	args = ["--abbrev=7", "--match=v*", "--dirty=-modified"],
	cargo_prefix = "v",
);

/// Command line arguments for minedmap CLI
#[derive(Debug, Parser)]
#[command(
	about,
	version = VERSION.strip_prefix("v").unwrap(),
	max_term_width = 100,
)]
pub struct Args {
	/// Number of parallel threads to use for processing
	///
	/// If not given, only a single thread is used. Pass 0 to
	/// use one thread per logical CPU core.
	#[arg(short, long)]
	pub jobs: Option<usize>,
	/// Enable verbose messages
	#[arg(short, long)]
	pub verbose: bool,
	/// Format of generated map tiles
	#[arg(long, value_enum, default_value_t)]
	pub image_format: ImageFormat,
	/// Prefix for text of signs to show on the map
	#[arg(long)]
	pub sign_prefix: Vec<String>,
	/// Regular expression for text of signs to show on the map
	///
	/// --sign-prefix and --sign-filter allow to filter for signs to display;
	/// by default, none are visible. The options may be passed multiple times,
	/// and a sign will be visible if it matches any pattern.
	///
	/// To make all signs visible, pass an empty string to either option.
	#[arg(long)]
	pub sign_filter: Vec<String>,
	/// Regular expression replacement pattern for sign texts
	///
	/// Accepts patterns of the form 's/regexp/replacement/'. Transforms
	/// are applied to each line of sign texts separately.
	#[arg(long)]
	pub sign_transform: Vec<String>,
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
	let config = Config::new(&args)?;

	tracing_subscriber::fmt()
		.with_max_level(if args.verbose {
			tracing::Level::DEBUG
		} else {
			tracing::Level::INFO
		})
		.with_target(false)
		.init();

	setup_threads(config.num_threads)?;

	let rt = tokio::runtime::Builder::new_current_thread()
		.build()
		.unwrap();

	let regions = RegionProcessor::new(&config).run()?;
	TileRenderer::new(&config, &rt, &regions).run()?;
	let tiles = TileMipmapper::new(&config, &regions).run()?;
	EntityCollector::new(&config, &regions).run()?;
	MetadataWriter::new(&config, &tiles).run()?;

	Ok(())
}
