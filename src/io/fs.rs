use std::{
	fs::{self, File},
	io::{BufWriter, Write},
	path::{Path, PathBuf},
};

use anyhow::{Context, Ok, Result};

fn tmpfile_name(path: &Path) -> PathBuf {
	let mut file_name = path.file_name().unwrap_or_default().to_os_string();
	file_name.push(".tmp");

	let mut ret = path.to_path_buf();
	ret.set_file_name(file_name);
	ret
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

pub fn create_with_tmpfile<T, F>(path: &Path, f: F) -> Result<T>
where
	F: FnOnce(&mut BufWriter<File>) -> Result<T>,
{
	let tmp_path = tmpfile_name(path);

	let ret = (|| {
		let ret = create(&tmp_path, f)?;
		rename(&tmp_path, path)?;
		Ok(ret)
	})();

	if ret.is_err() {
		let _ = fs::remove_file(&tmp_path);
	}

	ret
}
