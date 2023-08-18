use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
	/// Filename to dump
	file: PathBuf,
}

fn main() -> Result<()> {
	let args = Args::parse();

	let value: fastnbt::Value = minedmap_core::io::data::from_file(args.file.as_path())?;
	println!("{:#x?}", value);

	Ok(())
}
