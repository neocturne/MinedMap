use super::{Biome, BlockType, Color};

use glam::Vec3;

fn color_vec_unscaled(color: Color) -> Vec3 {
	Vec3::from_array(color.0.map(f32::from))
}

fn color_vec(color: Color) -> Vec3 {
	color_vec_unscaled(color) / 255.0
}

fn color_from_params(colors: &[Vec3; 3], biome: &Biome, depth: f32) -> Vec3 {
	let temp = (biome.temp() - f32::max((depth - 64.0) / 600.0, 0.0)).clamp(0.0, 1.0);
	let downfall = biome.downfall().clamp(0.0, 1.0) * temp;

	colors[0] + temp * colors[1] + downfall * colors[2]
}

trait BiomeExt {
	fn grass_color(&self, depth: f32) -> Vec3;
	fn foliage_color(&self, depth: f32) -> Vec3;
	fn water_color(&self) -> Vec3;
}

impl BiomeExt for Biome {
	fn grass_color(&self, depth: f32) -> Vec3 {
		const GRASS_COLORS: [Vec3; 3] = [
			Vec3::new(0.502, 0.706, 0.592),   // lower right
			Vec3::new(0.247, 0.012, -0.259),  // lower left - lower right
			Vec3::new(-0.471, 0.086, -0.133), // upper left - lower left
		];

		self.grass_color
			.map(color_vec)
			.unwrap_or_else(|| color_from_params(&GRASS_COLORS, self, depth))
	}

	fn foliage_color(&self, depth: f32) -> Vec3 {
		use super::BiomeGrassColorModifier::*;

		const FOLIAGE_COLORS: [Vec3; 3] = [
			Vec3::new(0.502, 0.706, 0.592),   // lower right
			Vec3::new(0.247, 0.012, -0.259),  // lower left - lower right
			Vec3::new(-0.471, 0.086, -0.133), // upper left - lower left
		];
		const DARK_FOREST_COLOR: Vec3 = Vec3::new(0.157, 0.204, 0.039); // == color_vec(Color([40, 52, 10]))
		const SWAMP_FOLIAGE_COLOR: Vec3 = Vec3::new(0.416, 0.439, 0.224); // == color_vec(Color([106, 112, 57]))

		let regular_color = || {
			self.foliage_color
				.map(color_vec)
				.unwrap_or_else(|| color_from_params(&FOLIAGE_COLORS, self, depth))
		};

		match self.grass_color_modifier {
			Some(DarkForest) => 0.5 * (regular_color() + DARK_FOREST_COLOR),
			Some(Swamp) => SWAMP_FOLIAGE_COLOR,
			None => regular_color(),
		}
	}

	fn water_color(&self) -> Vec3 {
		const DEFAULT_WATER_COLOR: Vec3 = Vec3::new(0.247, 0.463, 0.894); // == color_vec(Color([63, 118, 228]))

		self.water_color
			.map(color_vec)
			.unwrap_or(DEFAULT_WATER_COLOR)
	}
}

const BIRCH_COLOR: Vec3 = Vec3::new(0.502, 0.655, 0.333); // == color_vec(Color([128, 167, 85]))
const EVERGREEN_COLOR: Vec3 = Vec3::new(0.380, 0.600, 0.380); // == color_vec(Color([97, 153, 97]))

pub fn block_color(block: BlockType, biome: &Biome, depth: f32) -> [u8; 4] {
	use super::BlockFlag::*;

	let mut color = color_vec_unscaled(block.color);

	if block.is(Grass) {
		color *= biome.grass_color(depth);
	}
	if block.is(Foliage) {
		color *= biome.foliage_color(depth);
	}
	if block.is(Birch) {
		color *= BIRCH_COLOR;
	}
	if block.is(Spruce) {
		color *= EVERGREEN_COLOR;
	}
	if block.is(Water) {
		color *= biome.water_color();
	}

	color *= 0.5 + 0.005 * depth;

	[color[0] as u8, color[1] as u8, color[2] as u8, 255]
}
