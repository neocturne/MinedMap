use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use minedmap::{resource, world};

#[derive(Debug, Parser)]
struct Args {
	/// Filename to dump
	file: PathBuf,
}

fn main() -> Result<()> {
	let args = Args::parse();

	let block_types = resource::block_types();

	minedmap::io::region::from_file(args.file.as_path())?.foreach_chunk(
		|coords, data: world::de::Chunk| {
			let chunk = match world::chunk::Chunk::new(&data) {
				Ok(chunk) => chunk,
				Err(err) => {
					eprintln!("Chunk {:?}: {}", coords, err);
					return;
				}
			};

			match world::layer::top_layer(&chunk, &block_types) {
				Ok(_) => {}
				Err(err) => println!("{:?}", err),
			}
		},
	)
}
