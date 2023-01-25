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

	let value: fastnbt::Value = minedmap::io::data::from_file(&args.file)?;
	println!("{:#x?}", value);

	Ok(())
}
