//! Functions for reading and deserializing region data

use std::{
	fs::File,
	io::{prelude::*, SeekFrom},
	path::Path,
};

use anyhow::{bail, Context, Result};
use flate2::read::ZlibDecoder;
use serde::de::DeserializeOwned;

use crate::types::*;

/// Data block size of region data files
///
/// After one header block, the region file consists of one or more consecutive blocks
/// of data for each populated chunk.
const BLOCKSIZE: usize = 4096;

/// Chunk descriptor extracted from region file header
#[derive(Debug)]
struct ChunkDesc {
	/// Offset of data block where the chunk starts
	offset: u32,
	/// Number of data block used by the chunk
	len: u8,
	/// Coodinates of chunk described by this descriptor
	coords: ChunkCoords,
}

/// Parses the header of a region data file
fn parse_header(header: &ChunkArray<u32>) -> Vec<ChunkDesc> {
	let mut chunks: Vec<_> = header
		.iter()
		.filter_map(|(coords, &chunk)| {
			let offset_len = u32::from_be(chunk);

			let offset = offset_len >> 8;
			let len = offset_len as u8;

			if offset == 0 || len == 0 {
				return None;
			}

			Some(ChunkDesc {
				offset,
				len,
				coords,
			})
		})
		.collect();

	chunks.sort_by_key(|chunk| chunk.offset);

	chunks
}

/// Decompresses chunk data and deserializes to a given data structure
fn decode_chunk<T>(buf: &[u8]) -> Result<T>
where
	T: DeserializeOwned,
{
	let (format, buf) = buf.split_at(1);
	if format[0] != 2 {
		bail!("Unknown chunk format");
	}

	let mut decoder = ZlibDecoder::new(buf);
	let mut decode_buffer = vec![];
	decoder
		.read_to_end(&mut decode_buffer)
		.context("Failed to decompress chunk data")?;

	fastnbt::from_bytes(&decode_buffer).context("Failed to decode NBT data")
}

/// Wraps a reader used to read a region data file
#[derive(Debug)]
pub struct Region<R: Read + Seek> {
	/// The wrapper reader
	reader: R,
}

impl<R: Read + Seek> Region<R> {
	/// Iterates over the chunks of the region data
	///
	/// The order of iteration is based on the order the chunks appear in the
	/// data file.
	pub fn foreach_chunk<T, F>(self, mut f: F) -> Result<()>
	where
		R: Read + Seek,
		T: DeserializeOwned,
		F: FnMut(ChunkCoords, T) -> Result<()>,
	{
		let Region { mut reader } = self;

		let chunks = {
			let mut header = ChunkArray::<u32>::default();
			reader
				.read_exact(bytemuck::cast_mut::<_, [u8; BLOCKSIZE]>(&mut header.0))
				.context("Failed to read region header")?;

			parse_header(&header)
		};

		let mut seen = ChunkArray::<bool>::default();

		for ChunkDesc {
			offset,
			len,
			coords,
		} in chunks
		{
			if seen[coords] {
				bail!("Duplicate chunk {:?}", coords);
			}
			seen[coords] = true;

			reader
				.seek(SeekFrom::Start(offset as u64 * BLOCKSIZE as u64))
				.context("Failed to seek chunk data")?;

			let mut len_buf = [0u8; 4];
			reader
				.read_exact(&mut len_buf)
				.with_context(|| format!("Failed to read length for chunk {:?}", coords))?;
			let byte_len = u32::from_be_bytes(len_buf) as usize;
			if byte_len < 1 || byte_len > (len as usize) * BLOCKSIZE - 4 {
				bail!("Invalid length for chunk {:?}", coords);
			}

			let mut buffer = vec![0; byte_len];
			reader
				.read_exact(&mut buffer)
				.with_context(|| format!("Failed to read data for chunk {:?}", coords))?;
			let chunk = decode_chunk(&buffer)
				.with_context(|| format!("Failed to decode data for chunk {:?}", coords))?;

			f(coords, chunk)?;
		}

		Ok(())
	}
}

/// Creates a new [Region] from a reader
pub fn from_reader<R>(reader: R) -> Region<R>
where
	R: Read + Seek,
{
	Region { reader }
}

/// Creates a new [Region] for a file
pub fn from_file<P>(path: P) -> Result<Region<File>>
where
	P: AsRef<Path>,
{
	let file = File::open(path).context("Failed to open file")?;
	Ok(from_reader(file))
}
