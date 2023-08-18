//! Helpers and common functions for filesystem access

use std::{
	fs::{self, File},
	io::{BufReader, BufWriter, Read, Write},
	path::{Path, PathBuf},
	time::SystemTime,
};

use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};

/// A file metadata version number
///
/// Deserialized metadata with non-current version number are considered invalid
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileMetaVersion(pub u32);

/// Metadata stored with generated files to track required incremental updates
#[derive(Debug, Serialize, Deserialize)]
struct FileMeta {
	/// Version of data described by the FileMeta
	version: FileMetaVersion,
	/// Timestamp stored with generated data
	///
	/// This timestamp is always the time of last modification of the inputs
	/// that were used to generate the file described by the FileMeta.
	timestamp: SystemTime,
}

/// Helper for creating suffixed file paths
fn suffix_name(path: &Path, suffix: &str) -> PathBuf {
	let mut file_name = path.file_name().unwrap_or_default().to_os_string();
	file_name.push(suffix);

	let mut ret = path.to_path_buf();
	ret.set_file_name(file_name);
	ret
}

/// Derives the filename for temporary storage of data during generation
fn tmpfile_name(path: &Path) -> PathBuf {
	suffix_name(path, ".tmp")
}

/// Derives the filename for associated metadata for generated files
fn metafile_name(path: &Path) -> PathBuf {
	suffix_name(path, ".meta")
}

/// Creates a directory including all its parents
///
/// Wrapper around [fs::create_dir_all] that adds a more descriptive error message
pub fn create_dir_all(path: &Path) -> Result<()> {
	fs::create_dir_all(path)
		.with_context(|| format!("Failed to create directory {}", path.display(),))
}

/// Renames a file or directory
///
/// Wrapper around [fs::rename] that adds a more descriptive error message
pub fn rename(from: &Path, to: &Path) -> Result<()> {
	fs::rename(from, to)
		.with_context(|| format!("Failed to rename {} to {}", from.display(), to.display()))
}

/// Creates a new file
///
/// The contents of the file are defined by the passed function.
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

/// Checks whether the contents of two files are equal
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

/// Creates a new file, temporarily storing its contents in a temporary file
///
/// Storing the data in a temporary file prevents leaving half-written files
/// when the function is interrupted. In addition, the old and new contents of
/// the file are compared if a file with the same name already exists, and the
/// file timestamp is only updated if the contents have changed.
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

/// Returns the time of last modification for a given file path
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

/// Reads the stored timestamp from file metadata for a file previously written
/// using [create_with_timestamp]
pub fn read_timestamp(path: &Path, version: FileMetaVersion) -> Option<SystemTime> {
	let meta_path = metafile_name(path);
	let mut file = BufReader::new(fs::File::open(meta_path).ok()?);

	let meta: FileMeta = serde_json::from_reader(&mut file).ok()?;
	if meta.version != version {
		return None;
	}

	Some(meta.timestamp)
}

/// Creates a new file, temporarily storing its contents in a temporary file
/// like [create_with_tmpfile], and storing a timestamp in a metadata file
/// if successful
///
/// The timestamp can be retrieved later using [read_timestamp].
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
