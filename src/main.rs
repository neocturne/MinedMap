use std::{
	fs,
	path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use clap::Parser;

use minedmap::{
	io::storage,
	resource::{self, Biome},
	types::*,
	world::{
		self,
		layer::{BlockInfo, BlockLightArray},
	},
};

#[derive(Debug, Parser)]
struct Args {
	/// Minecraft save directory
	input_dir: PathBuf,
	/// MinedMap data directory
	output_dir: PathBuf,
}

type RegionCoords = (i32, i32);
type ProcessedRegion = ChunkArray<
	Option<(
		Box<world::layer::BlockInfoArray>,
		Box<world::layer::BiomeArray>,
	)>,
>;

struct Config {
	region_dir: PathBuf,
	processed_dir: PathBuf,
	light_dir: PathBuf,
	map_dir: PathBuf,
}

impl Config {
	fn new(args: Args) -> Self {
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

	fn processed_path(&self, coords: RegionCoords, temp: bool) -> PathBuf {
		let filename = format!(
			"r.{}.{}.bin{}",
			coords.0,
			coords.1,
			if temp { ".tmp" } else { "" },
		);
		[&self.processed_dir, Path::new(&filename)].iter().collect()
	}

	fn light_path(&self, coords: RegionCoords, temp: bool) -> PathBuf {
		let filename = format!(
			"r.{}.{}.png{}",
			coords.0,
			coords.1,
			if temp { ".tmp" } else { "" },
		);
		[&self.light_dir, Path::new(&filename)].iter().collect()
	}

	fn map_path(&self, coords: RegionCoords, temp: bool) -> PathBuf {
		let filename = format!(
			"r.{}.{}.png{}",
			coords.0,
			coords.1,
			if temp { ".tmp" } else { "" },
		);
		[&self.map_dir, Path::new(&filename)].iter().collect()
	}
}

fn overlay_chunk<I, J>(image: &mut I, chunk: &J, coords: ChunkCoords)
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

/// Type with methods for processing the regions of a Minecraft save directory
struct RegionProcessor<'a> {
	block_types: resource::BlockTypes,
	biome_types: resource::BiomeTypes,
	config: &'a Config,
}

impl<'a> RegionProcessor<'a> {
	fn new(config: &'a Config) -> Self {
		RegionProcessor {
			block_types: resource::BlockTypes::default(),
			biome_types: resource::BiomeTypes::default(),
			config,
		}
	}

	/// Parses a filename in the format r.X.Z.mca into the contained X and Z values
	fn parse_region_filename(path: &Path) -> Option<RegionCoords> {
		let file_name = path.file_name()?.to_str()?;
		let parts: Vec<_> = file_name.split('.').collect();
		let &["r", x, z, "mca"] = parts.as_slice() else {
			return None;
		};

		Some((x.parse().ok()?, z.parse().ok()?))
	}

	/// Processes a single chunk
	fn process_chunk(
		&self,
		data: world::de::Chunk,
	) -> Result<
		Option<(
			(
				Box<world::layer::BlockInfoArray>,
				Box<world::layer::BiomeArray>,
			),
			Box<world::layer::BlockLightArray>,
		)>,
	> {
		let chunk = world::chunk::Chunk::new(&data, &self.block_types, &self.biome_types)?;
		world::layer::top_layer(&chunk)
	}

	fn chunk_lightmap(block_light: Box<BlockLightArray>) -> image::GrayAlphaImage {
		const N: u32 = BLOCKS_PER_CHUNK as u32;

		image::GrayAlphaImage::from_fn(N, N, |x, z| {
			let v: f32 = block_light[LayerBlockCoords {
				x: BlockX(x as u8),
				z: BlockZ(z as u8),
			}]
			.into();
			image::LumaA([0, (192.0 * (1.0 - v / 15.0)) as u8])
		})
	}

	fn save_region(&self, coords: RegionCoords, processed_region: &ProcessedRegion) -> Result<()> {
		let tmp_path = self.config.processed_path(coords, true);
		storage::write(&tmp_path, processed_region)?;

		let output_path = self.config.processed_path(coords, false);
		fs::rename(&tmp_path, &output_path).with_context(|| {
			format!(
				"Failed to rename {} to {}",
				tmp_path.display(),
				output_path.display(),
			)
		})?;

		Ok(())
	}

	fn save_lightmap(&self, coords: RegionCoords, lightmap: &image::GrayAlphaImage) -> Result<()> {
		let tmp_path = self.config.light_path(coords, true);
		lightmap
			.save_with_format(&tmp_path, image::ImageFormat::Png)
			.context("Failed to save image")?;

		let output_path = self.config.light_path(coords, false);
		fs::rename(&tmp_path, &output_path).with_context(|| {
			format!(
				"Failed to rename {} to {}",
				tmp_path.display(),
				output_path.display(),
			)
		})?;

		Ok(())
	}

	/// Processes a single region file
	fn process_region(&self, path: &Path, coords: RegionCoords) -> Result<()> {
		const N: u32 = (BLOCKS_PER_CHUNK * CHUNKS_PER_REGION) as u32;

		println!("Processing region r.{}.{}.mca", coords.0, coords.1);

		let mut processed_region = ProcessedRegion::default();
		let mut lightmap = image::GrayAlphaImage::new(N, N);

		minedmap::io::region::from_file(path)?.foreach_chunk(
			|chunk_coords, data: world::de::Chunk| {
				let Some((processed_chunk, block_light)) = self
					.process_chunk(data)
					.with_context(|| format!("Failed to process chunk {:?}", chunk_coords))?
				else {
					return Ok(());
				};
				processed_region[chunk_coords] = Some(processed_chunk);

				let chunk_lightmap = Self::chunk_lightmap(block_light);
				overlay_chunk(&mut lightmap, &chunk_lightmap, chunk_coords);

				Ok(())
			},
		)?;

		self.save_region(coords, &processed_region)?;
		self.save_lightmap(coords, &lightmap)?;

		Ok(())
	}

	/// Iterates over all region files of a Minecraft save directory
	///
	/// Returns a list of the coordinates of all processed regions
	fn run(self) -> Result<Vec<RegionCoords>> {
		let read_dir = self.config.region_dir.read_dir().with_context(|| {
			format!(
				"Failed to read directory {}",
				self.config.region_dir.display()
			)
		})?;

		fs::create_dir_all(&self.config.processed_dir).with_context(|| {
			format!(
				"Failed to create directory {}",
				self.config.processed_dir.display(),
			)
		})?;
		fs::create_dir_all(&self.config.light_dir).with_context(|| {
			format!(
				"Failed to create directory {}",
				self.config.light_dir.display(),
			)
		})?;

		let mut ret = Vec::new();

		for entry in read_dir.filter_map(|entry| entry.ok()).filter(|entry| {
			// We are only interested in regular files
			entry
				.file_type()
				.map(|file_type| file_type.is_file())
				.unwrap_or_default()
		}) {
			let path = entry.path();
			let Some(coords) = Self::parse_region_filename(&path) else {
				continue;
			};

			if let Err(err) = self.process_region(&path, coords) {
				eprintln!(
					"Failed to process region {}: {:?}",
					path.file_name().unwrap_or_default().to_string_lossy(),
					err,
				);
			}

			ret.push(coords);
		}

		Ok(ret)
	}
}

struct TileRenderer<'a> {
	config: &'a Config,
}

impl<'a> TileRenderer<'a> {
	fn new(config: &'a Config) -> Self {
		TileRenderer { config }
	}

	fn load_region(&self, coords: RegionCoords) -> Result<ProcessedRegion> {
		let processed_path = self.config.processed_path(coords, false);
		storage::read(&processed_path).context("Failed to load processed region data")
	}

	fn block_color(block: &BlockInfo, _biome: &Biome) -> [u8; 4] {
		let h = block
			.depth
			.map(|depth| 0.5 + 0.005 * depth.0 as f32)
			.unwrap_or_default();
		let c = block
			.block_type
			.color
			.0
			.map(|v| (f32::from(v) * h).clamp(0.0, 255.0) as u8);
		[c[0], c[1], c[2], 255]
	}

	fn render_chunk(
		image: &mut image::RgbaImage,
		coords: ChunkCoords,
		blocks: &world::layer::BlockInfoArray,
		biomes: &world::layer::BiomeArray,
	) {
		const N: u32 = BLOCKS_PER_CHUNK as u32;

		let chunk_image = image::RgbaImage::from_fn(N, N, |x, z| {
			let coords = LayerBlockCoords {
				x: BlockX(x as u8),
				z: BlockZ(z as u8),
			};
			image::Rgba(match (&blocks[coords], &biomes[coords]) {
				(Some(block), Some(biome)) => Self::block_color(block, biome),
				_ => [0, 0, 0, 0],
			})
		});
		overlay_chunk(image, &chunk_image, coords);
	}

	fn render_region(image: &mut image::RgbaImage, region: &ProcessedRegion) {
		for (coords, chunk) in region.iter() {
			let Some((blocks, biomes)) = chunk else {
				continue;
			};

			Self::render_chunk(image, coords, blocks, biomes);
		}
	}

	fn render_tile(&self, coords: RegionCoords) -> Result<()> {
		const N: u32 = (BLOCKS_PER_CHUNK * CHUNKS_PER_REGION) as u32;

		let tmp_path = self.config.map_path(coords, true);
		let output_path = self.config.map_path(coords, false);
		println!(
			"Rendering tile {}",
			output_path
				.file_name()
				.unwrap_or_default()
				.to_string_lossy(),
		);

		let region = self.load_region(coords)?;
		let mut image = image::RgbaImage::new(N, N);
		Self::render_region(&mut image, &region);
		image
			.save_with_format(&tmp_path, image::ImageFormat::Png)
			.context("Failed to save image")?;
		fs::rename(&tmp_path, &output_path).with_context(|| {
			format!(
				"Failed to rename {} to {}",
				tmp_path.display(),
				output_path.display(),
			)
		})?;

		Ok(())
	}

	fn run(self, regions: &[RegionCoords]) -> Result<()> {
		fs::create_dir_all(&self.config.map_dir).with_context(|| {
			format!(
				"Failed to create directory {}",
				self.config.map_dir.display(),
			)
		})?;

		for &coords in regions {
			if let Err(err) = self.render_tile(coords) {
				eprintln!("Failed to render tile {:?}: {:?}", coords, err,);
			}
		}

		Ok(())
	}
}

fn main() -> Result<()> {
	let args = Args::parse();
	let config = Config::new(args);

	let regions = RegionProcessor::new(&config).run()?;
	TileRenderer::new(&config).run(&regions)?;

	Ok(())
}
