//! Manually maintained biome data (aliases and legacy biome IDs)

/// Biome ID aliases
///
/// Some biomes have been renamed or merged in recent Minecraft versions.
/// Maintain a list of aliases to support chunks saved by older versions.
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

/// Maps old numeric biome IDs to new string IDs
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