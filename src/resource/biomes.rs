use serde::{Deserialize, Serialize};

use super::Color;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BiomeGrassColorModifier {
	DarkForest,
	Swamp,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Biome {
	pub temp: i8,
	pub downfall: i8,
	pub water_color: Option<Color>,
	pub foliage_color: Option<Color>,
	pub grass_color: Option<Color>,
	pub grass_color_modifier: Option<BiomeGrassColorModifier>,
}

impl Biome {
	const fn new(temp: i16, downfall: i16) -> Biome {
		const fn encode(v: i16) -> i8 {
			(v / 5) as i8
		}
		Biome {
			temp: encode(temp),
			downfall: encode(downfall),
			grass_color_modifier: None,
			water_color: None,
			foliage_color: None,
			grass_color: None,
		}
	}

	const fn water(self, water_color: [u8; 3]) -> Biome {
		Biome {
			water_color: Some(Color(water_color)),
			..self
		}
	}

	const fn foliage(self, foliage_color: [u8; 3]) -> Biome {
		Biome {
			foliage_color: Some(Color(foliage_color)),
			..self
		}
	}

	const fn grass(self, grass_color: [u8; 3]) -> Biome {
		Biome {
			grass_color: Some(Color(grass_color)),
			..self
		}
	}

	const fn modify(self, grass_color_modifier: BiomeGrassColorModifier) -> Biome {
		Biome {
			grass_color_modifier: Some(grass_color_modifier),
			..self
		}
	}
}

// Data extracted from Minecraft code decompiled using https://github.com/Hexeption/MCP-Reborn

#[allow(clippy::zero_prefixed_literal)]
pub const BIOMES: &[(&str, Biome)] = {
	use BiomeGrassColorModifier::*;

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

pub const BIOME_ALIASES: &[(&str, &str)] = &[
	// Biomes fix
	("beaches", "beach"),
	("cold_beach", "snowy_beach"),
	("cold_deep_ocean", "deep_cold_ocean"),
	("extreme_hills", "mountains"),
	("extreme_hills_with_trees", "wooded_mountains"),
	("forest_hills", "wooded_hills"),
	("frozen_deep_ocean", "deep_frozen_ocean"),
	("hell", "nether_wastes"),
	("ice_flats", "snowy_tundra"),
	("ice_mountains", "snowy_mountains"),
	("lukewarm_deep_ocean", "deep_lukewarm_ocean"),
	("mesa", "badlands"),
	("mesa_clear_rock", "badlands_plateau"),
	("mesa_rock", "wooded_badlands_plateau"),
	("mushroom_island", "mushroom_fields"),
	("mushroom_island_shore", "mushroom_field_shore"),
	("mutated_birch_forest", "tall_birch_forest"),
	("mutated_birch_forest_hills", "tall_birch_hills"),
	("mutated_desert", "desert_lakes"),
	("mutated_extreme_hills", "gravelly_mountains"),
	(
		"mutated_extreme_hills_with_trees",
		"modified_gravelly_mountains",
	),
	("mutated_forest", "flower_forest"),
	("mutated_ice_flats", "ice_spikes"),
	("mutated_jungle", "modified_jungle"),
	("mutated_jungle_edge", "modified_jungle_edge"),
	("mutated_mesa", "eroded_badlands"),
	("mutated_mesa_clear_rock", "modified_badlands_plateau"),
	("mutated_mesa_rock", "modified_wooded_badlands_plateau"),
	("mutated_plains", "sunflower_plains"),
	("mutated_redwood_taiga", "giant_spruce_taiga"),
	("mutated_redwood_taiga_hills", "giant_spruce_taiga_hills"),
	("mutated_roofed_forest", "dark_forest_hills"),
	("mutated_savanna", "shattered_savanna"),
	("mutated_savanna_rock", "shattered_savanna_plateau"),
	("mutated_swampland", "swamp_hills"),
	("mutated_taiga", "taiga_mountains"),
	("mutated_taiga_cold", "snowy_taiga_mountains"),
	("redwood_taiga", "giant_tree_taiga"),
	("redwood_taiga_hills", "giant_tree_taiga_hills"),
	("roofed_forest", "dark_forest"),
	("savanna_rock", "savanna_plateau"),
	("sky", "the_end"),
	("sky_island_barren", "end_barrens"),
	("sky_island_high", "end_highlands"),
	("sky_island_low", "small_end_islands"),
	("sky_island_medium", "end_midlands"),
	("smaller_extreme_hills", "mountain_edge"),
	("stone_beach", "stone_shore"),
	("swampland", "swamp"),
	("taiga_cold", "snowy_taiga"),
	("taiga_cold_hills", "snowy_taiga_hills"),
	("void", "the_void"),
	("warm_deep_ocean", "deep_warm_ocean"),
	// Nether biome rename
	("nether", "nether_wastes"),
	// Caves and Cliffs biome renames
	("badlands_plateau", "badlands"),
	("bamboo_jungle_hills", "bamboo_jungle"),
	("birch_forest_hills", "birch_forest"),
	("dark_forest_hills", "dark_forest"),
	("desert_hills", "desert"),
	("desert_lakes", "desert"),
	("giant_spruce_taiga", "old_growth_spruce_taiga"),
	("giant_spruce_taiga_hills", "old_growth_spruce_taiga"),
	("giant_tree_taiga", "old_growth_pine_taiga"),
	("giant_tree_taiga_hills", "old_growth_pine_taiga"),
	("gravelly_mountains", "windswept_gravelly_hills"),
	("jungle_edge", "sparse_jungle"),
	("jungle_hills", "jungle"),
	("lofty_peaks", "jagged_peaks"),
	("modified_badlands_plateau", "badlands"),
	("modified_gravelly_mountains", "windswept_gravelly_hills"),
	("modified_jungle", "jungle"),
	("modified_jungle_edge", "sparse_jungle"),
	("modified_wooded_badlands_plateau", "wooded_badlands"),
	("mountain_edge", "windswept_hills"),
	("mountains", "windswept_hills"),
	("mushroom_field_shore", "mushroom_fields"),
	("shattered_savanna", "windswept_savanna"),
	("shattered_savanna_plateau", "windswept_savanna"),
	("snowcapped_peaks", "frozen_peaks"),
	("snowy_mountains", "snowy_plains"),
	("snowy_taiga_hills", "snowy_taiga"),
	("snowy_taiga_mountains", "snowy_taiga"),
	("snowy_tundra", "snowy_plains"),
	("stone_shore", "stony_shore"),
	("swamp_hills", "swamp"),
	("taiga_hills", "taiga"),
	("taiga_mountains", "taiga"),
	("tall_birch_forest", "old_growth_birch_forest"),
	("tall_birch_hills", "old_growth_birch_forest"),
	("wooded_badlands_plateau", "wooded_badlands"),
	("wooded_hills", "forest"),
	("wooded_mountains", "windswept_forest"),
	// Remove Deep Warm Ocean
	("deep_warm_ocean", "warm_ocean"),
];

pub fn legacy_biome(index: u8) -> &'static str {
	match index {
		0 => "ocean",
		1 => "plains",
		2 => "desert",
		3 => "mountains",
		4 => "forest",
		5 => "taiga",
		6 => "swamp",
		7 => "river",
		8 => "nether_wastes",
		9 => "the_end",
		10 => "frozen_ocean",
		11 => "frozen_river",
		12 => "snowy_tundra",
		13 => "snowy_mountains",
		14 => "mushroom_fields",
		15 => "mushroom_field_shore",
		16 => "beach",
		17 => "desert_hills",
		18 => "wooded_hills",
		19 => "taiga_hills",
		20 => "mountain_edge",
		21 => "jungle",
		22 => "jungle_hills",
		23 => "jungle_edge",
		24 => "deep_ocean",
		25 => "stone_shore",
		26 => "snowy_beach",
		27 => "birch_forest",
		28 => "birch_forest_hills",
		29 => "dark_forest",
		30 => "snowy_taiga",
		31 => "snowy_taiga_hills",
		32 => "giant_tree_taiga",
		33 => "giant_tree_taiga_hills",
		34 => "wooded_mountains",
		35 => "savanna",
		36 => "savanna_plateau",
		37 => "badlands",
		38 => "wooded_badlands_plateau",
		39 => "badlands_plateau",
		40 => "small_end_islands",
		41 => "end_midlands",
		42 => "end_highlands",
		43 => "end_barrens",
		44 => "warm_ocean",
		45 => "lukewarm_ocean",
		46 => "cold_ocean",
		47 => "deep_warm_ocean",
		48 => "deep_lukewarm_ocean",
		49 => "deep_cold_ocean",
		50 => "deep_frozen_ocean",
		127 => "the_void",
		129 => "sunflower_plains",
		130 => "desert_lakes",
		131 => "gravelly_mountains",
		132 => "flower_forest",
		133 => "taiga_mountains",
		134 => "swamp_hills",
		140 => "ice_spikes",
		149 => "modified_jungle",
		151 => "modified_jungle_edge",
		155 => "tall_birch_forest",
		156 => "tall_birch_hills",
		157 => "dark_forest_hills",
		158 => "snowy_taiga_mountains",
		160 => "giant_spruce_taiga",
		161 => "giant_spruce_taiga_hills",
		162 => "modified_gravelly_mountains",
		163 => "shattered_savanna",
		164 => "shattered_savanna_plateau",
		165 => "eroded_badlands",
		166 => "modified_wooded_badlands_plateau",
		167 => "modified_badlands_plateau",
		168 => "bamboo_jungle",
		169 => "bamboo_jungle_hills",
		170 => "soul_sand_valley",
		171 => "crimson_forest",
		172 => "warped_forest",
		173 => "basalt_deltas",
		174 => "dripstone_caves",
		175 => "lush_caves",
		177 => "meadow",
		178 => "grove",
		179 => "snowy_slopes",
		180 => "snowcapped_peaks",
		181 => "lofty_peaks",
		182 => "stony_peaks",
		_ => "ocean",
	}
}
