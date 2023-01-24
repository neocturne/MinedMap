use std::{fs::File, io::prelude::*, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use flate2::read::GzDecoder;

#[derive(Debug, Parser)]
struct Args {
	/// Filename to dump
	file: PathBuf,
}

fn main() -> Result<()> {
	let args = Args::parse();

	let file = File::open(&args.file).context("Failed to open file")?;

	let mut decoder = GzDecoder::new(file);
	let mut buf = vec![];
	decoder
		.read_to_end(&mut buf)
		.context("Failed to read file")?;

	let nbt: fastnbt::Value =
		fastnbt::from_bytes(buf.as_slice()).context("Failed to decode NBT data")?;

	println!("{:#x?}", nbt);

	Ok(())
}
