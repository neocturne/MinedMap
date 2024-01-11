//! Functions for computations of block colors

use super::{Biome, BlockColor, Color, Colorf};

/// Converts an u8 RGB color to a float vector
fn color_vec_unscaled(color: Color) -> Colorf {
	Colorf::from_array(color.0.map(f32::from))
}

/// Converts an u8 RGB color to a float vector, scaling the components to 0.0..1.0
fn color_vec(color: Color) -> Colorf {
	color_vec_unscaled(color) / 255.0
}

/// Helper for grass and foliage colors
///
/// Biome temperature and downfall are modified based on the depth value
/// before using them to compute the final color
fn color_from_params(colors: &[Colorf; 3], biome: &Biome, depth: f32) -> Colorf {
	let temp = (biome.temp() - f32::max((depth - 64.0) / 600.0, 0.0)).clamp(0.0, 1.0);
	let downfall = biome.downfall().clamp(0.0, 1.0) * temp;

	colors[0] + temp * colors[1] + downfall * colors[2]
}

/// Extension trait with helpers for computing biome-specific block colors
trait BiomeExt {
	/// Returns the grass color of the biome at a given depth
	fn grass_color(&self, depth: f32) -> Colorf;
	/// Returns the foliage color of the biome at a given depth
	fn foliage_color(&self, depth: f32) -> Colorf;
	/// Returns the water color of the biome
	fn water_color(&self) -> Colorf;
}

impl BiomeExt for Biome {
	fn grass_color(&self, depth: f32) -> Colorf {
		use super::BiomeGrassColorModifier::*;

		/// Color matrix extracted from grass color texture
		const GRASS_COLORS: [Colorf; 3] = [
			Colorf::new(0.502, 0.706, 0.592),   // lower right
			Colorf::new(0.247, 0.012, -0.259),  // lower left - lower right
			Colorf::new(-0.471, 0.086, -0.133), // upper left - lower left
		];
		/// Used for dark forst grass color modifier
		const DARK_FOREST_GRASS_COLOR: Colorf = Colorf::new(0.157, 0.204, 0.039); // == color_vec(Color([40, 52, 10]))
		/// Grass color in swamp biomes
		const SWAMP_GRASS_COLOR: Colorf = Colorf::new(0.416, 0.439, 0.224); // == color_vec(Color([106, 112, 57]))

		let regular_color = || {
			self.grass_color
				.map(color_vec)
				.unwrap_or_else(|| color_from_params(&GRASS_COLORS, self, depth))
		};

		match self.grass_color_modifier {
			Some(DarkForest) => 0.5 * (regular_color() + DARK_FOREST_GRASS_COLOR),
			Some(Swamp) => SWAMP_GRASS_COLOR,
			None => regular_color(),
		}
	}

	fn foliage_color(&self, depth: f32) -> Colorf {
		/// Color matrix extracted from foliage color texture
		const FOLIAGE_COLORS: [Colorf; 3] = [
			Colorf::new(0.376, 0.631, 0.482),   // lower right
			Colorf::new(0.306, 0.012, -0.317),  // lower left - lower right
			Colorf::new(-0.580, 0.106, -0.165), // upper left - lower left
		];

		self.foliage_color
			.map(color_vec)
			.unwrap_or_else(|| color_from_params(&FOLIAGE_COLORS, self, depth))
	}

	fn water_color(&self) -> Colorf {
		/// Default biome water color
		///
		/// Used for biomes that don't explicitly set a water color
		const DEFAULT_WATER_COLOR: Colorf = Colorf::new(0.247, 0.463, 0.894); // == color_vec(Color([63, 118, 228]))

		self.water_color
			.map(color_vec)
			.unwrap_or(DEFAULT_WATER_COLOR)
	}
}

/// Color multiplier for birch leaves
const BIRCH_COLOR: Colorf = Colorf::new(0.502, 0.655, 0.333); // == color_vec(Color([128, 167, 85]))
/// Color multiplier for spruce leaves
const EVERGREEN_COLOR: Colorf = Colorf::new(0.380, 0.600, 0.380); // == color_vec(Color([97, 153, 97]))

/// Determined if calling [block_color] for a given [BlockColor] needs biome information
pub fn needs_biome(block: BlockColor) -> bool {
	use super::BlockFlag::*;

	block.is(Grass) || block.is(Foliage) || block.is(Water)
}

/// Determined the block color to display for a given [BlockColor]
///
/// [needs_biome] must be used to determine whether passing a [Biome] is necessary.
/// Will panic if a [Biome] is necessary, but none is passed.
pub fn block_color(block: BlockColor, biome: Option<&Biome>, depth: f32) -> Colorf {
	use super::BlockFlag::*;

	let get_biome = || biome.expect("needs biome to determine block color");

	let mut color = color_vec_unscaled(block.color);

	if block.is(Grass) {
		color *= get_biome().grass_color(depth);
	}
	if block.is(Foliage) {
		color *= get_biome().foliage_color(depth);
	}
	if block.is(Birch) {
		color *= BIRCH_COLOR;
	}
	if block.is(Spruce) {
		color *= EVERGREEN_COLOR;
	}
	if block.is(Water) {
		color *= get_biome().water_color();
	}

	color * (0.5 + 0.005 * depth)
}
