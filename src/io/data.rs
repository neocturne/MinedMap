use std::{fs::File, io::prelude::*, path::Path};

use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use serde::de::DeserializeOwned;

pub fn from_reader<R, T>(reader: R) -> Result<T>
where
	R: Read,
	T: DeserializeOwned,
{
	let mut decoder = GzDecoder::new(reader);
	let mut buf = vec![];
	decoder
		.read_to_end(&mut buf)
		.context("Failed to read file")?;

	fastnbt::from_bytes(&buf).context("Failed to decode NBT data")
}

pub fn from_file<P, T>(path: P) -> Result<T>
where
	P: AsRef<Path>,
	T: DeserializeOwned,
{
	let file = File::open(path).context("Failed to open file")?;
	from_reader(file)
}
