use std::{
	fs::File,
	io::{BufWriter, Write},
	path::Path,
};

use anyhow::{Context, Result};
use serde::Serialize;

pub fn write<T: Serialize>(path: &Path, value: &T) -> Result<()> {
	(|| -> Result<()> {
		let file = File::create(path)?;
		let writer = BufWriter::new(file);
		let mut compressor = zstd::Encoder::new(writer, 1)?;
		bincode::serialize_into(&mut compressor, value)?;
		let writer = compressor.finish()?;
		let mut file = writer.into_inner()?;
		file.flush()?;
		Ok(())
	})()
	.with_context(|| format!("Failed to write file {}", path.display()))
}
