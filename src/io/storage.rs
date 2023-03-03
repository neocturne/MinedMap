use std::{
	fs::File,
	io::{Read, Write},
	path::Path,
};

use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};

pub fn write<T: Serialize>(path: &Path, value: &T) -> Result<()> {
	(|| -> Result<()> {
		let data = bincode::serialize(value)?;
		let len = u32::try_from(data.len())?;
		let compressed = zstd::bulk::compress(&data, 1)?;
		drop(data);

		let mut file = File::create(path)?;
		file.write_all(&len.to_be_bytes())?;
		file.write_all(&compressed)?;
		file.flush()?;

		Ok(())
	})()
	.with_context(|| format!("Failed to write file {}", path.display()))
}

pub fn read<T: DeserializeOwned>(path: &Path) -> Result<T> {
	(|| -> Result<T> {
		let mut file = File::open(path)?;

		let mut len_buf = [0u8; 4];
		file.read_exact(&mut len_buf)?;
		let len = usize::try_from(u32::from_be_bytes(len_buf))?;

		let mut compressed = vec![];
		file.read_to_end(&mut compressed)?;
		let data = zstd::bulk::decompress(&compressed, len)?;
		drop(compressed);

		Ok(bincode::deserialize(&data)?)
	})()
	.with_context(|| format!("Failed to read file {}", path.display()))
}
