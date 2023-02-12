const fn simple(id: &str) -> [&str; 16] {
	[
		id, id, id, id, id, id, id, id, id, id, id, id, id, id, id, id,
	]
}

const DEF: &str = "minecraft:air";
const EMPTY: [&str; 16] = simple(DEF);

pub const LEGACY_BLOCK_TYPES: [[&str; 16]; 256] = [
	/* 0 */
	simple("minecraft:air"),
	/* 1 */
	[
		"minecraft:stone",
		"minecraft:granite",
		"minecraft:polished_granite",
		"minecraft:diorite",
		"minecraft:polished_diorite",
		"minecraft:andesite",
		"minecraft:polished_andesite",
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
	simple("minecraft:grass_block"),
	/* 3 */
	[
		"minecraft:dirt",
		"minecraft:coarse_dirt",
		"minecraft:podzol",
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
	simple("minecraft:cobblestone"),
	/* 5 */
	[
		"minecraft:oak_planks",
		"minecraft:spruce_planks",
		"minecraft:birch_planks",
		"minecraft:jungle_planks",
		"minecraft:acacia_planks",
		"minecraft:dark_oak_planks",
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
		"minecraft:oak_sapling",
		"minecraft:spruce_sapling",
		"minecraft:birch_sapling",
		"minecraft:jungle_sapling",
		"minecraft:acacia_sapling",
		"minecraft:dark_oak_sapling",
		DEF,
		DEF,
		"minecraft:oak_sapling",
		"minecraft:spruce_sapling",
		"minecraft:birch_sapling",
		"minecraft:jungle_sapling",
		"minecraft:acacia_sapling",
		"minecraft:dark_oak_sapling",
		DEF,
		DEF,
	],
	/* 7 */
	simple("minecraft:bedrock"),
	/* 8 */
	simple("minecraft:water"),
	/* 9 */
	simple("minecraft:water"),
	/* 10 */
	simple("minecraft:lava"),
	/* 11 */
	simple("minecraft:lava"),
	/* 12 */
	[
		"minecraft:sand",
		"minecraft:red_sand",
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
	/* 13 */
	simple("minecraft:gravel"),
	/* 14 */
	simple("minecraft:gold_ore"),
	/* 15 */
	simple("minecraft:iron_ore"),
	/* 16 */
	simple("minecraft:coal_ore"),
	/* 17 */
	[
		"minecraft:oak_log",
		"minecraft:spruce_log",
		"minecraft:birch_log",
		"minecraft:jungle_log",
		"minecraft:oak_log",
		"minecraft:spruce_log",
		"minecraft:birch_log",
		"minecraft:jungle_log",
		"minecraft:oak_log",
		"minecraft:spruce_log",
		"minecraft:birch_log",
		"minecraft:jungle_log",
		"minecraft:oak_log",
		"minecraft:spruce_log",
		"minecraft:birch_log",
		"minecraft:jungle_log",
	],
	/* 18 */
	[
		"minecraft:oak_leaves",
		"minecraft:spruce_leaves",
		"minecraft:birch_leaves",
		"minecraft:jungle_leaves",
		"minecraft:oak_leaves",
		"minecraft:spruce_leaves",
		"minecraft:birch_leaves",
		"minecraft:jungle_leaves",
		"minecraft:oak_leaves",
		"minecraft:spruce_leaves",
		"minecraft:birch_leaves",
		"minecraft:jungle_leaves",
		"minecraft:oak_leaves",
		"minecraft:spruce_leaves",
		"minecraft:birch_leaves",
		"minecraft:jungle_leaves",
	],
	/* 19 */
	simple("minecraft:sponge"),
	/* 20 */
	simple("minecraft:glass"),
	/* 21 */
	simple("minecraft:lapis_ore"),
	/* 22 */
	simple("minecraft:lapis_block"),
	/* 23 */
	simple("minecraft:dispenser"),
	/* 24 */
	simple("minecraft:sandstone"),
	/* 25 */
	simple("minecraft:note_block"),
	/* 26 */
	EMPTY, // bed
	/* 27 */
	simple("minecraft:powered_rail"),
	/* 28 */
	simple("minecraft:detector_rail"),
	/* 29 */
	simple("minecraft:sticky_piston"),
	/* 30 */
	simple("minecraft:cobweb"),
	/* 31 */
	[
		"minecraft:grass",
		"minecraft:fern",
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
	/* 32 */
	simple("minecraft:dead_bush"),
	/* 33 */
	simple("minecraft:piston"),
	/* 34 */
	simple("minecraft:piston_head"),
	/* 35 */
	[
		"minecraft:white_wool",
		"minecraft:orange_wool",
		"minecraft:magenta_wool",
		"minecraft:light_blue_wool",
		"minecraft:yellow_wool",
		"minecraft:lime_wool",
		"minecraft:pink_wool",
		"minecraft:gray_wool",
		"minecraft:light_gray_wool",
		"minecraft:cyan_wool",
		"minecraft:purple_wool",
		"minecraft:blue_wool",
		"minecraft:brown_wool",
		"minecraft:green_wool",
		"minecraft:red_wool",
		"minecraft:black_wool",
	],
	/* 36 */
	simple("minecraft:moving_piston"),
	/* 37 */
	simple("minecraft:dandelion"),
	/* 38 */
	[
		"minecraft:poppy",
		"minecraft:blue_orchid",
		"minecraft:allium",
		"minecraft:azure_bluet",
		"minecraft:red_tulip",
		"minecraft:orange_tulip",
		"minecraft:white_tulip",
		"minecraft:pink_tulip",
		"minecraft:oxeye_daisy",
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
		DEF,
	],
	/* 39 */
	simple("minecraft:brown_mushroom"),
	/* 40 */
	simple("minecraft:red_mushroom"),
	/* 41 */
	simple("minecraft:gold_block"),
	/* 42 */
	simple("minecraft:iron_block"),
	/* 43 */
	[
		"minecraft:smooth_stone_slab",
		"minecraft:sandstone_slab",
		"minecraft:oak_slab",
		"minecraft:cobblestone_slab",
		"minecraft:brick_slab",
		"minecraft:stone_brick_slab",
		"minecraft:nether_brick_slab",
		"minecraft:quartz_slab",
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
		"minecraft:smooth_stone_slab",
		"minecraft:sandstone_slab",
		"minecraft:oak_slab",
		"minecraft:cobblestone_slab",
		"minecraft:brick_slab",
		"minecraft:stone_brick_slab",
		"minecraft:nether_brick_slab",
		"minecraft:quartz_slab",
		"minecraft:stone_slab",
		"minecraft:sandstone_slab",
		"minecraft:oak_slab",
		"minecraft:cobblestone_slab",
		"minecraft:brick_slab",
		"minecraft:stone_brick_slab",
		"minecraft:nether_brick_slab",
		"minecraft:quartz_slab",
	],
	/* 45 */
	simple("minecraft:bricks"),
	/* 46 */
	simple("minecraft:tnt"),
	/* 47 */
	simple("minecraft:bookshelf"),
	/* 48 */
	simple("minecraft:mossy_cobblestone"),
	/* 49 */
	simple("minecraft:obsidian"),
	/* 50 */
	[
		DEF,
		"minecraft:wall_torch",
		"minecraft:wall_torch",
		"minecraft:wall_torch",
		"minecraft:wall_torch",
		"minecraft:torch",
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
	simple("minecraft:fire"),
	/* 52 */
	simple("minecraft:spawner"),
	/* 53 */
	simple("minecraft:oak_stairs"),
	/* 54 */
	simple("minecraft:chest"),
	/* 55 */
	simple("minecraft:redstone_wire"),
	/* 56 */
	simple("minecraft:diamond_ore"),
	/* 57 */
	simple("minecraft:diamond_block"),
	/* 58 */
	simple("minecraft:crafting_table"),
	/* 59 */
	simple("minecraft:wheat"),
	/* 60 */
	simple("minecraft:farmland"),
	/* 61 */
	simple("minecraft:furnace"),
	/* 62 */
	simple("minecraft:furnace"),
	/* 63 */
	simple("minecraft:sign"),
	/* 64 */
	simple("minecraft:oak_door"),
	/* 65 */
	simple("minecraft:ladder"),
	/* 66 */
	simple("minecraft:rail"),
	/* 67 */
	simple("minecraft:cobblestone_stairs"),
	/* 68 */
	simple("minecraft:wall_sign"),
	/* 69 */
	simple("minecraft:lever"),
	/* 70 */
	simple("minecraft:stone_pressure_plate"),
	/* 71 */
	simple("minecraft:iron_door"),
	/* 72 */
	simple("minecraft:oak_pressure_plate"),
	/* 73 */
	simple("minecraft:redstone_ore"),
	/* 74 */
	simple("minecraft:redstone_ore"),
	/* 75 */
	[
		DEF,
		"minecraft:redstone_wall_torch",
		"minecraft:redstone_wall_torch",
		"minecraft:redstone_wall_torch",
		"minecraft:redstone_wall_torch",
		"minecraft:redstone_torch",
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
		"minecraft:redstone_wall_torch",
		"minecraft:redstone_wall_torch",
		"minecraft:redstone_wall_torch",
		"minecraft:redstone_wall_torch",
		"minecraft:redstone_torch",
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
	simple("minecraft:stone_button"),
	/* 78 */
	simple("minecraft:snow"),
	/* 79 */
	simple("minecraft:ice"),
	/* 80 */
	simple("minecraft:snow_block"),
	/* 81 */
	simple("minecraft:cactus"),
	/* 82 */
	simple("minecraft:clay"),
	/* 83 */
	simple("minecraft:sugar_cane"),
	/* 84 */
	simple("minecraft:jukebox"),
	/* 85 */
	simple("minecraft:oak_fence"),
	/* 86 */
	simple("minecraft:pumpkin"),
	/* 87 */
	simple("minecraft:netherrack"),
	/* 88 */
	simple("minecraft:soul_sand"),
	/* 89 */
	simple("minecraft:glowstone"),
	/* 90 */
	simple("minecraft:nether_portal"),
	/* 91 */
	simple("minecraft:pumpkin"),
	/* 92 */
	simple("minecraft:cake"),
	/* 93 */
	simple("minecraft:repeater"),
	/* 94 */
	simple("minecraft:repeater"),
	/* 95 */
	[
		"minecraft:white_stained_glass",
		"minecraft:orange_stained_glass",
		"minecraft:magenta_stained_glass",
		"minecraft:light_blue_stained_glass",
		"minecraft:yellow_stained_glass",
		"minecraft:lime_stained_glass",
		"minecraft:pink_stained_glass",
		"minecraft:gray_stained_glass",
		"minecraft:light_gray_stained_glass",
		"minecraft:cyan_stained_glass",
		"minecraft:purple_stained_glass",
		"minecraft:blue_stained_glass",
		"minecraft:brown_stained_glass",
		"minecraft:green_stained_glass",
		"minecraft:red_stained_glass",
		"minecraft:black_stained_glass",
	],
	/* 96 */
	simple("minecraft:oak_trapdoor"),
	/* 97 */
	[
		"minecraft:infested_stone",
		"minecraft:infested_cobblestone",
		"minecraft:infested_stone_bricks",
		"minecraft:infested_mossy_stone_bricks",
		"minecraft:infested_cracked_stone_bricks",
		"minecraft:infested_chiseled_stone_bricks",
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
		"minecraft:stone_bricks",
		"minecraft:mossy_stone_bricks",
		"minecraft:cracked_stone_bricks",
		"minecraft:chiseled_stone_bricks",
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
	simple("minecraft:brown_mushroom_block"),
	/* 100 */
	simple("minecraft:red_mushroom_block"),
	/* 101 */
	simple("minecraft:iron_bars"),
	/* 102 */
	simple("minecraft:glass_pane"),
	/* 103 */
	simple("minecraft:melon"),
	/* 104 */
	simple("minecraft:pumpkin_stem"),
	/* 105 */
	simple("minecraft:melon_stem"),
	/* 106 */
	simple("minecraft:vine"),
	/* 107 */
	simple("minecraft:oak_fence_gate"),
	/* 108 */
	simple("minecraft:brick_stairs"),
	/* 109 */
	simple("minecraft:stone_brick_stairs"),
	/* 110 */
	simple("minecraft:mycelium"),
	/* 111 */
	simple("minecraft:lily_pad"),
	/* 112 */
	simple("minecraft:nether_bricks"),
	/* 113 */
	simple("minecraft:nether_brick_fence"),
	/* 114 */
	simple("minecraft:nether_brick_stairs"),
	/* 115 */
	simple("minecraft:nether_wart"),
	/* 116 */
	simple("minecraft:enchanting_table"),
	/* 117 */
	simple("minecraft:brewing_stand"),
	/* 118 */
	simple("minecraft:cauldron"),
	/* 119 */
	simple("minecraft:end_portal"),
	/* 120 */
	simple("minecraft:end_portal_frame"),
	/* 121 */
	simple("minecraft:end_stone"),
	/* 122 */
	simple("minecraft:dragon_egg"),
	/* 123 */
	simple("minecraft:redstone_lamp"),
	/* 124 */
	simple("minecraft:redstone_lamp"),
	/* 125 */
	[
		"minecraft:oak_slab",
		"minecraft:spruce_slab",
		"minecraft:birch_slab",
		"minecraft:jungle_slab",
		"minecraft:acacia_slab",
		"minecraft:dark_oak_slab",
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
		"minecraft:oak_slab",
		"minecraft:spruce_slab",
		"minecraft:birch_slab",
		"minecraft:jungle_slab",
		"minecraft:acacia_slab",
		"minecraft:dark_oak_slab",
		DEF,
		DEF,
		"minecraft:oak_slab",
		"minecraft:spruce_slab",
		"minecraft:birch_slab",
		"minecraft:jungle_slab",
		"minecraft:acacia_slab",
		"minecraft:dark_oak_slab",
		DEF,
		DEF,
	],
	/* 127 */
	simple("minecraft:cocoa"),
	/* 128 */
	simple("minecraft:sandstone_stairs"),
	/* 129 */
	simple("minecraft:emerald_ore"),
	/* 130 */
	simple("minecraft:ender_chest"),
	/* 131 */
	simple("minecraft:tripwire_hook"),
	/* 132 */
	simple("minecraft:tripwire"),
	/* 133 */
	simple("minecraft:emerald_block"),
	/* 134 */
	simple("minecraft:spruce_stairs"),
	/* 135 */
	simple("minecraft:birch_stairs"),
	/* 136 */
	simple("minecraft:jungle_stairs"),
	/* 137 */
	simple("minecraft:command_block"),
	/* 138 */
	simple("minecraft:beacon"),
	/* 139 */
	[
		"minecraft:cobblestone_wall",
		"minecraft:mossy_cobblestone_wall",
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
	simple("minecraft:flower_pot"),
	/* 141 */
	simple("minecraft:carrots"),
	/* 142 */
	simple("minecraft:potatoes"),
	/* 143 */
	EMPTY,
	/* 144 */
	EMPTY,
	/* 145 */
	[
		"minecraft:anvil",
		"minecraft:anvil",
		"minecraft:anvil",
		"minecraft:anvil",
		"minecraft:chipped_anvil",
		"minecraft:chipped_anvil",
		"minecraft:chipped_anvil",
		"minecraft:chipped_anvil",
		"minecraft:damaged_anvil",
		"minecraft:damaged_anvil",
		"minecraft:damaged_anvil",
		"minecraft:damaged_anvil",
		DEF,
		DEF,
		DEF,
		DEF,
	],
	/* 146 */
	simple("minecraft:trapped_chest"),
	/* 147 */
	simple("minecraft:light_weighted_pressure_plate"),
	/* 148 */
	simple("minecraft:heavy_weighted_pressure_plate"),
	/* 149 */
	simple("minecraft:comparator"),
	/* 150 */
	simple("minecraft:comparator"),
	/* 151 */
	simple("minecraft:daylight_detector"),
	/* 152 */
	simple("minecraft:redstone_block"),
	/* 153 */
	simple("minecraft:nether_quartz_ore"),
	/* 154 */
	simple("minecraft:hopper"),
	/* 155 */
	simple("minecraft:quartz_block"),
	/* 156 */
	simple("minecraft:quartz_stairs"),
	/* 157 */
	simple("minecraft:activator_rail"),
	/* 158 */
	simple("minecraft:dropper"),
	/* 159 */
	[
		"minecraft:white_terracotta",
		"minecraft:orange_terracotta",
		"minecraft:magenta_terracotta",
		"minecraft:light_blue_terracotta",
		"minecraft:yellow_terracotta",
		"minecraft:lime_terracotta",
		"minecraft:pink_terracotta",
		"minecraft:gray_terracotta",
		"minecraft:light_gray_terracotta",
		"minecraft:cyan_terracotta",
		"minecraft:purple_terracotta",
		"minecraft:blue_terracotta",
		"minecraft:brown_terracotta",
		"minecraft:green_terracotta",
		"minecraft:red_terracotta",
		"minecraft:black_terracotta",
	],
	/* 160 */
	[
		"minecraft:white_stained_glass_pane",
		"minecraft:orange_stained_glass_pane",
		"minecraft:magenta_stained_glass_pane",
		"minecraft:light_blue_stained_glass_pane",
		"minecraft:yellow_stained_glass_pane",
		"minecraft:lime_stained_glass_pane",
		"minecraft:pink_stained_glass_pane",
		"minecraft:gray_stained_glass_pane",
		"minecraft:light_gray_stained_glass_pane",
		"minecraft:cyan_stained_glass_pane",
		"minecraft:purple_stained_glass_pane",
		"minecraft:blue_stained_glass_pane",
		"minecraft:brown_stained_glass_pane",
		"minecraft:green_stained_glass_pane",
		"minecraft:red_stained_glass_pane",
		"minecraft:black_stained_glass_pane",
	],
	/* 161 */
	[
		"minecraft:acacia_leaves",
		"minecraft:dark_oak_leaves",
		DEF,
		DEF,
		"minecraft:acacia_leaves",
		"minecraft:dark_oak_leaves",
		DEF,
		DEF,
		"minecraft:acacia_leaves",
		"minecraft:dark_oak_leaves",
		DEF,
		DEF,
		"minecraft:acacia_leaves",
		"minecraft:dark_oak_leaves",
		DEF,
		DEF,
	],
	/* 162 */
	[
		"minecraft:acacia_log",
		"minecraft:dark_oak_log",
		DEF,
		DEF,
		"minecraft:acacia_log",
		"minecraft:dark_oak_log",
		DEF,
		DEF,
		"minecraft:acacia_log",
		"minecraft:dark_oak_log",
		DEF,
		DEF,
		"minecraft:acacia_log",
		"minecraft:dark_oak_log",
		DEF,
		DEF,
	],
	/* 163 */
	simple("minecraft:acacia_stairs"),
	/* 164 */
	simple("minecraft:dark_oak_stairs"),
	/* 165 */
	simple("minecraft:slime_block"),
	/* 166 */
	simple("minecraft:barrier"),
	/* 167 */
	simple("minecraft:iron_trapdoor"),
	/* 168 */
	[
		"minecraft:prismarine",
		"minecraft:prismarine_bricks",
		"minecraft:dark_prismarine",
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
	simple("minecraft:sea_lantern"),
	/* 170 */
	simple("minecraft:hay_block"),
	/* 171 */
	[
		"minecraft:white_carpet",
		"minecraft:orange_carpet",
		"minecraft:magenta_carpet",
		"minecraft:light_blue_carpet",
		"minecraft:yellow_carpet",
		"minecraft:lime_carpet",
		"minecraft:pink_carpet",
		"minecraft:gray_carpet",
		"minecraft:light_gray_carpet",
		"minecraft:cyan_carpet",
		"minecraft:purple_carpet",
		"minecraft:blue_carpet",
		"minecraft:brown_carpet",
		"minecraft:green_carpet",
		"minecraft:red_carpet",
		"minecraft:black_carpet",
	],
	/* 172 */
	simple("minecraft:terracotta"),
	/* 173 */
	simple("minecraft:coal_block"),
	/* 174 */
	simple("minecraft:packed_ice"),
	/* 175 */
	[
		"minecraft:sunflower",
		"minecraft:lilac",
		"minecraft:tall_grass",
		"minecraft:large_fern",
		"minecraft:rose_bush",
		"minecraft:peony",
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
	simple("minecraft:daylight_detector"),
	/* 179 */
	simple("minecraft:red_sandstone"),
	/* 180 */
	simple("minecraft:red_sandstone_stairs"),
	/* 181 */
	simple("minecraft:red_sandstone_slab"),
	/* 182 */
	simple("minecraft:red_sandstone_slab"),
	/* 183 */
	simple("minecraft:spruce_fence_gate"),
	/* 184 */
	simple("minecraft:birch_fence_gate"),
	/* 185 */
	simple("minecraft:jungle_fence_gate"),
	/* 186 */
	simple("minecraft:dark_oak_fence_gate"),
	/* 187 */
	simple("minecraft:acacia_fence_gate"),
	/* 188 */
	simple("minecraft:spruce_fence"),
	/* 189 */
	simple("minecraft:birch_fence"),
	/* 190 */
	simple("minecraft:jungle_fence"),
	/* 191 */
	simple("minecraft:dark_oak_fence"),
	/* 192 */
	simple("minecraft:acacia_fence"),
	/* 193 */
	simple("minecraft:spruce_door"),
	/* 194 */
	simple("minecraft:birch_door"),
	/* 195 */
	simple("minecraft:jungle_door"),
	/* 196 */
	simple("minecraft:acacia_door"),
	/* 197 */
	simple("minecraft:dark_oak_door"),
	/* 198 */
	simple("minecraft:end_rod"),
	/* 199 */
	simple("minecraft:chorus_plant"),
	/* 200 */
	simple("minecraft:chorus_flower"),
	/* 201 */
	simple("minecraft:purpur_block"),
	/* 202 */
	simple("minecraft:purpur_pillar"),
	/* 203 */
	simple("minecraft:purpur_stairs"),
	/* 204 */
	simple("minecraft:purpur_slab"),
	/* 205 */
	simple("minecraft:purpur_slab"),
	/* 206 */
	simple("minecraft:end_stone_bricks"),
	/* 207 */
	simple("minecraft:beetroots"),
	/* 208 */
	simple("minecraft:grass_path"),
	/* 209 */
	simple("minecraft:end_gateway"),
	/* 210 */
	simple("minecraft:repeating_command_block"),
	/* 211 */
	simple("minecraft:chain_command_block"),
	/* 212 */
	simple("minecraft:frosted_ice"),
	/* 213 */
	simple("minecraft:magma_block"),
	/* 214 */
	simple("minecraft:nether_wart_block"),
	/* 215 */
	simple("minecraft:red_nether_bricks"),
	/* 216 */
	simple("minecraft:bone_block"),
	/* 217 */
	simple("minecraft:structure_void"),
	/* 218 */
	simple("minecraft:observer"),
	/* 219 */
	simple("minecraft:white_shulker_box"),
	/* 220 */
	simple("minecraft:orange_shulker_box"),
	/* 221 */
	simple("minecraft:magenta_shulker_box"),
	/* 222 */
	simple("minecraft:light_blue_shulker_box"),
	/* 223 */
	simple("minecraft:yellow_shulker_box"),
	/* 224 */
	simple("minecraft:lime_shulker_box"),
	/* 225 */
	simple("minecraft:pink_shulker_box"),
	/* 226 */
	simple("minecraft:gray_shulker_box"),
	/* 227 */
	simple("minecraft:light_gray_shulker_box"),
	/* 228 */
	simple("minecraft:cyan_shulker_box"),
	/* 229 */
	simple("minecraft:purple_shulker_box"),
	/* 230 */
	simple("minecraft:blue_shulker_box"),
	/* 231 */
	simple("minecraft:brown_shulker_box"),
	/* 232 */
	simple("minecraft:green_shulker_box"),
	/* 233 */
	simple("minecraft:red_shulker_box"),
	/* 234 */
	simple("minecraft:black_shulker_box"),
	/* 235 */
	simple("minecraft:white_glazed_terracotta"),
	/* 236 */
	simple("minecraft:orange_glazed_terracotta"),
	/* 237 */
	simple("minecraft:magenta_glazed_terracotta"),
	/* 238 */
	simple("minecraft:light_blue_glazed_terracotta"),
	/* 239 */
	simple("minecraft:yellow_glazed_terracotta"),
	/* 240 */
	simple("minecraft:lime_glazed_terracotta"),
	/* 241 */
	simple("minecraft:pink_glazed_terracotta"),
	/* 242 */
	simple("minecraft:gray_glazed_terracotta"),
	/* 243 */
	simple("minecraft:light_gray_glazed_terracotta"),
	/* 244 */
	simple("minecraft:cyan_glazed_terracotta"),
	/* 245 */
	simple("minecraft:purple_glazed_terracotta"),
	/* 246 */
	simple("minecraft:blue_glazed_terracotta"),
	/* 247 */
	simple("minecraft:brown_glazed_terracotta"),
	/* 248 */
	simple("minecraft:green_glazed_terracotta"),
	/* 249 */
	simple("minecraft:red_glazed_terracotta"),
	/* 250 */
	simple("minecraft:black_glazed_terracotta"),
	/* 251 */
	[
		"minecraft:white_concrete",
		"minecraft:orange_concrete",
		"minecraft:magenta_concrete",
		"minecraft:light_blue_concrete",
		"minecraft:yellow_concrete",
		"minecraft:lime_concrete",
		"minecraft:pink_concrete",
		"minecraft:gray_concrete",
		"minecraft:light_gray_concrete",
		"minecraft:cyan_concrete",
		"minecraft:purple_concrete",
		"minecraft:blue_concrete",
		"minecraft:brown_concrete",
		"minecraft:green_concrete",
		"minecraft:red_concrete",
		"minecraft:black_concrete",
	],
	/* 252 */
	[
		"minecraft:white_concrete_powder",
		"minecraft:orange_concrete_powder",
		"minecraft:magenta_concrete_powder",
		"minecraft:light_blue_concrete_powder",
		"minecraft:yellow_concrete_powder",
		"minecraft:lime_concrete_powder",
		"minecraft:pink_concrete_powder",
		"minecraft:gray_concrete_powder",
		"minecraft:light_gray_concrete_powder",
		"minecraft:cyan_concrete_powder",
		"minecraft:purple_concrete_powder",
		"minecraft:blue_concrete_powder",
		"minecraft:brown_concrete_powder",
		"minecraft:green_concrete_powder",
		"minecraft:red_concrete_powder",
		"minecraft:black_concrete_powder",
	],
	/* 253 */
	EMPTY,
	/* 254 */
	EMPTY,
	/* 255 */
	simple("minecraft:structure_block"),
];
