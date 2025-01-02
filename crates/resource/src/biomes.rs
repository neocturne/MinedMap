//! Biome data
//!
//! This file is generated using resource/biomes.py, do not edit

use super::*;
use BiomeGrassColorModifier::*;

/// List if known biomes and their properties
pub const BIOMES: &[(&str, Biome)] = &[
	(
		"badlands",
		Biome::new(200, 0)
			.foliage([158, 129, 77])
			.grass([144, 129, 77]),
	),
	("bamboo_jungle", Biome::new(95, 90)),
	("basalt_deltas", Biome::new(200, 0)),
	("beach", Biome::new(80, 40)),
	("birch_forest", Biome::new(60, 60)),
	(
		"cherry_grove",
		Biome::new(50, 80)
			.foliage([182, 219, 97])
			.grass([182, 219, 97])
			.water([93, 183, 239]),
	),
	("cold_ocean", Biome::new(50, 50).water([61, 87, 214])),
	("crimson_forest", Biome::new(200, 0)),
	("dark_forest", Biome::new(70, 80).modify(DarkForest)),
	("deep_cold_ocean", Biome::new(50, 50).water([61, 87, 214])),
	("deep_dark", Biome::new(80, 40)),
	("deep_frozen_ocean", Biome::new(50, 50).water([57, 56, 201])),
	(
		"deep_lukewarm_ocean",
		Biome::new(50, 50).water([69, 173, 242]),
	),
	("deep_ocean", Biome::new(50, 50)),
	("desert", Biome::new(200, 0)),
	("dripstone_caves", Biome::new(80, 40)),
	("end_barrens", Biome::new(50, 50)),
	("end_highlands", Biome::new(50, 50)),
	("end_midlands", Biome::new(50, 50)),
	(
		"eroded_badlands",
		Biome::new(200, 0)
			.foliage([158, 129, 77])
			.grass([144, 129, 77]),
	),
	("flower_forest", Biome::new(70, 80)),
	("forest", Biome::new(70, 80)),
	("frozen_ocean", Biome::new(0, 50).water([57, 56, 201])),
	("frozen_peaks", Biome::new(-70, 90)),
	("frozen_river", Biome::new(0, 50).water([57, 56, 201])),
	("grove", Biome::new(-20, 80)),
	("ice_spikes", Biome::new(0, 50)),
	("jagged_peaks", Biome::new(-70, 90)),
	("jungle", Biome::new(95, 90)),
	("lukewarm_ocean", Biome::new(50, 50).water([69, 173, 242])),
	("lush_caves", Biome::new(50, 50)),
	(
		"mangrove_swamp",
		Biome::new(80, 90)
			.foliage([141, 177, 39])
			.modify(Swamp)
			.water([58, 122, 106]),
	),
	("meadow", Biome::new(50, 80).water([14, 78, 207])),
	("mushroom_fields", Biome::new(90, 100)),
	("nether_wastes", Biome::new(200, 0)),
	("ocean", Biome::new(50, 50)),
	("old_growth_birch_forest", Biome::new(60, 60)),
	("old_growth_pine_taiga", Biome::new(30, 80)),
	("old_growth_spruce_taiga", Biome::new(25, 80)),
	(
		"pale_garden",
		Biome::new(70, 80)
			.foliage([135, 141, 118])
			.grass([119, 130, 114])
			.water([118, 136, 157]),
	),
	("plains", Biome::new(80, 40)),
	("river", Biome::new(50, 50)),
	("savanna", Biome::new(200, 0)),
	("savanna_plateau", Biome::new(200, 0)),
	("small_end_islands", Biome::new(50, 50)),
	("snowy_beach", Biome::new(5, 30).water([61, 87, 214])),
	("snowy_plains", Biome::new(0, 50)),
	("snowy_slopes", Biome::new(-30, 90)),
	("snowy_taiga", Biome::new(-50, 40).water([61, 87, 214])),
	("soul_sand_valley", Biome::new(200, 0)),
	("sparse_jungle", Biome::new(95, 80)),
	("stony_peaks", Biome::new(100, 30)),
	("stony_shore", Biome::new(20, 30)),
	("sunflower_plains", Biome::new(80, 40)),
	(
		"swamp",
		Biome::new(80, 90)
			.foliage([106, 112, 57])
			.modify(Swamp)
			.water([97, 123, 100]),
	),
	("taiga", Biome::new(25, 80)),
	("the_end", Biome::new(50, 50)),
	("the_void", Biome::new(50, 50)),
	("warm_ocean", Biome::new(50, 50).water([67, 213, 238])),
	("warped_forest", Biome::new(200, 0)),
	("windswept_forest", Biome::new(20, 30)),
	("windswept_gravelly_hills", Biome::new(20, 30)),
	("windswept_hills", Biome::new(20, 30)),
	("windswept_savanna", Biome::new(200, 0)),
	(
		"wooded_badlands",
		Biome::new(200, 0)
			.foliage([158, 129, 77])
			.grass([144, 129, 77]),
	),
];
