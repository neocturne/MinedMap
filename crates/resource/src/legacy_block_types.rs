//! Mapping of old numeric block type and damage/subtype IDs to new string IDs

/// Helper for block types that don't use the damage/subtype data
const fn simple(id: &str) -> [&str; 16] {
	[
		id, id, id, id, id, id, id, id, id, id, id, id, id, id, id, id,
	]
}

/// Default block type for unassigned numeric IDs
const DEF: &str = "air";
/// Default entry for block type numbers that are unassigned regardless of subtype
const EMPTY: [&str; 16] = simple(DEF);

/// Mapping from each numeric block type and damage/subtype ID to new string ID
#[allow(clippy::large_const_arrays)]
pub const LEGACY_BLOCK_TYPES: [[&str; 16]; 256] = [
	/* 0 */
	simple("air"),
	/* 1 */
	[
		"stone",
		"granite",
		"polished_granite",
		"diorite",
		"polished_diorite",
		"andesite",
		"polished_andesite",
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
	],
	/* 2 */
	simple("grass_block"),
	/* 3 */
	[
		"dirt",
		"coarse_dirt",
		"podzol",
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
	],
	/* 4 */
	simple("cobblestone"),
	/* 5 */
	[
		"oak_planks",
		"spruce_planks",
		"birch_planks",
		"jungle_planks",
		"acacia_planks",
		"dark_oak_planks",
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
	],
	/* 6 */
	[
		"oak_sapling",
		"spruce_sapling",
		"birch_sapling",
		"jungle_sapling",
		"acacia_sapling",
		"dark_oak_sapling",
		DEF,
		DEF,
		"oak_sapling",
		"spruce_sapling",
		"birch_sapling",
		"jungle_sapling",
		"acacia_sapling",
		"dark_oak_sapling",
		DEF,
		DEF,
	],
	/* 7 */
	simple("bedrock"),
	/* 8 */
	simple("water"),
	/* 9 */
	simple("water"),
	/* 10 */
	simple("lava"),
	/* 11 */
	simple("lava"),
	/* 12 */
	[
		"sand", "red_sand", DEF, DEF, DEF, DEF, DEF, DEF, DEF, DEF, DEF, DEF, DEF, DEF, DEF, DEF,
	],
	/* 13 */
	simple("gravel"),
	/* 14 */
	simple("gold_ore"),
	/* 15 */
	simple("iron_ore"),
	/* 16 */
	simple("coal_ore"),
	/* 17 */
	[
		"oak_log",
		"spruce_log",
		"birch_log",
		"jungle_log",
		"oak_log",
		"spruce_log",
		"birch_log",
		"jungle_log",
		"oak_log",
		"spruce_log",
		"birch_log",
		"jungle_log",
		"oak_log",
		"spruce_log",
		"birch_log",
		"jungle_log",
	],
	/* 18 */
	[
		"oak_leaves",
		"spruce_leaves",
		"birch_leaves",
		"jungle_leaves",
		"oak_leaves",
		"spruce_leaves",
		"birch_leaves",
		"jungle_leaves",
		"oak_leaves",
		"spruce_leaves",
		"birch_leaves",
		"jungle_leaves",
		"oak_leaves",
		"spruce_leaves",
		"birch_leaves",
		"jungle_leaves",
	],
	/* 19 */
	simple("sponge"),
	/* 20 */
	simple("glass"),
	/* 21 */
	simple("lapis_ore"),
	/* 22 */
	simple("lapis_block"),
	/* 23 */
	simple("dispenser"),
	/* 24 */
	simple("sandstone"),
	/* 25 */
	simple("note_block"),
	/* 26 */
	EMPTY, // bed
	/* 27 */
	simple("powered_rail"),
	/* 28 */
	simple("detector_rail"),
	/* 29 */
	simple("sticky_piston"),
	/* 30 */
	simple("cobweb"),
	/* 31 */
	[
		"grass", "fern", DEF, DEF, DEF, DEF, DEF, DEF, DEF, DEF, DEF, DEF, DEF, DEF, DEF, DEF,
	],
	/* 32 */
	simple("dead_bush"),
	/* 33 */
	simple("piston"),
	/* 34 */
	simple("piston_head"),
	/* 35 */
	[
		"white_wool",
		"orange_wool",
		"magenta_wool",
		"light_blue_wool",
		"yellow_wool",
		"lime_wool",
		"pink_wool",
		"gray_wool",
		"light_gray_wool",
		"cyan_wool",
		"purple_wool",
		"blue_wool",
		"brown_wool",
		"green_wool",
		"red_wool",
		"black_wool",
	],
	/* 36 */
	simple("moving_piston"),
	/* 37 */
	simple("dandelion"),
	/* 38 */
	[
		"poppy",
		"blue_orchid",
		"allium",
		"azure_bluet",
		"red_tulip",
		"orange_tulip",
		"white_tulip",
		"pink_tulip",
		"oxeye_daisy",
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
	],
	/* 39 */
	simple("brown_mushroom"),
	/* 40 */
	simple("red_mushroom"),
	/* 41 */
	simple("gold_block"),
	/* 42 */
	simple("iron_block"),
	/* 43 */
	[
		"smooth_stone_slab",
		"sandstone_slab",
		"oak_slab",
		"cobblestone_slab",
		"brick_slab",
		"stone_brick_slab",
		"nether_brick_slab",
		"quartz_slab",
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
	],
	/* 44 */
	[
		"smooth_stone_slab",
		"sandstone_slab",
		"oak_slab",
		"cobblestone_slab",
		"brick_slab",
		"stone_brick_slab",
		"nether_brick_slab",
		"quartz_slab",
		"stone_slab",
		"sandstone_slab",
		"oak_slab",
		"cobblestone_slab",
		"brick_slab",
		"stone_brick_slab",
		"nether_brick_slab",
		"quartz_slab",
	],
	/* 45 */
	simple("bricks"),
	/* 46 */
	simple("tnt"),
	/* 47 */
	simple("bookshelf"),
	/* 48 */
	simple("mossy_cobblestone"),
	/* 49 */
	simple("obsidian"),
	/* 50 */
	[
		DEF,
		"wall_torch",
		"wall_torch",
		"wall_torch",
		"wall_torch",
		"torch",
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
	],
	/* 51 */
	simple("fire"),
	/* 52 */
	simple("spawner"),
	/* 53 */
	simple("oak_stairs"),
	/* 54 */
	simple("chest"),
	/* 55 */
	simple("redstone_wire"),
	/* 56 */
	simple("diamond_ore"),
	/* 57 */
	simple("diamond_block"),
	/* 58 */
	simple("crafting_table"),
	/* 59 */
	simple("wheat"),
	/* 60 */
	simple("farmland"),
	/* 61 */
	simple("furnace"),
	/* 62 */
	simple("furnace"),
	/* 63 */
	simple("sign"),
	/* 64 */
	simple("oak_door"),
	/* 65 */
	simple("ladder"),
	/* 66 */
	simple("rail"),
	/* 67 */
	simple("cobblestone_stairs"),
	/* 68 */
	simple("wall_sign"),
	/* 69 */
	simple("lever"),
	/* 70 */
	simple("stone_pressure_plate"),
	/* 71 */
	simple("iron_door"),
	/* 72 */
	simple("oak_pressure_plate"),
	/* 73 */
	simple("redstone_ore"),
	/* 74 */
	simple("redstone_ore"),
	/* 75 */
	[
		DEF,
		"redstone_wall_torch",
		"redstone_wall_torch",
		"redstone_wall_torch",
		"redstone_wall_torch",
		"redstone_torch",
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
	],
	/* 76 */
	[
		DEF,
		"redstone_wall_torch",
		"redstone_wall_torch",
		"redstone_wall_torch",
		"redstone_wall_torch",
		"redstone_torch",
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
	],
	/* 77 */
	simple("stone_button"),
	/* 78 */
	simple("snow"),
	/* 79 */
	simple("ice"),
	/* 80 */
	simple("snow_block"),
	/* 81 */
	simple("cactus"),
	/* 82 */
	simple("clay"),
	/* 83 */
	simple("sugar_cane"),
	/* 84 */
	simple("jukebox"),
	/* 85 */
	simple("oak_fence"),
	/* 86 */
	simple("pumpkin"),
	/* 87 */
	simple("netherrack"),
	/* 88 */
	simple("soul_sand"),
	/* 89 */
	simple("glowstone"),
	/* 90 */
	simple("nether_portal"),
	/* 91 */
	simple("pumpkin"),
	/* 92 */
	simple("cake"),
	/* 93 */
	simple("repeater"),
	/* 94 */
	simple("repeater"),
	/* 95 */
	[
		"white_stained_glass",
		"orange_stained_glass",
		"magenta_stained_glass",
		"light_blue_stained_glass",
		"yellow_stained_glass",
		"lime_stained_glass",
		"pink_stained_glass",
		"gray_stained_glass",
		"light_gray_stained_glass",
		"cyan_stained_glass",
		"purple_stained_glass",
		"blue_stained_glass",
		"brown_stained_glass",
		"green_stained_glass",
		"red_stained_glass",
		"black_stained_glass",
	],
	/* 96 */
	simple("oak_trapdoor"),
	/* 97 */
	[
		"infested_stone",
		"infested_cobblestone",
		"infested_stone_bricks",
		"infested_mossy_stone_bricks",
		"infested_cracked_stone_bricks",
		"infested_chiseled_stone_bricks",
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
	],
	/* 98 */
	[
		"stone_bricks",
		"mossy_stone_bricks",
		"cracked_stone_bricks",
		"chiseled_stone_bricks",
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
	],
	/* 99 */
	simple("brown_mushroom_block"),
	/* 100 */
	simple("red_mushroom_block"),
	/* 101 */
	simple("iron_bars"),
	/* 102 */
	simple("glass_pane"),
	/* 103 */
	simple("melon"),
	/* 104 */
	simple("pumpkin_stem"),
	/* 105 */
	simple("melon_stem"),
	/* 106 */
	simple("vine"),
	/* 107 */
	simple("oak_fence_gate"),
	/* 108 */
	simple("brick_stairs"),
	/* 109 */
	simple("stone_brick_stairs"),
	/* 110 */
	simple("mycelium"),
	/* 111 */
	simple("lily_pad"),
	/* 112 */
	simple("nether_bricks"),
	/* 113 */
	simple("nether_brick_fence"),
	/* 114 */
	simple("nether_brick_stairs"),
	/* 115 */
	simple("nether_wart"),
	/* 116 */
	simple("enchanting_table"),
	/* 117 */
	simple("brewing_stand"),
	/* 118 */
	simple("cauldron"),
	/* 119 */
	simple("end_portal"),
	/* 120 */
	simple("end_portal_frame"),
	/* 121 */
	simple("end_stone"),
	/* 122 */
	simple("dragon_egg"),
	/* 123 */
	simple("redstone_lamp"),
	/* 124 */
	simple("redstone_lamp"),
	/* 125 */
	[
		"oak_slab",
		"spruce_slab",
		"birch_slab",
		"jungle_slab",
		"acacia_slab",
		"dark_oak_slab",
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
	],
	/* 126 */
	[
		"oak_slab",
		"spruce_slab",
		"birch_slab",
		"jungle_slab",
		"acacia_slab",
		"dark_oak_slab",
		DEF,
		DEF,
		"oak_slab",
		"spruce_slab",
		"birch_slab",
		"jungle_slab",
		"acacia_slab",
		"dark_oak_slab",
		DEF,
		DEF,
	],
	/* 127 */
	simple("cocoa"),
	/* 128 */
	simple("sandstone_stairs"),
	/* 129 */
	simple("emerald_ore"),
	/* 130 */
	simple("ender_chest"),
	/* 131 */
	simple("tripwire_hook"),
	/* 132 */
	simple("tripwire"),
	/* 133 */
	simple("emerald_block"),
	/* 134 */
	simple("spruce_stairs"),
	/* 135 */
	simple("birch_stairs"),
	/* 136 */
	simple("jungle_stairs"),
	/* 137 */
	simple("command_block"),
	/* 138 */
	simple("beacon"),
	/* 139 */
	[
		"cobblestone_wall",
		"mossy_cobblestone_wall",
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
	],
	/* 140 */
	simple("flower_pot"),
	/* 141 */
	simple("carrots"),
	/* 142 */
	simple("potatoes"),
	/* 143 */
	EMPTY,
	/* 144 */
	EMPTY,
	/* 145 */
	[
		"anvil",
		"anvil",
		"anvil",
		"anvil",
		"chipped_anvil",
		"chipped_anvil",
		"chipped_anvil",
		"chipped_anvil",
		"damaged_anvil",
		"damaged_anvil",
		"damaged_anvil",
		"damaged_anvil",
		DEF,
		DEF,
		DEF,
		DEF,
	],
	/* 146 */
	simple("trapped_chest"),
	/* 147 */
	simple("light_weighted_pressure_plate"),
	/* 148 */
	simple("heavy_weighted_pressure_plate"),
	/* 149 */
	simple("comparator"),
	/* 150 */
	simple("comparator"),
	/* 151 */
	simple("daylight_detector"),
	/* 152 */
	simple("redstone_block"),
	/* 153 */
	simple("nether_quartz_ore"),
	/* 154 */
	simple("hopper"),
	/* 155 */
	simple("quartz_block"),
	/* 156 */
	simple("quartz_stairs"),
	/* 157 */
	simple("activator_rail"),
	/* 158 */
	simple("dropper"),
	/* 159 */
	[
		"white_terracotta",
		"orange_terracotta",
		"magenta_terracotta",
		"light_blue_terracotta",
		"yellow_terracotta",
		"lime_terracotta",
		"pink_terracotta",
		"gray_terracotta",
		"light_gray_terracotta",
		"cyan_terracotta",
		"purple_terracotta",
		"blue_terracotta",
		"brown_terracotta",
		"green_terracotta",
		"red_terracotta",
		"black_terracotta",
	],
	/* 160 */
	[
		"white_stained_glass_pane",
		"orange_stained_glass_pane",
		"magenta_stained_glass_pane",
		"light_blue_stained_glass_pane",
		"yellow_stained_glass_pane",
		"lime_stained_glass_pane",
		"pink_stained_glass_pane",
		"gray_stained_glass_pane",
		"light_gray_stained_glass_pane",
		"cyan_stained_glass_pane",
		"purple_stained_glass_pane",
		"blue_stained_glass_pane",
		"brown_stained_glass_pane",
		"green_stained_glass_pane",
		"red_stained_glass_pane",
		"black_stained_glass_pane",
	],
	/* 161 */
	[
		"acacia_leaves",
		"dark_oak_leaves",
		DEF,
		DEF,
		"acacia_leaves",
		"dark_oak_leaves",
		DEF,
		DEF,
		"acacia_leaves",
		"dark_oak_leaves",
		DEF,
		DEF,
		"acacia_leaves",
		"dark_oak_leaves",
		DEF,
		DEF,
	],
	/* 162 */
	[
		"acacia_log",
		"dark_oak_log",
		DEF,
		DEF,
		"acacia_log",
		"dark_oak_log",
		DEF,
		DEF,
		"acacia_log",
		"dark_oak_log",
		DEF,
		DEF,
		"acacia_log",
		"dark_oak_log",
		DEF,
		DEF,
	],
	/* 163 */
	simple("acacia_stairs"),
	/* 164 */
	simple("dark_oak_stairs"),
	/* 165 */
	simple("slime_block"),
	/* 166 */
	simple("barrier"),
	/* 167 */
	simple("iron_trapdoor"),
	/* 168 */
	[
		"prismarine",
		"prismarine_bricks",
		"dark_prismarine",
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
	],
	/* 169 */
	simple("sea_lantern"),
	/* 170 */
	simple("hay_block"),
	/* 171 */
	[
		"white_carpet",
		"orange_carpet",
		"magenta_carpet",
		"light_blue_carpet",
		"yellow_carpet",
		"lime_carpet",
		"pink_carpet",
		"gray_carpet",
		"light_gray_carpet",
		"cyan_carpet",
		"purple_carpet",
		"blue_carpet",
		"brown_carpet",
		"green_carpet",
		"red_carpet",
		"black_carpet",
	],
	/* 172 */
	simple("terracotta"),
	/* 173 */
	simple("coal_block"),
	/* 174 */
	simple("packed_ice"),
	/* 175 */
	[
		"sunflower",
		"lilac",
		"tall_grass",
		"large_fern",
		"rose_bush",
		"peony",
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
	],
	/* 176 */
	EMPTY, // banner
	/* 177 */
	EMPTY, // wall banner
	/* 178 */
	simple("daylight_detector"),
	/* 179 */
	simple("red_sandstone"),
	/* 180 */
	simple("red_sandstone_stairs"),
	/* 181 */
	simple("red_sandstone_slab"),
	/* 182 */
	simple("red_sandstone_slab"),
	/* 183 */
	simple("spruce_fence_gate"),
	/* 184 */
	simple("birch_fence_gate"),
	/* 185 */
	simple("jungle_fence_gate"),
	/* 186 */
	simple("dark_oak_fence_gate"),
	/* 187 */
	simple("acacia_fence_gate"),
	/* 188 */
	simple("spruce_fence"),
	/* 189 */
	simple("birch_fence"),
	/* 190 */
	simple("jungle_fence"),
	/* 191 */
	simple("dark_oak_fence"),
	/* 192 */
	simple("acacia_fence"),
	/* 193 */
	simple("spruce_door"),
	/* 194 */
	simple("birch_door"),
	/* 195 */
	simple("jungle_door"),
	/* 196 */
	simple("acacia_door"),
	/* 197 */
	simple("dark_oak_door"),
	/* 198 */
	simple("end_rod"),
	/* 199 */
	simple("chorus_plant"),
	/* 200 */
	simple("chorus_flower"),
	/* 201 */
	simple("purpur_block"),
	/* 202 */
	simple("purpur_pillar"),
	/* 203 */
	simple("purpur_stairs"),
	/* 204 */
	simple("purpur_slab"),
	/* 205 */
	simple("purpur_slab"),
	/* 206 */
	simple("end_stone_bricks"),
	/* 207 */
	simple("beetroots"),
	/* 208 */
	simple("grass_path"),
	/* 209 */
	simple("end_gateway"),
	/* 210 */
	simple("repeating_command_block"),
	/* 211 */
	simple("chain_command_block"),
	/* 212 */
	simple("frosted_ice"),
	/* 213 */
	simple("magma_block"),
	/* 214 */
	simple("nether_wart_block"),
	/* 215 */
	simple("red_nether_bricks"),
	/* 216 */
	simple("bone_block"),
	/* 217 */
	simple("structure_void"),
	/* 218 */
	simple("observer"),
	/* 219 */
	simple("white_shulker_box"),
	/* 220 */
	simple("orange_shulker_box"),
	/* 221 */
	simple("magenta_shulker_box"),
	/* 222 */
	simple("light_blue_shulker_box"),
	/* 223 */
	simple("yellow_shulker_box"),
	/* 224 */
	simple("lime_shulker_box"),
	/* 225 */
	simple("pink_shulker_box"),
	/* 226 */
	simple("gray_shulker_box"),
	/* 227 */
	simple("light_gray_shulker_box"),
	/* 228 */
	simple("cyan_shulker_box"),
	/* 229 */
	simple("purple_shulker_box"),
	/* 230 */
	simple("blue_shulker_box"),
	/* 231 */
	simple("brown_shulker_box"),
	/* 232 */
	simple("green_shulker_box"),
	/* 233 */
	simple("red_shulker_box"),
	/* 234 */
	simple("black_shulker_box"),
	/* 235 */
	simple("white_glazed_terracotta"),
	/* 236 */
	simple("orange_glazed_terracotta"),
	/* 237 */
	simple("magenta_glazed_terracotta"),
	/* 238 */
	simple("light_blue_glazed_terracotta"),
	/* 239 */
	simple("yellow_glazed_terracotta"),
	/* 240 */
	simple("lime_glazed_terracotta"),
	/* 241 */
	simple("pink_glazed_terracotta"),
	/* 242 */
	simple("gray_glazed_terracotta"),
	/* 243 */
	simple("light_gray_glazed_terracotta"),
	/* 244 */
	simple("cyan_glazed_terracotta"),
	/* 245 */
	simple("purple_glazed_terracotta"),
	/* 246 */
	simple("blue_glazed_terracotta"),
	/* 247 */
	simple("brown_glazed_terracotta"),
	/* 248 */
	simple("green_glazed_terracotta"),
	/* 249 */
	simple("red_glazed_terracotta"),
	/* 250 */
	simple("black_glazed_terracotta"),
	/* 251 */
	[
		"white_concrete",
		"orange_concrete",
		"magenta_concrete",
		"light_blue_concrete",
		"yellow_concrete",
		"lime_concrete",
		"pink_concrete",
		"gray_concrete",
		"light_gray_concrete",
		"cyan_concrete",
		"purple_concrete",
		"blue_concrete",
		"brown_concrete",
		"green_concrete",
		"red_concrete",
		"black_concrete",
	],
	/* 252 */
	[
		"white_concrete_powder",
		"orange_concrete_powder",
		"magenta_concrete_powder",
		"light_blue_concrete_powder",
		"yellow_concrete_powder",
		"lime_concrete_powder",
		"pink_concrete_powder",
		"gray_concrete_powder",
		"light_gray_concrete_powder",
		"cyan_concrete_powder",
		"purple_concrete_powder",
		"blue_concrete_powder",
		"brown_concrete_powder",
		"green_concrete_powder",
		"red_concrete_powder",
		"black_concrete_powder",
	],
	/* 253 */
	EMPTY,
	/* 254 */
	EMPTY,
	/* 255 */
	simple("structure_block"),
];
