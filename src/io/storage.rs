//! Functions for serializing and deserializing MinedMap data structures efficiently
//!
//! Data is serialized using Bincode and compressed using zstd.

use std::{
	fs::File,
	io::{Read, Write},
	path::Path,
	time::SystemTime,
};

use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};

use super::fs;

/// Serializes data and writes it to a writer
pub fn write<W: Write, T: Serialize>(writer: &mut W, value: &T) -> Result<()> {
	let data = bincode::serialize(value)?;
	let len = u32::try_from(data.len())?;
	let compressed = zstd::bulk::compress(&data, 1)?;
	drop(data);

	writer.write_all(&len.to_be_bytes())?;
	writer.write_all(&compressed)?;

	Ok(())
}

/// Serializes data and stores it in a file
///
/// A timestamp is stored in an assiciated metadata file.
pub fn write_file<T: Serialize>(
	path: &Path,
	value: &T,
	version: fs::FileMetaVersion,
	timestamp: SystemTime,
) -> Result<()> {
	fs::create_with_timestamp(path, version, timestamp, |file| write(file, value))
}

/// Reads data from a reader and deserializes it
pub fn read<R: Read, T: DeserializeOwned>(reader: &mut R) -> Result<T> {
	let mut len_buf = [0u8; 4];
	reader.read_exact(&mut len_buf)?;
	let len = usize::try_from(u32::from_be_bytes(len_buf))?;

	let mut compressed = vec![];
	reader.read_to_end(&mut compressed)?;
	let data = zstd::bulk::decompress(&compressed, len)?;
	drop(compressed);

	Ok(bincode::deserialize(&data)?)
}

/// Reads data from a file and deserializes it
pub fn read_file<T: DeserializeOwned>(path: &Path) -> Result<T> {
	(|| -> Result<T> {
		let mut file = File::open(path)?;
		read(&mut file)
	})()
	.with_context(|| format!("Failed to read file {}", path.display()))
}
