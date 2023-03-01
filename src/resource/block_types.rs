use enumflags2::make_bitflags;

use super::*;

pub const BLOCK_TYPES: &[(&str, BlockType)] = &[
	(
		"minecraft:acacia_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:acacia_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(167, 95, 60),
		},
	),
	(
		"minecraft:acacia_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(168, 90, 50),
		},
	),
	(
		"minecraft:acacia_fence_gate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(168, 90, 50),
		},
	),
	(
		"minecraft:acacia_leaves",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Foliage}),
			color: BlockColor(149, 148, 148),
		},
	),
	(
		"minecraft:acacia_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(150, 88, 55),
		},
	),
	(
		"minecraft:acacia_planks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(168, 90, 50),
		},
	),
	(
		"minecraft:acacia_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(168, 90, 50),
		},
	),
	(
		"minecraft:acacia_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(118, 117, 23),
		},
	),
	(
		"minecraft:acacia_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(168, 90, 50),
		},
	),
	(
		"minecraft:acacia_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(168, 90, 50),
		},
	),
	(
		"minecraft:acacia_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(168, 90, 50),
		},
	),
	(
		"minecraft:acacia_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(156, 87, 51),
		},
	),
	(
		"minecraft:acacia_wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:acacia_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(103, 96, 86),
		},
	),
	(
		"minecraft:activator_rail",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(115, 87, 74),
		},
	),
	(
		"minecraft:air",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:allium",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:amethyst_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(133, 97, 191),
		},
	),
	(
		"minecraft:amethyst_cluster",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(163, 126, 207),
		},
	),
	(
		"minecraft:ancient_debris",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(94, 66, 58),
		},
	),
	(
		"minecraft:andesite",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(136, 136, 136),
		},
	),
	(
		"minecraft:andesite_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(136, 136, 136),
		},
	),
	(
		"minecraft:andesite_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(136, 136, 136),
		},
	),
	(
		"minecraft:andesite_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(136, 136, 136),
		},
	),
	(
		"minecraft:anvil",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(72, 72, 72),
		},
	),
	(
		"minecraft:attached_melon_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(141, 142, 141),
		},
	),
	(
		"minecraft:attached_pumpkin_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(139, 139, 139),
		},
	),
	(
		"minecraft:azalea",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(101, 124, 47),
		},
	),
	(
		"minecraft:azalea_leaves",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(90, 114, 44),
		},
	),
	(
		"minecraft:azure_bluet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:bamboo",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(93, 144, 19),
		},
	),
	(
		"minecraft:bamboo_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:barrel",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(134, 100, 58),
		},
	),
	(
		"minecraft:barrier",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:basalt",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(80, 81, 86),
		},
	),
	(
		"minecraft:beacon",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 220, 215),
		},
	),
	(
		"minecraft:bedrock",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(85, 85, 85),
		},
	),
	(
		"minecraft:bee_nest",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(202, 160, 74),
		},
	),
	(
		"minecraft:beehive",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(180, 146, 90),
		},
	),
	(
		"minecraft:beetroots",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(93, 91, 30),
		},
	),
	(
		"minecraft:bell",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(253, 235, 110),
		},
	),
	(
		"minecraft:big_dripleaf",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(111, 141, 51),
		},
	),
	(
		"minecraft:big_dripleaf_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:birch_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:birch_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(220, 209, 176),
		},
	),
	(
		"minecraft:birch_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 175, 121),
		},
	),
	(
		"minecraft:birch_fence_gate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 175, 121),
		},
	),
	(
		"minecraft:birch_leaves",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Birch}),
			color: BlockColor(130, 129, 130),
		},
	),
	(
		"minecraft:birch_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(193, 179, 135),
		},
	),
	(
		"minecraft:birch_planks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 175, 121),
		},
	),
	(
		"minecraft:birch_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 175, 121),
		},
	),
	(
		"minecraft:birch_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 160, 79),
		},
	),
	(
		"minecraft:birch_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 175, 121),
		},
	),
	(
		"minecraft:birch_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 175, 121),
		},
	),
	(
		"minecraft:birch_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 175, 121),
		},
	),
	(
		"minecraft:birch_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(207, 194, 157),
		},
	),
	(
		"minecraft:birch_wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:birch_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(216, 215, 210),
		},
	),
	(
		"minecraft:black_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:black_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:black_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:black_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"minecraft:black_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(20, 21, 25),
		},
	),
	(
		"minecraft:black_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(8, 10, 15),
		},
	),
	(
		"minecraft:black_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(25, 26, 31),
		},
	),
	(
		"minecraft:black_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(67, 30, 32),
		},
	),
	(
		"minecraft:black_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(25, 25, 29),
		},
	),
	(
		"minecraft:black_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(25, 25, 25),
		},
	),
	(
		"minecraft:black_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(24, 24, 24),
		},
	),
	(
		"minecraft:black_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(37, 22, 16),
		},
	),
	(
		"minecraft:black_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:black_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(20, 21, 25),
		},
	),
	(
		"minecraft:blackstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(42, 36, 41),
		},
	),
	(
		"minecraft:blackstone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(42, 36, 41),
		},
	),
	(
		"minecraft:blackstone_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(42, 36, 41),
		},
	),
	(
		"minecraft:blackstone_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(42, 36, 41),
		},
	),
	(
		"minecraft:blast_furnace",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(80, 80, 81),
		},
	),
	(
		"minecraft:blue_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:blue_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:blue_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:blue_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"minecraft:blue_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(53, 57, 157),
		},
	),
	(
		"minecraft:blue_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 46, 143),
		},
	),
	(
		"minecraft:blue_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(70, 73, 166),
		},
	),
	(
		"minecraft:blue_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(47, 64, 139),
		},
	),
	(
		"minecraft:blue_ice",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(116, 167, 253),
		},
	),
	(
		"minecraft:blue_orchid",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:blue_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(43, 45, 140),
		},
	),
	(
		"minecraft:blue_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(51, 76, 178),
		},
	),
	(
		"minecraft:blue_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(48, 73, 171),
		},
	),
	(
		"minecraft:blue_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(74, 59, 91),
		},
	),
	(
		"minecraft:blue_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:blue_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(53, 57, 157),
		},
	),
	(
		"minecraft:bone_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(209, 206, 179),
		},
	),
	(
		"minecraft:bookshelf",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"minecraft:brain_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:brain_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(207, 91, 159),
		},
	),
	(
		"minecraft:brain_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:brain_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:brewing_stand",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(122, 100, 80),
		},
	),
	(
		"minecraft:brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(150, 97, 83),
		},
	),
	(
		"minecraft:brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(150, 97, 83),
		},
	),
	(
		"minecraft:brick_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(150, 97, 83),
		},
	),
	(
		"minecraft:bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(150, 97, 83),
		},
	),
	(
		"minecraft:brown_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:brown_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:brown_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:brown_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"minecraft:brown_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 71, 40),
		},
	),
	(
		"minecraft:brown_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(96, 59, 31),
		},
	),
	(
		"minecraft:brown_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(125, 84, 53),
		},
	),
	(
		"minecraft:brown_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(119, 106, 85),
		},
	),
	(
		"minecraft:brown_mushroom",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:brown_mushroom_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(149, 111, 81),
		},
	),
	(
		"minecraft:brown_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(106, 66, 35),
		},
	),
	(
		"minecraft:brown_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(102, 76, 51),
		},
	),
	(
		"minecraft:brown_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(97, 73, 48),
		},
	),
	(
		"minecraft:brown_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(77, 51, 35),
		},
	),
	(
		"minecraft:brown_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:brown_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 71, 40),
		},
	),
	(
		"minecraft:bubble_column",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Water}),
			color: BlockColor(177, 177, 177),
		},
	),
	(
		"minecraft:bubble_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:bubble_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(165, 26, 162),
		},
	),
	(
		"minecraft:bubble_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:bubble_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:budding_amethyst",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(132, 96, 186),
		},
	),
	(
		"minecraft:cactus",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(85, 127, 43),
		},
	),
	(
		"minecraft:cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"minecraft:calcite",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 224, 220),
		},
	),
	(
		"minecraft:campfire",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(110, 88, 54),
		},
	),
	(
		"minecraft:candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"minecraft:carrots",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(81, 124, 37),
		},
	),
	(
		"minecraft:cartography_table",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(103, 87, 67),
		},
	),
	(
		"minecraft:carved_pumpkin",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(198, 118, 24),
		},
	),
	(
		"minecraft:cauldron",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(73, 72, 74),
		},
	),
	(
		"minecraft:cave_air",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:cave_vines",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(90, 109, 40),
		},
	),
	(
		"minecraft:cave_vines_plant",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(88, 101, 38),
		},
	),
	(
		"minecraft:chain",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:chain_command_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(131, 161, 147),
		},
	),
	(
		"minecraft:chest",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"minecraft:chipped_anvil",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(72, 72, 72),
		},
	),
	(
		"minecraft:chiseled_deepslate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(54, 54, 54),
		},
	),
	(
		"minecraft:chiseled_nether_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(47, 23, 28),
		},
	),
	(
		"minecraft:chiseled_polished_blackstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(53, 48, 56),
		},
	),
	(
		"minecraft:chiseled_quartz_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(231, 226, 218),
		},
	),
	(
		"minecraft:chiseled_red_sandstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"minecraft:chiseled_sandstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"minecraft:chiseled_stone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(119, 118, 119),
		},
	),
	(
		"minecraft:chorus_flower",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(151, 120, 151),
		},
	),
	(
		"minecraft:chorus_plant",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(93, 57, 93),
		},
	),
	(
		"minecraft:clay",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 166, 179),
		},
	),
	(
		"minecraft:coal_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(16, 15, 15),
		},
	),
	(
		"minecraft:coal_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(105, 105, 105),
		},
	),
	(
		"minecraft:coarse_dirt",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(119, 85, 59),
		},
	),
	(
		"minecraft:cobbled_deepslate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(77, 77, 80),
		},
	),
	(
		"minecraft:cobbled_deepslate_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(77, 77, 80),
		},
	),
	(
		"minecraft:cobbled_deepslate_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(77, 77, 80),
		},
	),
	(
		"minecraft:cobbled_deepslate_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(77, 77, 80),
		},
	),
	(
		"minecraft:cobblestone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 127, 127),
		},
	),
	(
		"minecraft:cobblestone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 127, 127),
		},
	),
	(
		"minecraft:cobblestone_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 127, 127),
		},
	),
	(
		"minecraft:cobblestone_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 127, 127),
		},
	),
	(
		"minecraft:cobweb",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(228, 233, 234),
		},
	),
	(
		"minecraft:cocoa",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 91, 40),
		},
	),
	(
		"minecraft:command_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 136, 108),
		},
	),
	(
		"minecraft:comparator",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(166, 161, 159),
		},
	),
	(
		"minecraft:composter",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(88, 61, 23),
		},
	),
	(
		"minecraft:conduit",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(159, 139, 113),
		},
	),
	(
		"minecraft:copper_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 107, 79),
		},
	),
	(
		"minecraft:copper_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(124, 125, 120),
		},
	),
	(
		"minecraft:cornflower",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:cracked_deepslate_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(64, 64, 65),
		},
	),
	(
		"minecraft:cracked_deepslate_tiles",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(52, 52, 52),
		},
	),
	(
		"minecraft:cracked_nether_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(40, 20, 23),
		},
	),
	(
		"minecraft:cracked_polished_blackstone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 37, 43),
		},
	),
	(
		"minecraft:cracked_stone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(118, 117, 118),
		},
	),
	(
		"minecraft:crafting_table",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(119, 73, 42),
		},
	),
	(
		"minecraft:creeper_head",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:creeper_wall_head",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:crimson_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:crimson_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 54, 79),
		},
	),
	(
		"minecraft:crimson_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(101, 48, 70),
		},
	),
	(
		"minecraft:crimson_fence_gate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(101, 48, 70),
		},
	),
	(
		"minecraft:crimson_fungus",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:crimson_hyphae",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(92, 25, 29),
		},
	),
	(
		"minecraft:crimson_nylium",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(130, 31, 31),
		},
	),
	(
		"minecraft:crimson_planks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(101, 48, 70),
		},
	),
	(
		"minecraft:crimson_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(101, 48, 70),
		},
	),
	(
		"minecraft:crimson_roots",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(126, 8, 41),
		},
	),
	(
		"minecraft:crimson_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(101, 48, 70),
		},
	),
	(
		"minecraft:crimson_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(101, 48, 70),
		},
	),
	(
		"minecraft:crimson_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(101, 48, 70),
		},
	),
	(
		"minecraft:crimson_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(112, 49, 70),
		},
	),
	(
		"minecraft:crimson_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(103, 50, 72),
		},
	),
	(
		"minecraft:crimson_wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:crying_obsidian",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(32, 10, 60),
		},
	),
	(
		"minecraft:cut_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(191, 106, 80),
		},
	),
	(
		"minecraft:cut_copper_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(191, 106, 80),
		},
	),
	(
		"minecraft:cut_copper_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(191, 106, 80),
		},
	),
	(
		"minecraft:cut_red_sandstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"minecraft:cut_red_sandstone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"minecraft:cut_sandstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"minecraft:cut_sandstone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"minecraft:cyan_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:cyan_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:cyan_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:cyan_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"minecraft:cyan_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(21, 137, 145),
		},
	),
	(
		"minecraft:cyan_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(21, 119, 136),
		},
	),
	(
		"minecraft:cyan_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(36, 147, 157),
		},
	),
	(
		"minecraft:cyan_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(52, 118, 125),
		},
	),
	(
		"minecraft:cyan_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(20, 121, 135),
		},
	),
	(
		"minecraft:cyan_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(76, 127, 153),
		},
	),
	(
		"minecraft:cyan_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(73, 122, 147),
		},
	),
	(
		"minecraft:cyan_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(86, 91, 91),
		},
	),
	(
		"minecraft:cyan_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:cyan_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(21, 137, 145),
		},
	),
	(
		"minecraft:damaged_anvil",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(72, 72, 72),
		},
	),
	(
		"minecraft:dandelion",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dark_oak_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dark_oak_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(76, 51, 25),
		},
	),
	(
		"minecraft:dark_oak_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(66, 43, 20),
		},
	),
	(
		"minecraft:dark_oak_fence_gate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(66, 43, 20),
		},
	),
	(
		"minecraft:dark_oak_leaves",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Foliage}),
			color: BlockColor(150, 150, 150),
		},
	),
	(
		"minecraft:dark_oak_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(67, 45, 22),
		},
	),
	(
		"minecraft:dark_oak_planks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(66, 43, 20),
		},
	),
	(
		"minecraft:dark_oak_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(66, 43, 20),
		},
	),
	(
		"minecraft:dark_oak_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(61, 90, 30),
		},
	),
	(
		"minecraft:dark_oak_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(66, 43, 20),
		},
	),
	(
		"minecraft:dark_oak_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(66, 43, 20),
		},
	),
	(
		"minecraft:dark_oak_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(66, 43, 20),
		},
	),
	(
		"minecraft:dark_oak_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(75, 49, 23),
		},
	),
	(
		"minecraft:dark_oak_wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dark_oak_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(60, 46, 26),
		},
	),
	(
		"minecraft:dark_prismarine",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(51, 91, 75),
		},
	),
	(
		"minecraft:dark_prismarine_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(51, 91, 75),
		},
	),
	(
		"minecraft:dark_prismarine_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(51, 91, 75),
		},
	),
	(
		"minecraft:daylight_detector",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(130, 116, 94),
		},
	),
	(
		"minecraft:dead_brain_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dead_brain_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(124, 117, 114),
		},
	),
	(
		"minecraft:dead_brain_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dead_brain_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dead_bubble_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dead_bubble_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(131, 123, 119),
		},
	),
	(
		"minecraft:dead_bubble_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dead_bubble_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dead_bush",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(107, 78, 40),
		},
	),
	(
		"minecraft:dead_fire_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dead_fire_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(131, 123, 119),
		},
	),
	(
		"minecraft:dead_fire_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dead_fire_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dead_horn_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dead_horn_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(133, 126, 122),
		},
	),
	(
		"minecraft:dead_horn_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dead_horn_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dead_tube_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dead_tube_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(130, 123, 119),
		},
	),
	(
		"minecraft:dead_tube_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dead_tube_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:deepslate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(80, 80, 82),
		},
	),
	(
		"minecraft:deepslate_brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(70, 70, 71),
		},
	),
	(
		"minecraft:deepslate_brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(70, 70, 71),
		},
	),
	(
		"minecraft:deepslate_brick_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(70, 70, 71),
		},
	),
	(
		"minecraft:deepslate_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(70, 70, 71),
		},
	),
	(
		"minecraft:deepslate_coal_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(74, 74, 76),
		},
	),
	(
		"minecraft:deepslate_copper_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(92, 93, 89),
		},
	),
	(
		"minecraft:deepslate_diamond_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(83, 106, 106),
		},
	),
	(
		"minecraft:deepslate_emerald_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(78, 104, 87),
		},
	),
	(
		"minecraft:deepslate_gold_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(115, 102, 78),
		},
	),
	(
		"minecraft:deepslate_iron_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(106, 99, 94),
		},
	),
	(
		"minecraft:deepslate_lapis_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(79, 90, 115),
		},
	),
	(
		"minecraft:deepslate_redstone_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(104, 73, 74),
		},
	),
	(
		"minecraft:deepslate_tile_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(54, 54, 55),
		},
	),
	(
		"minecraft:deepslate_tile_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(54, 54, 55),
		},
	),
	(
		"minecraft:deepslate_tile_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(54, 54, 55),
		},
	),
	(
		"minecraft:deepslate_tiles",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(54, 54, 55),
		},
	),
	(
		"minecraft:detector_rail",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(123, 104, 90),
		},
	),
	(
		"minecraft:diamond_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(98, 237, 228),
		},
	),
	(
		"minecraft:diamond_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(121, 141, 140),
		},
	),
	(
		"minecraft:diorite",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(188, 188, 188),
		},
	),
	(
		"minecraft:diorite_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(188, 188, 188),
		},
	),
	(
		"minecraft:diorite_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(188, 188, 188),
		},
	),
	(
		"minecraft:diorite_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(188, 188, 188),
		},
	),
	(
		"minecraft:dirt",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(134, 96, 67),
		},
	),
	(
		"minecraft:dirt_path",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(148, 121, 65),
		},
	),
	(
		"minecraft:dispenser",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(110, 109, 109),
		},
	),
	(
		"minecraft:dragon_egg",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(12, 9, 15),
		},
	),
	(
		"minecraft:dragon_head",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dragon_wall_head",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:dried_kelp_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(50, 58, 38),
		},
	),
	(
		"minecraft:dripstone_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(134, 107, 92),
		},
	),
	(
		"minecraft:dropper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(110, 109, 109),
		},
	),
	(
		"minecraft:emerald_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(42, 203, 87),
		},
	),
	(
		"minecraft:emerald_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(108, 136, 115),
		},
	),
	(
		"minecraft:enchanting_table",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(128, 75, 85),
		},
	),
	(
		"minecraft:end_gateway",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(15, 10, 24),
		},
	),
	(
		"minecraft:end_portal",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(15, 10, 24),
		},
	),
	(
		"minecraft:end_portal_frame",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(91, 120, 97),
		},
	),
	(
		"minecraft:end_rod",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:end_stone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(219, 222, 158),
		},
	),
	(
		"minecraft:end_stone_brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(218, 224, 162),
		},
	),
	(
		"minecraft:end_stone_brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(218, 224, 162),
		},
	),
	(
		"minecraft:end_stone_brick_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(218, 224, 162),
		},
	),
	(
		"minecraft:end_stone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(218, 224, 162),
		},
	),
	(
		"minecraft:ender_chest",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(15, 10, 24),
		},
	),
	(
		"minecraft:exposed_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(161, 125, 103),
		},
	),
	(
		"minecraft:exposed_cut_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 121, 101),
		},
	),
	(
		"minecraft:exposed_cut_copper_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 121, 101),
		},
	),
	(
		"minecraft:exposed_cut_copper_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 121, 101),
		},
	),
	(
		"minecraft:farmland",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(81, 44, 15),
		},
	),
	(
		"minecraft:fern",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:fire",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(211, 140, 53),
		},
	),
	(
		"minecraft:fire_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:fire_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(163, 35, 46),
		},
	),
	(
		"minecraft:fire_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:fire_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:fletching_table",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(197, 180, 133),
		},
	),
	(
		"minecraft:flower_pot",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(124, 68, 53),
		},
	),
	(
		"minecraft:flowering_azalea",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(112, 121, 64),
		},
	),
	(
		"minecraft:flowering_azalea_leaves",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 111, 60),
		},
	),
	(
		"minecraft:frogspawn",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(105, 90, 82),
		},
	),
	(
		"minecraft:frosted_ice",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(140, 181, 252),
		},
	),
	(
		"minecraft:furnace",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(110, 109, 109),
		},
	),
	(
		"minecraft:gilded_blackstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(55, 42, 38),
		},
	),
	(
		"minecraft:glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(175, 213, 219),
		},
	),
	(
		"minecraft:glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(170, 210, 217),
		},
	),
	(
		"minecraft:glow_item_frame",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:glow_lichen",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:glowstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(171, 131, 84),
		},
	),
	(
		"minecraft:gold_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(246, 208, 61),
		},
	),
	(
		"minecraft:gold_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(145, 133, 106),
		},
	),
	(
		"minecraft:granite",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(149, 103, 85),
		},
	),
	(
		"minecraft:granite_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(149, 103, 85),
		},
	),
	(
		"minecraft:granite_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(149, 103, 85),
		},
	),
	(
		"minecraft:granite_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(149, 103, 85),
		},
	),
	(
		"minecraft:grass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:grass_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(147, 147, 147),
		},
	),
	(
		"minecraft:grass_path",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(148, 121, 65),
		},
	),
	(
		"minecraft:gravel",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(131, 127, 126),
		},
	),
	(
		"minecraft:gray_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:gray_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:gray_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:gray_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"minecraft:gray_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(62, 68, 71),
		},
	),
	(
		"minecraft:gray_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(54, 57, 61),
		},
	),
	(
		"minecraft:gray_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(76, 81, 84),
		},
	),
	(
		"minecraft:gray_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(83, 90, 93),
		},
	),
	(
		"minecraft:gray_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(55, 58, 62),
		},
	),
	(
		"minecraft:gray_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(76, 76, 76),
		},
	),
	(
		"minecraft:gray_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(73, 73, 73),
		},
	),
	(
		"minecraft:gray_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(57, 42, 35),
		},
	),
	(
		"minecraft:gray_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:gray_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(62, 68, 71),
		},
	),
	(
		"minecraft:green_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:green_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:green_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:green_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"minecraft:green_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(84, 109, 27),
		},
	),
	(
		"minecraft:green_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(73, 91, 36),
		},
	),
	(
		"minecraft:green_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(97, 119, 44),
		},
	),
	(
		"minecraft:green_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 142, 67),
		},
	),
	(
		"minecraft:green_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(79, 100, 31),
		},
	),
	(
		"minecraft:green_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(102, 127, 51),
		},
	),
	(
		"minecraft:green_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(97, 122, 48),
		},
	),
	(
		"minecraft:green_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(76, 83, 42),
		},
	),
	(
		"minecraft:green_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:green_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(84, 109, 27),
		},
	),
	(
		"minecraft:grindstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(142, 142, 142),
		},
	),
	(
		"minecraft:hanging_roots",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(161, 115, 91),
		},
	),
	(
		"minecraft:hay_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(165, 139, 12),
		},
	),
	(
		"minecraft:heavy_weighted_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(220, 220, 220),
		},
	),
	(
		"minecraft:honey_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(251, 185, 52),
		},
	),
	(
		"minecraft:honeycomb_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(229, 148, 29),
		},
	),
	(
		"minecraft:hopper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(75, 74, 75),
		},
	),
	(
		"minecraft:horn_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:horn_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(216, 199, 66),
		},
	),
	(
		"minecraft:horn_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:horn_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:ice",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(145, 183, 253),
		},
	),
	(
		"minecraft:infested_chiseled_stone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(119, 118, 119),
		},
	),
	(
		"minecraft:infested_cobblestone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 127, 127),
		},
	),
	(
		"minecraft:infested_cracked_stone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(118, 117, 118),
		},
	),
	(
		"minecraft:infested_deepslate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(80, 80, 82),
		},
	),
	(
		"minecraft:infested_mossy_stone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(115, 121, 105),
		},
	),
	(
		"minecraft:infested_stone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(125, 125, 125),
		},
	),
	(
		"minecraft:infested_stone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(122, 121, 122),
		},
	),
	(
		"minecraft:iron_bars",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(136, 139, 135),
		},
	),
	(
		"minecraft:iron_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(220, 220, 220),
		},
	),
	(
		"minecraft:iron_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(194, 193, 193),
		},
	),
	(
		"minecraft:iron_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(136, 129, 122),
		},
	),
	(
		"minecraft:iron_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(202, 202, 202),
		},
	),
	(
		"minecraft:item_frame",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:jack_o_lantern",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(214, 152, 52),
		},
	),
	(
		"minecraft:jigsaw",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(80, 69, 81),
		},
	),
	(
		"minecraft:jukebox",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(93, 64, 47),
		},
	),
	(
		"minecraft:jungle_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:jungle_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(163, 119, 84),
		},
	),
	(
		"minecraft:jungle_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 115, 80),
		},
	),
	(
		"minecraft:jungle_fence_gate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 115, 80),
		},
	),
	(
		"minecraft:jungle_leaves",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Foliage}),
			color: BlockColor(156, 154, 143),
		},
	),
	(
		"minecraft:jungle_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(149, 109, 70),
		},
	),
	(
		"minecraft:jungle_planks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 115, 80),
		},
	),
	(
		"minecraft:jungle_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 115, 80),
		},
	),
	(
		"minecraft:jungle_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(47, 81, 16),
		},
	),
	(
		"minecraft:jungle_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 115, 80),
		},
	),
	(
		"minecraft:jungle_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 115, 80),
		},
	),
	(
		"minecraft:jungle_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 115, 80),
		},
	),
	(
		"minecraft:jungle_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(152, 110, 77),
		},
	),
	(
		"minecraft:jungle_wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:jungle_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(85, 67, 25),
		},
	),
	(
		"minecraft:kelp",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:kelp_plant",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(86, 130, 42),
		},
	),
	(
		"minecraft:ladder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:lantern",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(106, 91, 83),
		},
	),
	(
		"minecraft:lapis_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(30, 67, 140),
		},
	),
	(
		"minecraft:lapis_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(107, 117, 141),
		},
	),
	(
		"minecraft:large_amethyst_bud",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:large_fern",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(125, 125, 125),
		},
	),
	(
		"minecraft:lava",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(212, 90, 18),
		},
	),
	(
		"minecraft:lava_cauldron",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(73, 72, 74),
		},
	),
	(
		"minecraft:lectern",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(173, 137, 83),
		},
	),
	(
		"minecraft:lever",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:light",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:light_blue_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:light_blue_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:light_blue_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:light_blue_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"minecraft:light_blue_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(58, 175, 217),
		},
	),
	(
		"minecraft:light_blue_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(35, 137, 198),
		},
	),
	(
		"minecraft:light_blue_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(74, 180, 213),
		},
	),
	(
		"minecraft:light_blue_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(94, 164, 208),
		},
	),
	(
		"minecraft:light_blue_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(49, 163, 212),
		},
	),
	(
		"minecraft:light_blue_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(102, 153, 216),
		},
	),
	(
		"minecraft:light_blue_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(97, 147, 208),
		},
	),
	(
		"minecraft:light_blue_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(113, 108, 137),
		},
	),
	(
		"minecraft:light_blue_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:light_blue_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(58, 175, 217),
		},
	),
	(
		"minecraft:light_gray_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:light_gray_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:light_gray_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:light_gray_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"minecraft:light_gray_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(142, 142, 134),
		},
	),
	(
		"minecraft:light_gray_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(125, 125, 115),
		},
	),
	(
		"minecraft:light_gray_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 154, 148),
		},
	),
	(
		"minecraft:light_gray_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(144, 166, 167),
		},
	),
	(
		"minecraft:light_gray_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(124, 124, 115),
		},
	),
	(
		"minecraft:light_gray_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(153, 153, 153),
		},
	),
	(
		"minecraft:light_gray_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(147, 147, 147),
		},
	),
	(
		"minecraft:light_gray_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(135, 106, 97),
		},
	),
	(
		"minecraft:light_gray_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:light_gray_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(142, 142, 134),
		},
	),
	(
		"minecraft:light_weighted_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(246, 208, 61),
		},
	),
	(
		"minecraft:lightning_rod",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:lilac",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 125, 147),
		},
	),
	(
		"minecraft:lily_of_the_valley",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:lily_pad",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(133, 133, 133),
		},
	),
	(
		"minecraft:lime_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:lime_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:lime_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:lime_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"minecraft:lime_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(112, 185, 25),
		},
	),
	(
		"minecraft:lime_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(94, 168, 24),
		},
	),
	(
		"minecraft:lime_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(125, 189, 41),
		},
	),
	(
		"minecraft:lime_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 197, 55),
		},
	),
	(
		"minecraft:lime_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 172, 23),
		},
	),
	(
		"minecraft:lime_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 204, 25),
		},
	),
	(
		"minecraft:lime_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(122, 196, 24),
		},
	),
	(
		"minecraft:lime_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(103, 117, 52),
		},
	),
	(
		"minecraft:lime_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:lime_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(112, 185, 25),
		},
	),
	(
		"minecraft:lodestone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(147, 149, 152),
		},
	),
	(
		"minecraft:loom",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(142, 119, 91),
		},
	),
	(
		"minecraft:magenta_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:magenta_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:magenta_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:magenta_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"minecraft:magenta_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(189, 68, 179),
		},
	),
	(
		"minecraft:magenta_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(169, 48, 159),
		},
	),
	(
		"minecraft:magenta_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 83, 184),
		},
	),
	(
		"minecraft:magenta_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(208, 100, 191),
		},
	),
	(
		"minecraft:magenta_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(173, 54, 163),
		},
	),
	(
		"minecraft:magenta_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(178, 76, 216),
		},
	),
	(
		"minecraft:magenta_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(171, 73, 208),
		},
	),
	(
		"minecraft:magenta_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(149, 88, 108),
		},
	),
	(
		"minecraft:magenta_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:magenta_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(189, 68, 179),
		},
	),
	(
		"minecraft:magma_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(142, 63, 31),
		},
	),
	(
		"minecraft:mangrove_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:mangrove_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(112, 48, 46),
		},
	),
	(
		"minecraft:mangrove_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 54, 48),
		},
	),
	(
		"minecraft:mangrove_fence_gate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 54, 48),
		},
	),
	(
		"minecraft:mangrove_leaves",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Foliage}),
			color: BlockColor(129, 128, 128),
		},
	),
	(
		"minecraft:mangrove_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(102, 48, 42),
		},
	),
	(
		"minecraft:mangrove_planks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 54, 48),
		},
	),
	(
		"minecraft:mangrove_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 54, 48),
		},
	),
	(
		"minecraft:mangrove_propagule",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(96, 174, 83),
		},
	),
	(
		"minecraft:mangrove_roots",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(74, 59, 38),
		},
	),
	(
		"minecraft:mangrove_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 54, 48),
		},
	),
	(
		"minecraft:mangrove_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 54, 48),
		},
	),
	(
		"minecraft:mangrove_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 54, 48),
		},
	),
	(
		"minecraft:mangrove_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(110, 46, 42),
		},
	),
	(
		"minecraft:mangrove_wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:mangrove_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(83, 66, 41),
		},
	),
	(
		"minecraft:medium_amethyst_bud",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:melon",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(111, 144, 30),
		},
	),
	(
		"minecraft:melon_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(153, 153, 153),
		},
	),
	(
		"minecraft:moss_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(89, 109, 45),
		},
	),
	(
		"minecraft:moss_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(89, 109, 45),
		},
	),
	(
		"minecraft:mossy_cobblestone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(110, 118, 94),
		},
	),
	(
		"minecraft:mossy_cobblestone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(110, 118, 94),
		},
	),
	(
		"minecraft:mossy_cobblestone_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(110, 118, 94),
		},
	),
	(
		"minecraft:mossy_cobblestone_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(110, 118, 94),
		},
	),
	(
		"minecraft:mossy_stone_brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(115, 121, 105),
		},
	),
	(
		"minecraft:mossy_stone_brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(115, 121, 105),
		},
	),
	(
		"minecraft:mossy_stone_brick_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(115, 121, 105),
		},
	),
	(
		"minecraft:mossy_stone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(115, 121, 105),
		},
	),
	(
		"minecraft:moving_piston",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:mud",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(60, 57, 60),
		},
	),
	(
		"minecraft:mud_brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(137, 103, 79),
		},
	),
	(
		"minecraft:mud_brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(137, 103, 79),
		},
	),
	(
		"minecraft:mud_brick_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(137, 103, 79),
		},
	),
	(
		"minecraft:mud_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(137, 103, 79),
		},
	),
	(
		"minecraft:muddy_mangrove_roots",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(70, 58, 45),
		},
	),
	(
		"minecraft:mushroom_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(203, 196, 185),
		},
	),
	(
		"minecraft:mycelium",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(111, 98, 101),
		},
	),
	(
		"minecraft:nether_brick_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 21, 26),
		},
	),
	(
		"minecraft:nether_brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 21, 26),
		},
	),
	(
		"minecraft:nether_brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 21, 26),
		},
	),
	(
		"minecraft:nether_brick_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 21, 26),
		},
	),
	(
		"minecraft:nether_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 21, 26),
		},
	),
	(
		"minecraft:nether_gold_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(115, 54, 42),
		},
	),
	(
		"minecraft:nether_portal",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(89, 11, 192),
		},
	),
	(
		"minecraft:nether_quartz_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 65, 62),
		},
	),
	(
		"minecraft:nether_sprouts",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(19, 151, 133),
		},
	),
	(
		"minecraft:nether_wart",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(111, 18, 19),
		},
	),
	(
		"minecraft:nether_wart_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 2, 2),
		},
	),
	(
		"minecraft:netherite_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(66, 61, 63),
		},
	),
	(
		"minecraft:netherrack",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(97, 38, 38),
		},
	),
	(
		"minecraft:note_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(88, 58, 40),
		},
	),
	(
		"minecraft:oak_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:oak_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(140, 110, 66),
		},
	),
	(
		"minecraft:oak_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"minecraft:oak_fence_gate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"minecraft:oak_leaves",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Foliage}),
			color: BlockColor(144, 144, 144),
		},
	),
	(
		"minecraft:oak_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(151, 121, 73),
		},
	),
	(
		"minecraft:oak_planks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"minecraft:oak_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"minecraft:oak_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(77, 106, 40),
		},
	),
	(
		"minecraft:oak_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"minecraft:oak_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"minecraft:oak_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"minecraft:oak_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(124, 99, 56),
		},
	),
	(
		"minecraft:oak_wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:oak_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 85, 50),
		},
	),
	(
		"minecraft:observer",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(98, 98, 98),
		},
	),
	(
		"minecraft:obsidian",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(15, 10, 24),
		},
	),
	(
		"minecraft:ochre_froglight",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(250, 245, 206),
		},
	),
	(
		"minecraft:orange_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:orange_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:orange_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:orange_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"minecraft:orange_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(240, 118, 19),
		},
	),
	(
		"minecraft:orange_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(224, 97, 0),
		},
	),
	(
		"minecraft:orange_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(227, 131, 31),
		},
	),
	(
		"minecraft:orange_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 147, 91),
		},
	),
	(
		"minecraft:orange_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(234, 106, 8),
		},
	),
	(
		"minecraft:orange_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(216, 127, 51),
		},
	),
	(
		"minecraft:orange_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(208, 122, 48),
		},
	),
	(
		"minecraft:orange_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(161, 83, 37),
		},
	),
	(
		"minecraft:orange_tulip",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:orange_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:orange_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(240, 118, 19),
		},
	),
	(
		"minecraft:oxeye_daisy",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:oxidized_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(82, 162, 132),
		},
	),
	(
		"minecraft:oxidized_cut_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(79, 153, 126),
		},
	),
	(
		"minecraft:oxidized_cut_copper_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(79, 153, 126),
		},
	),
	(
		"minecraft:oxidized_cut_copper_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(79, 153, 126),
		},
	),
	(
		"minecraft:packed_ice",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(141, 180, 250),
		},
	),
	(
		"minecraft:packed_mud",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(142, 106, 79),
		},
	),
	(
		"minecraft:pearlescent_froglight",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(245, 240, 239),
		},
	),
	(
		"minecraft:peony",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(129, 126, 139),
		},
	),
	(
		"minecraft:petrified_oak_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"minecraft:pink_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:pink_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:pink_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:pink_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"minecraft:pink_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(237, 141, 172),
		},
	),
	(
		"minecraft:pink_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(213, 101, 142),
		},
	),
	(
		"minecraft:pink_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(228, 153, 181),
		},
	),
	(
		"minecraft:pink_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(235, 154, 181),
		},
	),
	(
		"minecraft:pink_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(230, 121, 157),
		},
	),
	(
		"minecraft:pink_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(242, 127, 165),
		},
	),
	(
		"minecraft:pink_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(233, 122, 159),
		},
	),
	(
		"minecraft:pink_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(161, 78, 78),
		},
	),
	(
		"minecraft:pink_tulip",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:pink_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:pink_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(237, 141, 172),
		},
	),
	(
		"minecraft:piston",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 104, 96),
		},
	),
	(
		"minecraft:piston_head",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(153, 127, 85),
		},
	),
	(
		"minecraft:player_head",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:player_wall_head",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:podzol",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(91, 63, 24),
		},
	),
	(
		"minecraft:pointed_dripstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(129, 102, 89),
		},
	),
	(
		"minecraft:polished_andesite",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(132, 134, 133),
		},
	),
	(
		"minecraft:polished_andesite_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(132, 134, 133),
		},
	),
	(
		"minecraft:polished_andesite_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(132, 134, 133),
		},
	),
	(
		"minecraft:polished_basalt",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 98, 100),
		},
	),
	(
		"minecraft:polished_blackstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(53, 48, 56),
		},
	),
	(
		"minecraft:polished_blackstone_brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(48, 42, 49),
		},
	),
	(
		"minecraft:polished_blackstone_brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(48, 42, 49),
		},
	),
	(
		"minecraft:polished_blackstone_brick_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(48, 42, 49),
		},
	),
	(
		"minecraft:polished_blackstone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(48, 42, 49),
		},
	),
	(
		"minecraft:polished_blackstone_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:polished_blackstone_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(53, 48, 56),
		},
	),
	(
		"minecraft:polished_blackstone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(53, 48, 56),
		},
	),
	(
		"minecraft:polished_blackstone_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(53, 48, 56),
		},
	),
	(
		"minecraft:polished_blackstone_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(53, 48, 56),
		},
	),
	(
		"minecraft:polished_deepslate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(72, 72, 73),
		},
	),
	(
		"minecraft:polished_deepslate_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(72, 72, 73),
		},
	),
	(
		"minecraft:polished_deepslate_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(72, 72, 73),
		},
	),
	(
		"minecraft:polished_deepslate_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(72, 72, 73),
		},
	),
	(
		"minecraft:polished_diorite",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 193, 194),
		},
	),
	(
		"minecraft:polished_diorite_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 193, 194),
		},
	),
	(
		"minecraft:polished_diorite_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 193, 194),
		},
	),
	(
		"minecraft:polished_granite",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 106, 89),
		},
	),
	(
		"minecraft:polished_granite_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 106, 89),
		},
	),
	(
		"minecraft:polished_granite_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 106, 89),
		},
	),
	(
		"minecraft:poppy",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:potatoes",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(84, 135, 47),
		},
	),
	(
		"minecraft:potted_acacia_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(118, 117, 23),
		},
	),
	(
		"minecraft:potted_allium",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(158, 137, 183),
		},
	),
	(
		"minecraft:potted_azalea_bush",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(101, 124, 47),
		},
	),
	(
		"minecraft:potted_azure_bluet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(169, 204, 127),
		},
	),
	(
		"minecraft:potted_bamboo",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(93, 144, 19),
		},
	),
	(
		"minecraft:potted_birch_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 160, 79),
		},
	),
	(
		"minecraft:potted_blue_orchid",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(47, 162, 168),
		},
	),
	(
		"minecraft:potted_brown_mushroom",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(153, 116, 92),
		},
	),
	(
		"minecraft:potted_cactus",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(85, 127, 43),
		},
	),
	(
		"minecraft:potted_cornflower",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(79, 121, 146),
		},
	),
	(
		"minecraft:potted_crimson_fungus",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(141, 44, 29),
		},
	),
	(
		"minecraft:potted_crimson_roots",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 8, 41),
		},
	),
	(
		"minecraft:potted_dandelion",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(147, 172, 43),
		},
	),
	(
		"minecraft:potted_dark_oak_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(61, 90, 30),
		},
	),
	(
		"minecraft:potted_dead_bush",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(107, 78, 40),
		},
	),
	(
		"minecraft:potted_fern",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(124, 124, 124),
		},
	),
	(
		"minecraft:potted_flowering_azalea_bush",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(112, 121, 64),
		},
	),
	(
		"minecraft:potted_jungle_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(47, 81, 16),
		},
	),
	(
		"minecraft:potted_lily_of_the_valley",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(123, 174, 95),
		},
	),
	(
		"minecraft:potted_mangrove_propagule",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(96, 174, 83),
		},
	),
	(
		"minecraft:potted_oak_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(77, 106, 40),
		},
	),
	(
		"minecraft:potted_orange_tulip",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(93, 142, 30),
		},
	),
	(
		"minecraft:potted_oxeye_daisy",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(179, 202, 143),
		},
	),
	(
		"minecraft:potted_pink_tulip",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 157, 78),
		},
	),
	(
		"minecraft:potted_poppy",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(128, 64, 37),
		},
	),
	(
		"minecraft:potted_red_mushroom",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(216, 75, 67),
		},
	),
	(
		"minecraft:potted_red_tulip",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(89, 128, 32),
		},
	),
	(
		"minecraft:potted_spruce_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 60, 36),
		},
	),
	(
		"minecraft:potted_warped_fungus",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(74, 109, 87),
		},
	),
	(
		"minecraft:potted_warped_roots",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(20, 136, 123),
		},
	),
	(
		"minecraft:potted_white_tulip",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(93, 164, 71),
		},
	),
	(
		"minecraft:potted_wither_rose",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(41, 44, 23),
		},
	),
	(
		"minecraft:powder_snow",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 253, 253),
		},
	),
	(
		"minecraft:powder_snow_cauldron",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(73, 72, 74),
		},
	),
	(
		"minecraft:powered_rail",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(137, 109, 74),
		},
	),
	(
		"minecraft:prismarine",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 156, 151),
		},
	),
	(
		"minecraft:prismarine_brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 171, 158),
		},
	),
	(
		"minecraft:prismarine_brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 171, 158),
		},
	),
	(
		"minecraft:prismarine_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 171, 158),
		},
	),
	(
		"minecraft:prismarine_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 156, 151),
		},
	),
	(
		"minecraft:prismarine_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 156, 151),
		},
	),
	(
		"minecraft:prismarine_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 156, 151),
		},
	),
	(
		"minecraft:pumpkin",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(198, 118, 24),
		},
	),
	(
		"minecraft:pumpkin_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(154, 154, 154),
		},
	),
	(
		"minecraft:purple_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:purple_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:purple_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:purple_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"minecraft:purple_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(121, 42, 172),
		},
	),
	(
		"minecraft:purple_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(100, 31, 156),
		},
	),
	(
		"minecraft:purple_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(131, 55, 177),
		},
	),
	(
		"minecraft:purple_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 48, 152),
		},
	),
	(
		"minecraft:purple_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(103, 32, 156),
		},
	),
	(
		"minecraft:purple_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 63, 178),
		},
	),
	(
		"minecraft:purple_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(122, 61, 171),
		},
	),
	(
		"minecraft:purple_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(118, 70, 86),
		},
	),
	(
		"minecraft:purple_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:purple_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(121, 42, 172),
		},
	),
	(
		"minecraft:purpur_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(169, 125, 169),
		},
	),
	(
		"minecraft:purpur_pillar",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(171, 129, 171),
		},
	),
	(
		"minecraft:purpur_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(169, 125, 169),
		},
	),
	(
		"minecraft:purpur_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(169, 125, 169),
		},
	),
	(
		"minecraft:quartz_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(235, 229, 222),
		},
	),
	(
		"minecraft:quartz_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(234, 229, 221),
		},
	),
	(
		"minecraft:quartz_pillar",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(235, 230, 224),
		},
	),
	(
		"minecraft:quartz_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(235, 229, 222),
		},
	),
	(
		"minecraft:quartz_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(235, 229, 222),
		},
	),
	(
		"minecraft:rail",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(125, 111, 88),
		},
	),
	(
		"minecraft:raw_copper_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 105, 79),
		},
	),
	(
		"minecraft:raw_gold_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(221, 169, 46),
		},
	),
	(
		"minecraft:raw_iron_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(166, 135, 107),
		},
	),
	(
		"minecraft:red_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:red_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:red_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:red_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"minecraft:red_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 39, 34),
		},
	),
	(
		"minecraft:red_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(142, 32, 32),
		},
	),
	(
		"minecraft:red_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(168, 54, 50),
		},
	),
	(
		"minecraft:red_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 59, 53),
		},
	),
	(
		"minecraft:red_mushroom",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:red_mushroom_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(200, 46, 45),
		},
	),
	(
		"minecraft:red_nether_brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(69, 7, 9),
		},
	),
	(
		"minecraft:red_nether_brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(69, 7, 9),
		},
	),
	(
		"minecraft:red_nether_brick_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(69, 7, 9),
		},
	),
	(
		"minecraft:red_nether_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(69, 7, 9),
		},
	),
	(
		"minecraft:red_sand",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(190, 102, 33),
		},
	),
	(
		"minecraft:red_sandstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"minecraft:red_sandstone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"minecraft:red_sandstone_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"minecraft:red_sandstone_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"minecraft:red_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(140, 31, 30),
		},
	),
	(
		"minecraft:red_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(153, 51, 51),
		},
	),
	(
		"minecraft:red_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(147, 48, 48),
		},
	),
	(
		"minecraft:red_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(143, 61, 46),
		},
	),
	(
		"minecraft:red_tulip",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:red_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:red_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 39, 34),
		},
	),
	(
		"minecraft:redstone_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(175, 24, 5),
		},
	),
	(
		"minecraft:redstone_lamp",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(95, 54, 30),
		},
	),
	(
		"minecraft:redstone_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(140, 109, 109),
		},
	),
	(
		"minecraft:redstone_torch",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:redstone_wall_torch",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:redstone_wire",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(175, 24, 5),
		},
	),
	(
		"minecraft:reinforced_deepslate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(80, 82, 78),
		},
	),
	(
		"minecraft:repeater",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 157, 156),
		},
	),
	(
		"minecraft:repeating_command_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(129, 111, 176),
		},
	),
	(
		"minecraft:respawn_anchor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(75, 26, 144),
		},
	),
	(
		"minecraft:rooted_dirt",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(144, 103, 76),
		},
	),
	(
		"minecraft:rose_bush",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(131, 66, 37),
		},
	),
	(
		"minecraft:sand",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(219, 207, 163),
		},
	),
	(
		"minecraft:sandstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"minecraft:sandstone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"minecraft:sandstone_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"minecraft:sandstone_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"minecraft:scaffolding",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(174, 134, 80),
		},
	),
	(
		"minecraft:sculk",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(12, 29, 36),
		},
	),
	(
		"minecraft:sculk_catalyst",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(15, 31, 38),
		},
	),
	(
		"minecraft:sculk_sensor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(7, 70, 84),
		},
	),
	(
		"minecraft:sculk_shrieker",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(198, 205, 169),
		},
	),
	(
		"minecraft:sculk_vein",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(7, 48, 57),
		},
	),
	(
		"minecraft:sea_lantern",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(172, 199, 190),
		},
	),
	(
		"minecraft:sea_pickle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(90, 97, 39),
		},
	),
	(
		"minecraft:seagrass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:shroomlight",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(240, 146, 70),
		},
	),
	(
		"minecraft:shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(139, 96, 139),
		},
	),
	(
		"minecraft:sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"minecraft:skeleton_skull",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:skeleton_wall_skull",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:slime_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(111, 192, 91),
		},
	),
	(
		"minecraft:small_amethyst_bud",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:small_dripleaf",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:smithing_table",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(57, 58, 70),
		},
	),
	(
		"minecraft:smoker",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(85, 83, 81),
		},
	),
	(
		"minecraft:smooth_basalt",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(72, 72, 78),
		},
	),
	(
		"minecraft:smooth_quartz",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(235, 229, 222),
		},
	),
	(
		"minecraft:smooth_quartz_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(235, 229, 222),
		},
	),
	(
		"minecraft:smooth_quartz_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(235, 229, 222),
		},
	),
	(
		"minecraft:smooth_red_sandstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"minecraft:smooth_red_sandstone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"minecraft:smooth_red_sandstone_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"minecraft:smooth_sandstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"minecraft:smooth_sandstone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"minecraft:smooth_sandstone_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"minecraft:smooth_stone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(158, 158, 158),
		},
	),
	(
		"minecraft:smooth_stone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(158, 158, 158),
		},
	),
	(
		"minecraft:snow",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(249, 254, 254),
		},
	),
	(
		"minecraft:snow_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(249, 254, 254),
		},
	),
	(
		"minecraft:soul_campfire",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(80, 204, 208),
		},
	),
	(
		"minecraft:soul_fire",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(51, 192, 197),
		},
	),
	(
		"minecraft:soul_lantern",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(71, 99, 114),
		},
	),
	(
		"minecraft:soul_sand",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(81, 62, 50),
		},
	),
	(
		"minecraft:soul_soil",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(75, 57, 46),
		},
	),
	(
		"minecraft:soul_torch",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:soul_wall_torch",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:spawner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(36, 46, 62),
		},
	),
	(
		"minecraft:sponge",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(195, 192, 74),
		},
	),
	(
		"minecraft:spore_blossom",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(206, 96, 158),
		},
	),
	(
		"minecraft:spruce_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:spruce_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(106, 80, 48),
		},
	),
	(
		"minecraft:spruce_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 84, 48),
		},
	),
	(
		"minecraft:spruce_fence_gate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 84, 48),
		},
	),
	(
		"minecraft:spruce_leaves",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Spruce}),
			color: BlockColor(126, 126, 126),
		},
	),
	(
		"minecraft:spruce_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(108, 80, 46),
		},
	),
	(
		"minecraft:spruce_planks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 84, 48),
		},
	),
	(
		"minecraft:spruce_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 84, 48),
		},
	),
	(
		"minecraft:spruce_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 60, 36),
		},
	),
	(
		"minecraft:spruce_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 84, 48),
		},
	),
	(
		"minecraft:spruce_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 84, 48),
		},
	),
	(
		"minecraft:spruce_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 84, 48),
		},
	),
	(
		"minecraft:spruce_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(103, 79, 47),
		},
	),
	(
		"minecraft:spruce_wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:spruce_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(58, 37, 16),
		},
	),
	(
		"minecraft:sticky_piston",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 104, 96),
		},
	),
	(
		"minecraft:stone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(125, 125, 125),
		},
	),
	(
		"minecraft:stone_brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(122, 121, 122),
		},
	),
	(
		"minecraft:stone_brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(122, 121, 122),
		},
	),
	(
		"minecraft:stone_brick_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(122, 121, 122),
		},
	),
	(
		"minecraft:stone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(122, 121, 122),
		},
	),
	(
		"minecraft:stone_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:stone_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(125, 125, 125),
		},
	),
	(
		"minecraft:stone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(125, 125, 125),
		},
	),
	(
		"minecraft:stone_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(125, 125, 125),
		},
	),
	(
		"minecraft:stonecutter",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(123, 118, 111),
		},
	),
	(
		"minecraft:stripped_acacia_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(166, 91, 51),
		},
	),
	(
		"minecraft:stripped_acacia_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(174, 92, 59),
		},
	),
	(
		"minecraft:stripped_birch_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(191, 171, 116),
		},
	),
	(
		"minecraft:stripped_birch_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(196, 176, 118),
		},
	),
	(
		"minecraft:stripped_crimson_hyphae",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(137, 57, 90),
		},
	),
	(
		"minecraft:stripped_crimson_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(121, 56, 82),
		},
	),
	(
		"minecraft:stripped_dark_oak_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(65, 44, 22),
		},
	),
	(
		"minecraft:stripped_dark_oak_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(72, 56, 36),
		},
	),
	(
		"minecraft:stripped_jungle_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(165, 122, 81),
		},
	),
	(
		"minecraft:stripped_jungle_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(171, 132, 84),
		},
	),
	(
		"minecraft:stripped_mangrove_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 43, 43),
		},
	),
	(
		"minecraft:stripped_mangrove_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(119, 54, 47),
		},
	),
	(
		"minecraft:stripped_oak_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 129, 77),
		},
	),
	(
		"minecraft:stripped_oak_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(177, 144, 86),
		},
	),
	(
		"minecraft:stripped_spruce_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(105, 80, 46),
		},
	),
	(
		"minecraft:stripped_spruce_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(115, 89, 52),
		},
	),
	(
		"minecraft:stripped_warped_hyphae",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(57, 150, 147),
		},
	),
	(
		"minecraft:stripped_warped_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(52, 128, 124),
		},
	),
	(
		"minecraft:structure_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(88, 74, 90),
		},
	),
	(
		"minecraft:structure_void",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:sugar_cane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(148, 192, 101),
		},
	),
	(
		"minecraft:sunflower",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(246, 196, 54),
		},
	),
	(
		"minecraft:sweet_berry_bush",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(68, 77, 50),
		},
	),
	(
		"minecraft:tall_grass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(151, 149, 151),
		},
	),
	(
		"minecraft:tall_seagrass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(59, 139, 14),
		},
	),
	(
		"minecraft:target",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(226, 170, 157),
		},
	),
	(
		"minecraft:terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(152, 94, 67),
		},
	),
	(
		"minecraft:tinted_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 38, 46),
		},
	),
	(
		"minecraft:tnt",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(142, 62, 53),
		},
	),
	(
		"minecraft:torch",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:trapped_chest",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"minecraft:tripwire",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:tripwire_hook",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:tube_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:tube_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(49, 87, 206),
		},
	),
	(
		"minecraft:tube_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:tube_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:tuff",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(108, 109, 102),
		},
	),
	(
		"minecraft:turtle_egg",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(228, 226, 191),
		},
	),
	(
		"minecraft:twisting_vines",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(20, 143, 124),
		},
	),
	(
		"minecraft:twisting_vines_plant",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(20, 135, 122),
		},
	),
	(
		"minecraft:verdant_froglight",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(229, 244, 228),
		},
	),
	(
		"minecraft:vine",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(116, 116, 116),
		},
	),
	(
		"minecraft:void_air",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:wall_torch",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:warped_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:warped_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 126, 120),
		},
	),
	(
		"minecraft:warped_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(43, 104, 99),
		},
	),
	(
		"minecraft:warped_fence_gate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(43, 104, 99),
		},
	),
	(
		"minecraft:warped_fungus",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:warped_hyphae",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(58, 58, 77),
		},
	),
	(
		"minecraft:warped_nylium",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(43, 114, 101),
		},
	),
	(
		"minecraft:warped_planks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(43, 104, 99),
		},
	),
	(
		"minecraft:warped_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(43, 104, 99),
		},
	),
	(
		"minecraft:warped_roots",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(20, 138, 124),
		},
	),
	(
		"minecraft:warped_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(43, 104, 99),
		},
	),
	(
		"minecraft:warped_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(43, 104, 99),
		},
	),
	(
		"minecraft:warped_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(43, 104, 99),
		},
	),
	(
		"minecraft:warped_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(53, 109, 110),
		},
	),
	(
		"minecraft:warped_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(47, 119, 111),
		},
	),
	(
		"minecraft:warped_wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:warped_wart_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(22, 119, 121),
		},
	),
	(
		"minecraft:water",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Water}),
			color: BlockColor(177, 177, 177),
		},
	),
	(
		"minecraft:water_cauldron",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(73, 72, 74),
		},
	),
	(
		"minecraft:waxed_copper_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 107, 79),
		},
	),
	(
		"minecraft:waxed_cut_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(191, 106, 80),
		},
	),
	(
		"minecraft:waxed_cut_copper_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(191, 106, 80),
		},
	),
	(
		"minecraft:waxed_cut_copper_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(191, 106, 80),
		},
	),
	(
		"minecraft:waxed_exposed_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(161, 125, 103),
		},
	),
	(
		"minecraft:waxed_exposed_cut_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 121, 101),
		},
	),
	(
		"minecraft:waxed_exposed_cut_copper_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 121, 101),
		},
	),
	(
		"minecraft:waxed_exposed_cut_copper_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 121, 101),
		},
	),
	(
		"minecraft:waxed_oxidized_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(82, 162, 132),
		},
	),
	(
		"minecraft:waxed_oxidized_cut_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(79, 153, 126),
		},
	),
	(
		"minecraft:waxed_oxidized_cut_copper_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(79, 153, 126),
		},
	),
	(
		"minecraft:waxed_oxidized_cut_copper_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(79, 153, 126),
		},
	),
	(
		"minecraft:waxed_weathered_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(108, 153, 110),
		},
	),
	(
		"minecraft:waxed_weathered_cut_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 145, 107),
		},
	),
	(
		"minecraft:waxed_weathered_cut_copper_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 145, 107),
		},
	),
	(
		"minecraft:waxed_weathered_cut_copper_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 145, 107),
		},
	),
	(
		"minecraft:weathered_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(108, 153, 110),
		},
	),
	(
		"minecraft:weathered_cut_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 145, 107),
		},
	),
	(
		"minecraft:weathered_cut_copper_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 145, 107),
		},
	),
	(
		"minecraft:weathered_cut_copper_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 145, 107),
		},
	),
	(
		"minecraft:weeping_vines",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(104, 1, 0),
		},
	),
	(
		"minecraft:weeping_vines_plant",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(132, 16, 12),
		},
	),
	(
		"minecraft:wet_sponge",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(171, 181, 70),
		},
	),
	(
		"minecraft:wheat",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(166, 151, 73),
		},
	),
	(
		"minecraft:white_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:white_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:white_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:white_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"minecraft:white_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(233, 236, 236),
		},
	),
	(
		"minecraft:white_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(207, 213, 214),
		},
	),
	(
		"minecraft:white_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(225, 227, 227),
		},
	),
	(
		"minecraft:white_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(188, 212, 202),
		},
	),
	(
		"minecraft:white_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(215, 220, 221),
		},
	),
	(
		"minecraft:white_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(255, 255, 255),
		},
	),
	(
		"minecraft:white_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(246, 246, 246),
		},
	),
	(
		"minecraft:white_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(209, 178, 161),
		},
	),
	(
		"minecraft:white_tulip",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:white_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:white_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(233, 236, 236),
		},
	),
	(
		"minecraft:wither_rose",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:wither_skeleton_skull",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:wither_skeleton_wall_skull",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:yellow_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:yellow_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:yellow_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:yellow_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"minecraft:yellow_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 197, 39),
		},
	),
	(
		"minecraft:yellow_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(240, 175, 21),
		},
	),
	(
		"minecraft:yellow_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(232, 199, 54),
		},
	),
	(
		"minecraft:yellow_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(234, 192, 88),
		},
	),
	(
		"minecraft:yellow_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 188, 29),
		},
	),
	(
		"minecraft:yellow_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(229, 229, 51),
		},
	),
	(
		"minecraft:yellow_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(221, 221, 48),
		},
	),
	(
		"minecraft:yellow_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(186, 133, 35),
		},
	),
	(
		"minecraft:yellow_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:yellow_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 197, 39),
		},
	),
	(
		"minecraft:zombie_head",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"minecraft:zombie_wall_head",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
];
