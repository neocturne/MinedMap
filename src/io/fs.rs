use std::{
	fs::{self, File},
	io::{BufReader, BufWriter, Read, Write},
	path::{Path, PathBuf},
	time::SystemTime,
};

use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileMetaVersion(pub u32);

#[derive(Debug, Serialize, Deserialize)]
struct FileMeta {
	version: FileMetaVersion,
	timestamp: SystemTime,
}

fn suffix_name(path: &Path, suffix: &str) -> PathBuf {
	let mut file_name = path.file_name().unwrap_or_default().to_os_string();
	file_name.push(suffix);

	let mut ret = path.to_path_buf();
	ret.set_file_name(file_name);
	ret
}

fn tmpfile_name(path: &Path) -> PathBuf {
	suffix_name(path, ".tmp")
}

fn metafile_name(path: &Path) -> PathBuf {
	suffix_name(path, ".meta")
}

pub fn create_dir_all(path: &Path) -> Result<()> {
	fs::create_dir_all(path)
		.with_context(|| format!("Failed to create directory {}", path.display(),))
}

pub fn rename(from: &Path, to: &Path) -> Result<()> {
	fs::rename(from, to)
		.with_context(|| format!("Failed to rename {} to {}", from.display(), to.display()))
}

pub fn create<T, F>(path: &Path, f: F) -> Result<T>
where
	F: FnOnce(&mut BufWriter<File>) -> Result<T>,
{
	(|| {
		let file = File::create(path)?;
		let mut writer = BufWriter::new(file);

		let ret = f(&mut writer)?;
		writer.flush()?;

		Ok(ret)
	})()
	.with_context(|| format!("Failed to write file {}", path.display()))
}

pub fn equal(path1: &Path, path2: &Path) -> Result<bool> {
	let mut file1 = BufReader::new(
		fs::File::open(path1)
			.with_context(|| format!("Failed to open file {}", path1.display()))?,
	)
	.bytes();
	let mut file2 = BufReader::new(
		fs::File::open(path2)
			.with_context(|| format!("Failed to open file {}", path2.display()))?,
	)
	.bytes();

	Ok(loop {
		match (file1.next().transpose()?, file2.next().transpose()?) {
			(Some(b1), Some(b2)) if b1 == b2 => continue,
			(None, None) => break true,
			_ => break false,
		};
	})
}

pub fn create_with_tmpfile<T, F>(path: &Path, f: F) -> Result<T>
where
	F: FnOnce(&mut BufWriter<File>) -> Result<T>,
{
	let tmp_path = tmpfile_name(path);
	let mut cleanup = true;

	let ret = (|| {
		let ret = create(&tmp_path, f)?;
		if !matches!(equal(path, &tmp_path), Result::Ok(true)) {
			rename(&tmp_path, path)?;
			cleanup = false;
		}
		Ok(ret)
	})();

	if cleanup {
		let _ = fs::remove_file(&tmp_path);
	}

	ret
}

pub fn modified_timestamp(path: &Path) -> Result<SystemTime> {
	fs::metadata(path)
		.and_then(|meta| meta.modified())
		.with_context(|| {
			format!(
				"Failed to get modified timestamp of file {}",
				path.display()
			)
		})
}

pub fn read_timestamp(path: &Path, version: FileMetaVersion) -> Option<SystemTime> {
	let meta_path = metafile_name(path);
	let mut file = BufReader::new(fs::File::open(meta_path).ok()?);

	let meta: FileMeta = serde_json::from_reader(&mut file).ok()?;
	if meta.version != version {
		return None;
	}

	Some(meta.timestamp)
}

pub fn create_with_timestamp<T, F>(
	path: &Path,
	version: FileMetaVersion,
	timestamp: SystemTime,
	f: F,
) -> Result<T>
where
	F: FnOnce(&mut BufWriter<File>) -> Result<T>,
{
	let ret = create_with_tmpfile(path, f)?;

	let meta_path = metafile_name(path);
	create(&meta_path, |file| {
		serde_json::to_writer(file, &FileMeta { version, timestamp })?;
		Ok(())
	})?;

	Ok(ret)
}
