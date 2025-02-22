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

use std::{
	path::PathBuf,
	sync::mpsc::{self, Receiver},
	thread,
	time::Duration,
};

use anyhow::{Context, Result};
use clap::Parser;
use git_version::git_version;

use common::{Config, ImageFormat};
use metadata_writer::MetadataWriter;
use notify::{RecommendedWatcher, RecursiveMode, Watcher as _};
use rayon::ThreadPool;
use region_processor::RegionProcessor;
use tile_mipmapper::TileMipmapper;
use tile_renderer::TileRenderer;
use tokio::runtime::Runtime;
use tracing::{info, warn};

use self::entity_collector::EntityCollector;

/// Returns the MinedMap version number
fn version() -> &'static str {
	option_env!("MINEDMAP_VERSION").unwrap_or(
		git_version!(
			args = ["--abbrev=7", "--match=v*", "--dirty=-modified"],
			cargo_prefix = "v",
		)
		.strip_prefix("v")
		.unwrap(),
	)
}

/// Command line arguments for minedmap CLI
#[derive(Debug, Parser)]
#[command(
	about,
	version = version(),
	max_term_width = 100,
)]
pub struct Args {
	/// Number of parallel threads to use for processing
	///
	/// If not given, only a single thread is used. Pass 0 to
	/// use one thread per logical CPU core.
	#[arg(short, long)]
	pub jobs: Option<usize>,
	/// Number of parallel threads to use for initial processing
	///
	/// Passing this option only makes sense with --watch. The first run after
	/// starting MinedMap will use as many parallel jobs as configured using
	/// --job-initial, while subsequent regenerations of tiles will use the
	/// the number configured using --jobs.
	///
	/// If not given, the value from the --jobs option is used.
	#[arg(long)]
	pub jobs_initial: Option<usize>,
	/// Enable verbose messages
	#[arg(short, long)]
	pub verbose: bool,
	/// Watch for file changes and regenerate tiles automatically instead of
	/// exiting after generation
	#[arg(long)]
	pub watch: bool,
	/// Minimum delay between map generation cycles in watch mode
	#[arg(long, value_parser = humantime::parse_duration, default_value = "30s")]
	pub watch_delay: Duration,
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

/// Configures a Rayon thread pool for parallel processing
fn setup_threads(num_threads: usize) -> Result<ThreadPool> {
	rayon::ThreadPoolBuilder::new()
		.num_threads(num_threads)
		.build()
		.context("Failed to configure thread pool")
}

/// Runs all MinedMap generation steps, updating all tiles as needed
fn generate(config: &Config, rt: &Runtime) -> Result<()> {
	let regions = RegionProcessor::new(config).run()?;
	TileRenderer::new(config, rt, &regions).run()?;
	let tiles = TileMipmapper::new(config, &regions).run()?;
	EntityCollector::new(config, &regions).run()?;
	MetadataWriter::new(config, &tiles).run()
}

/// Creates a file watcher for the
fn create_watcher(args: &Args) -> Result<(RecommendedWatcher, Receiver<()>)> {
	let (tx, rx) = mpsc::sync_channel::<()>(1);
	let mut watcher = notify::recommended_watcher(move |res| {
		// Ignore errors - we already have a watch trigger queued if try_send() fails
		let event: notify::Event = match res {
			Ok(event) => event,
			Err(err) => {
				warn!("Watch error: {err}");
				return;
			}
		};
		let notify::EventKind::Modify(modify_kind) = event.kind else {
			return;
		};
		if !matches!(
			modify_kind,
			notify::event::ModifyKind::Data(_)
				| notify::event::ModifyKind::Name(notify::event::RenameMode::To)
		) {
			return;
		}
		if !event
			.paths
			.iter()
			.any(|path| path.ends_with("level.dat") || path.extension() == Some("mcu".as_ref()))
		{
			return;
		}
		let _ = tx.try_send(());
	})?;
	watcher.watch(&args.input_dir, RecursiveMode::Recursive)?;
	Ok((watcher, rx))
}

/// Watches the data directory for changes, returning when a change has happened
fn wait_watcher(args: &Args, watch_channel: &Receiver<()>) -> Result<()> {
	info!("Watching for changes...");
	let () = watch_channel
		.recv()
		.context("Failed to read watch event channel")?;
	info!("Change detected.");

	thread::sleep(args.watch_delay);

	let _ = watch_channel.try_recv();

	Ok(())
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

	let mut pool = setup_threads(config.num_threads_initial)?;

	let rt = tokio::runtime::Builder::new_current_thread()
		.build()
		.unwrap();

	let watch = args.watch.then(|| create_watcher(&args)).transpose()?;

	pool.install(|| generate(&config, &rt))?;

	let Some((_watcher, watch_channel)) = watch else {
		// watch mode disabled
		return Ok(());
	};

	if config.num_threads != config.num_threads_initial {
		pool = setup_threads(config.num_threads)?;
	}
	pool.install(move || loop {
		wait_watcher(&args, &watch_channel)?;
		generate(&config, &rt)?;
	})
}
