//! Dumps data from a NBT data file in a human-readable format

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

/// Dump a Minecraft NBT data file in a human-readable format
#[derive(Debug, Parser)]
struct Args {
	/// Filename to dump
	file: PathBuf,
}

fn main() -> Result<()> {
	let args = Args::parse();

	let value: fastnbt::Value = minedmap_nbt::data::from_file(args.file.as_path())?;
	println!("{:#x?}", value);

	Ok(())
}
