//! Dumps data from a region data file in a human-readable format

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

/// Command line arguments for regiondump
#[derive(Debug, Parser)]
struct Args {
	/// Filename to dump
	file: PathBuf,
}

fn main() -> Result<()> {
	let args = Args::parse();

	minedmap_nbt::region::from_file(args.file.as_path())?.foreach_chunk(
		|coords, value: fastnbt::Value| {
			println!("Chunk {:?}: {:#x?}", coords, value);
			Ok(())
		},
	)
}
