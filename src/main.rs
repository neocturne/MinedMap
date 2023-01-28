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

	minedmap::io::region::from_file(args.file.as_path())?.foreach_chunk(
		|coords, value: minedmap::world::de::Chunk| {
			println!("Chunk {:?}: {:#?}", coords, value);
		},
	)
}
