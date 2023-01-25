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

	minedmap::io::region::from_file(&args.file)?.foreach_chunk(|x, z, value: fastnbt::Value| {
		println!("Chunk({}, {}): {:#x?}", x.0, z.0, value);
	})
}
