//! Biome data

use super::*;

/// Standard biome specifications
pub const BIOMES: &[(&str, Biome)] = {
	use BiomeGrassColorModifier::*;

	// Data extracted from Minecraft code decompiled using https://github.com/Hexeption/MCP-Reborn

	// We can't use floats in const functions, to temperature and downfall values
	// are specified multipled by 100. The underscore is used in place of the decimal point
	// of the original values.

	#[allow(clippy::zero_prefixed_literal)]
	&[
		// Overworld
		(
			"badlands",
			Biome::new(2_00, 0_00)
				.foliage([158, 129, 77])
				.grass([144, 129, 77]),
		),
		("bamboo_jungle", Biome::new(0_95, 0_90)),
		("beach", Biome::new(0_80, 0_40)),
		("birch_forest", Biome::new(0_60, 0_60)),
		(
			"cherry_grove",
			Biome::new(0_50, 0_80)
				.water([93, 183, 239])
				.grass([182, 219, 97])
				.foliage([182, 219, 97]),
		),
		("cold_ocean", Biome::new(0_50, 0_50).water([61, 87, 214])),
		("dark_forest", Biome::new(0_70, 0_80).modify(DarkForest)),
		(
			"deep_cold_ocean",
			Biome::new(0_50, 0_50).water([61, 87, 214]),
		),
		("deep_dark", Biome::new(0_80, 0_40)),
		(
			"deep_frozen_ocean",
			Biome::new(0_50, 0_50).water([57, 56, 201]),
		),
		(
			"deep_lukewarm_ocean",
			Biome::new(0_50, 0_50).water([69, 173, 242]),
		),
		("deep_ocean", Biome::new(0_50, 0_50)),
		("desert", Biome::new(2_00, 0_00)),
		("dripstone_caves", Biome::new(0_80, 0_40)),
		(
			"eroded_badlands",
			Biome::new(2_00, 0_00)
				.foliage([158, 129, 77])
				.grass([144, 129, 77]),
		),
		("flower_forest", Biome::new(0_70, 0_80)),
		("forest", Biome::new(0_70, 0_80)),
		("frozen_ocean", Biome::new(0_00, 0_50).water([57, 56, 201])),
		("frozen_peaks", Biome::new(-0_70, 0_90)),
		("frozen_river", Biome::new(0_00, 0_50).water([57, 56, 201])),
		("grove", Biome::new(-0_20, 0_80)),
		("ice_spikes", Biome::new(0_00, 0_50)),
		("jagged_peaks", Biome::new(-0_70, 0_90)),
		("jungle", Biome::new(0_95, 0_90)),
		(
			"lukewarm_ocean",
			Biome::new(0_50, 0_50).water([69, 173, 242]),
		),
		("lush_caves", Biome::new(0_50, 0_50)),
		(
			"mangrove_swamp",
			Biome::new(0_80, 0_90)
				.water([58, 122, 106])
				.foliage([141, 177, 39])
				.modify(Swamp),
		),
		("meadow", Biome::new(0_50, 0_80).water([14, 78, 207])),
		("mushroom_fields", Biome::new(0_90, 1_00)),
		("ocean", Biome::new(0_50, 0_50)),
		("old_growth_birch_forest", Biome::new(0_60, 0_60)),
		("old_growth_pine_taiga", Biome::new(0_30, 0_80)),
		("old_growth_spruce_taiga", Biome::new(0_25, 0_80)),
		(
			"pale_garden",
			Biome::new(0_70, 0_80)
				.water([118, 136, 157])
				.foliage([135, 141, 118])
				.grass([119, 130, 114]),
		),
		("plains", Biome::new(0_80, 0_40)),
		("river", Biome::new(0_50, 0_50)),
		("savanna", Biome::new(2_00, 0_00)),
		("savanna_plateau", Biome::new(2_00, 0_00)),
		("snowy_beach", Biome::new(0_05, 0_30).water([61, 87, 214])),
		("snowy_plains", Biome::new(0_00, 0_50)),
		("snowy_slopes", Biome::new(-0_30, 0_90)),
		("snowy_taiga", Biome::new(-0_50, 0_40).water([61, 87, 214])),
		("sparse_jungle", Biome::new(0_95, 0_80)),
		("stony_peaks", Biome::new(1_00, 0_30)),
		("stony_shore", Biome::new(0_20, 0_30)),
		("sunflower_plains", Biome::new(0_80, 0_40)),
		(
			"swamp",
			Biome::new(0_80, 0_90)
				.water([97, 123, 100])
				.foliage([106, 112, 57])
				.modify(Swamp),
		),
		("taiga", Biome::new(0_25, 0_80)),
		("the_void", Biome::new(0_50, 0_50)),
		("warm_ocean", Biome::new(0_50, 0_50).water([67, 213, 238])),
		("windswept_forest", Biome::new(0_20, 0_30)),
		("windswept_gravelly_hills", Biome::new(0_20, 0_30)),
		("windswept_hills", Biome::new(0_20, 0_30)),
		("windswept_savanna", Biome::new(2_00, 0_00)),
		(
			"wooded_badlands",
			Biome::new(2_00, 0_00)
				.foliage([158, 129, 77])
				.grass([144, 129, 77]),
		),
		// Nether
		("basalt_deltas", Biome::new(2_00, 0_00)),
		("crimson_forest", Biome::new(2_00, 0_00)),
		("nether_wastes", Biome::new(2_00, 0_00)),
		("soul_sand_valley", Biome::new(2_00, 0_00)),
		("warped_forest", Biome::new(2_00, 0_00)),
		// End
		("end_barrens", Biome::new(0_50, 0_50)),
		("end_highlands", Biome::new(0_50, 0_50)),
		("end_midlands", Biome::new(0_50, 0_50)),
		("small_end_islands", Biome::new(0_50, 0_50)),
		("the_end", Biome::new(0_50, 0_50)),
	]
};
