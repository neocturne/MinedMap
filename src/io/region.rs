use std::{
	collections::HashMap,
	fs::File,
	io::{prelude::*, SeekFrom},
	path::Path,
};

use anyhow::{bail, Context, Result};
use flate2::read::ZlibDecoder;
use serde::de::DeserializeOwned;

use crate::types::*;

const BLOCKSIZE: usize = 4096;

#[derive(Debug)]
struct ChunkDesc {
	coords: ChunkCoords,
	len: u8,
}

fn parse_header(header: &[u8; BLOCKSIZE]) -> HashMap<u32, ChunkDesc> {
	let mut map = HashMap::new();

	for coords in ChunkArray::<()>::keys() {
		let chunk = &header[(4
			* (usize::from(CHUNKS_PER_REGION) * usize::from(coords.z.0)
				+ usize::from(coords.x.0)))..];

		let offset = u32::from(chunk[0]) << 16 | u32::from(chunk[1]) << 8 | u32::from(chunk[2]);
		if offset == 0 {
			continue;
		}

		let len = chunk[3];

		map.insert(offset, ChunkDesc { coords, len });
	}

	map
}

fn decode_chunk<T>(buf: &[u8]) -> Result<T>
where
	T: DeserializeOwned,
{
	let (len_bytes, buf) = buf.split_at(4);
	let len = u32::from_be_bytes(
		len_bytes
			.try_into()
			.context("Failed to decode chunk size")?,
	) as usize;

	let buf = &buf[..len];
	let (format, buf) = buf.split_at(1);
	if format.get(0) != Some(&2) {
		bail!("Unknown chunk format");
	}

	let mut decoder = ZlibDecoder::new(&buf[..]);
	let mut decode_buffer = vec![];
	decoder
		.read_to_end(&mut decode_buffer)
		.context("Failed to decompress chunk data")?;

	fastnbt::from_bytes(&decode_buffer).context("Failed to decode NBT data")
}

#[derive(Debug)]
pub struct Region<R: Read + Seek> {
	reader: R,
}

impl<R: Read + Seek> Region<R> {
	pub fn foreach_chunk<T, F>(self, mut f: F) -> Result<()>
	where
		R: Read + Seek,
		T: DeserializeOwned,
		F: FnMut(ChunkCoords, T),
	{
		let Region { mut reader } = self;

		let mut chunk_map = {
			let mut header = [0u8; BLOCKSIZE];
			reader
				.read_exact(&mut header)
				.context("Failed to read region header")?;

			parse_header(&header)
		};

		let mut index = 1;
		let mut seen = ChunkArray::<bool>::default();

		while !chunk_map.is_empty() {
			let Some(ChunkDesc { coords, len }) = chunk_map.remove(&index) else {
				reader.seek(SeekFrom::Current(BLOCKSIZE as i64)).context("Failed to seek chunk data")?;
				index += 1;
				continue;
			};

			if seen[coords] {
				bail!("Duplicate chunk");
			}
			seen[coords] = true;

			let mut buffer = vec![0; (len as usize) * BLOCKSIZE];
			reader
				.read_exact(&mut buffer[..])
				.context("Failed to read chunk data")?;

			f(coords, decode_chunk(&buffer[..])?);

			index += len as u32;
		}

		Ok(())
	}
}

pub fn from_reader<R>(reader: R) -> Region<R>
where
	R: Read + Seek,
{
	Region { reader }
}

pub fn from_file<P>(path: P) -> Result<Region<File>>
where
	P: AsRef<Path>,
{
	let file = File::open(path).context("Failed to open file")?;
	Ok(from_reader(file))
}
