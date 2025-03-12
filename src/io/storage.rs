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
use bincode::{Decode, Encode};

use super::fs;

/// Bincode configuration
const BINCODE_CONFIG: bincode::config::Configuration = bincode::config::standard();

/// Serializes data and writes it to a writer
pub fn write<W: Write, T: Encode>(writer: &mut W, value: &T) -> Result<()> {
	let data = bincode::encode_to_vec(value, BINCODE_CONFIG)?;
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
pub fn write_file<T: Encode>(
	path: &Path,
	value: &T,
	version: fs::FileMetaVersion,
	timestamp: SystemTime,
) -> Result<()> {
	fs::create_with_timestamp(path, version, timestamp, |file| write(file, value))
}

/// Reads data from a reader and deserializes it
pub fn read<R, T>(reader: &mut R) -> Result<T>
where
	R: Read,
	T: Decode<()>,
{
	let mut len_buf = [0u8; 4];
	reader.read_exact(&mut len_buf)?;
	let len = usize::try_from(u32::from_be_bytes(len_buf))?;

	let mut compressed = vec![];
	reader.read_to_end(&mut compressed)?;
	let data = zstd::bulk::decompress(&compressed, len)?;
	drop(compressed);

	Ok(bincode::decode_from_slice(&data, BINCODE_CONFIG)?.0)
}

/// Reads data from a file and deserializes it
pub fn read_file<T>(path: &Path) -> Result<T>
where
	T: Decode<()>,
{
	(|| -> Result<T> {
		let mut file = File::open(path)?;
		read(&mut file)
	})()
	.with_context(|| format!("Failed to read file {}", path.display()))
}
