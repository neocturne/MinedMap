use std::path::{Path, PathBuf};

use minedmap::{types::*, world};

pub type RegionCoords = (i32, i32);
pub type ProcessedRegion = ChunkArray<
	Option<(
		Box<world::layer::BlockInfoArray>,
		Box<world::layer::BiomeArray>,
	)>,
>;

pub struct Config {
	pub region_dir: PathBuf,
	pub processed_dir: PathBuf,
	pub light_dir: PathBuf,
	pub map_dir: PathBuf,
}

impl Config {
	pub fn new(args: super::Args) -> Self {
		let region_dir = [&args.input_dir, Path::new("region")].iter().collect();
		let processed_dir = [&args.output_dir, Path::new("processed")].iter().collect();
		let light_dir = [&args.output_dir, Path::new("light/0")].iter().collect();
		let map_dir = [&args.output_dir, Path::new("map/0")].iter().collect();

		Config {
			region_dir,
			processed_dir,
			light_dir,
			map_dir,
		}
	}

	pub fn processed_path(&self, coords: RegionCoords, temp: bool) -> PathBuf {
		let filename = format!(
			"r.{}.{}.bin{}",
			coords.0,
			coords.1,
			if temp { ".tmp" } else { "" },
		);
		[&self.processed_dir, Path::new(&filename)].iter().collect()
	}

	pub fn light_path(&self, coords: RegionCoords, temp: bool) -> PathBuf {
		let filename = format!(
			"r.{}.{}.png{}",
			coords.0,
			coords.1,
			if temp { ".tmp" } else { "" },
		);
		[&self.light_dir, Path::new(&filename)].iter().collect()
	}

	pub fn map_path(&self, coords: RegionCoords, temp: bool) -> PathBuf {
		let filename = format!(
			"r.{}.{}.png{}",
			coords.0,
			coords.1,
			if temp { ".tmp" } else { "" },
		);
		[&self.map_dir, Path::new(&filename)].iter().collect()
	}
}

pub fn overlay_chunk<I, J>(image: &mut I, chunk: &J, coords: ChunkCoords)
where
	I: image::GenericImage,
	J: image::GenericImageView<Pixel = I::Pixel>,
{
	image::imageops::overlay(
		image,
		chunk,
		coords.x.0 as i64 * BLOCKS_PER_CHUNK as i64,
		coords.z.0 as i64 * BLOCKS_PER_CHUNK as i64,
	);
}
