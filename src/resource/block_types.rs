use enumflags2::make_bitflags;

use super::*;

pub const BLOCK_TYPES: &[(&str, BlockType)] = &[
	(
		"acacia_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"acacia_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(167, 95, 60),
		},
	),
	(
		"acacia_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(168, 90, 50),
		},
	),
	(
		"acacia_fence_gate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(168, 90, 50),
		},
	),
	(
		"acacia_leaves",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Foliage}),
			color: BlockColor(149, 148, 148),
		},
	),
	(
		"acacia_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(150, 88, 55),
		},
	),
	(
		"acacia_planks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(168, 90, 50),
		},
	),
	(
		"acacia_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(168, 90, 50),
		},
	),
	(
		"acacia_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(118, 117, 23),
		},
	),
	(
		"acacia_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(168, 90, 50),
		},
	),
	(
		"acacia_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(168, 90, 50),
		},
	),
	(
		"acacia_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(168, 90, 50),
		},
	),
	(
		"acacia_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(156, 87, 51),
		},
	),
	(
		"acacia_wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"acacia_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(103, 96, 86),
		},
	),
	(
		"activator_rail",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(115, 87, 74),
		},
	),
	(
		"air",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"allium",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"amethyst_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(133, 97, 191),
		},
	),
	(
		"amethyst_cluster",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(163, 126, 207),
		},
	),
	(
		"ancient_debris",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(94, 66, 58),
		},
	),
	(
		"andesite",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(136, 136, 136),
		},
	),
	(
		"andesite_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(136, 136, 136),
		},
	),
	(
		"andesite_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(136, 136, 136),
		},
	),
	(
		"andesite_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(136, 136, 136),
		},
	),
	(
		"anvil",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(72, 72, 72),
		},
	),
	(
		"attached_melon_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(141, 142, 141),
		},
	),
	(
		"attached_pumpkin_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(139, 139, 139),
		},
	),
	(
		"azalea",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(101, 124, 47),
		},
	),
	(
		"azalea_leaves",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(90, 114, 44),
		},
	),
	(
		"azure_bluet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"bamboo",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(93, 144, 19),
		},
	),
	(
		"bamboo_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"barrel",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(134, 100, 58),
		},
	),
	(
		"barrier",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"basalt",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(80, 81, 86),
		},
	),
	(
		"beacon",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 220, 215),
		},
	),
	(
		"bedrock",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(85, 85, 85),
		},
	),
	(
		"bee_nest",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(202, 160, 74),
		},
	),
	(
		"beehive",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(180, 146, 90),
		},
	),
	(
		"beetroots",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(93, 91, 30),
		},
	),
	(
		"bell",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(253, 235, 110),
		},
	),
	(
		"big_dripleaf",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(111, 141, 51),
		},
	),
	(
		"big_dripleaf_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"birch_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"birch_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(220, 209, 176),
		},
	),
	(
		"birch_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 175, 121),
		},
	),
	(
		"birch_fence_gate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 175, 121),
		},
	),
	(
		"birch_leaves",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Birch}),
			color: BlockColor(130, 129, 130),
		},
	),
	(
		"birch_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(193, 179, 135),
		},
	),
	(
		"birch_planks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 175, 121),
		},
	),
	(
		"birch_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 175, 121),
		},
	),
	(
		"birch_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 160, 79),
		},
	),
	(
		"birch_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 175, 121),
		},
	),
	(
		"birch_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 175, 121),
		},
	),
	(
		"birch_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 175, 121),
		},
	),
	(
		"birch_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(207, 194, 157),
		},
	),
	(
		"birch_wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"birch_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(216, 215, 210),
		},
	),
	(
		"black_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"black_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"black_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"black_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"black_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(20, 21, 25),
		},
	),
	(
		"black_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(8, 10, 15),
		},
	),
	(
		"black_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(25, 26, 31),
		},
	),
	(
		"black_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(67, 30, 32),
		},
	),
	(
		"black_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(25, 25, 29),
		},
	),
	(
		"black_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(25, 25, 25),
		},
	),
	(
		"black_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(24, 24, 24),
		},
	),
	(
		"black_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(37, 22, 16),
		},
	),
	(
		"black_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"black_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(20, 21, 25),
		},
	),
	(
		"blackstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(42, 36, 41),
		},
	),
	(
		"blackstone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(42, 36, 41),
		},
	),
	(
		"blackstone_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(42, 36, 41),
		},
	),
	(
		"blackstone_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(42, 36, 41),
		},
	),
	(
		"blast_furnace",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(80, 80, 81),
		},
	),
	(
		"blue_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"blue_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"blue_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"blue_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"blue_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(53, 57, 157),
		},
	),
	(
		"blue_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 46, 143),
		},
	),
	(
		"blue_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(70, 73, 166),
		},
	),
	(
		"blue_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(47, 64, 139),
		},
	),
	(
		"blue_ice",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(116, 167, 253),
		},
	),
	(
		"blue_orchid",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"blue_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(43, 45, 140),
		},
	),
	(
		"blue_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(51, 76, 178),
		},
	),
	(
		"blue_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(48, 73, 171),
		},
	),
	(
		"blue_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(74, 59, 91),
		},
	),
	(
		"blue_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"blue_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(53, 57, 157),
		},
	),
	(
		"bone_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(209, 206, 179),
		},
	),
	(
		"bookshelf",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"brain_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"brain_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(207, 91, 159),
		},
	),
	(
		"brain_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"brain_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"brewing_stand",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(122, 100, 80),
		},
	),
	(
		"brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(150, 97, 83),
		},
	),
	(
		"brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(150, 97, 83),
		},
	),
	(
		"brick_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(150, 97, 83),
		},
	),
	(
		"bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(150, 97, 83),
		},
	),
	(
		"brown_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"brown_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"brown_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"brown_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"brown_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 71, 40),
		},
	),
	(
		"brown_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(96, 59, 31),
		},
	),
	(
		"brown_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(125, 84, 53),
		},
	),
	(
		"brown_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(119, 106, 85),
		},
	),
	(
		"brown_mushroom",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"brown_mushroom_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(149, 111, 81),
		},
	),
	(
		"brown_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(106, 66, 35),
		},
	),
	(
		"brown_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(102, 76, 51),
		},
	),
	(
		"brown_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(97, 73, 48),
		},
	),
	(
		"brown_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(77, 51, 35),
		},
	),
	(
		"brown_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"brown_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 71, 40),
		},
	),
	(
		"bubble_column",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Water}),
			color: BlockColor(177, 177, 177),
		},
	),
	(
		"bubble_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"bubble_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(165, 26, 162),
		},
	),
	(
		"bubble_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"bubble_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"budding_amethyst",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(132, 96, 186),
		},
	),
	(
		"cactus",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(85, 127, 43),
		},
	),
	(
		"cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"calcite",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 224, 220),
		},
	),
	(
		"campfire",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(110, 88, 54),
		},
	),
	(
		"candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"carrots",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(81, 124, 37),
		},
	),
	(
		"cartography_table",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(103, 87, 67),
		},
	),
	(
		"carved_pumpkin",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(198, 118, 24),
		},
	),
	(
		"cauldron",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(73, 72, 74),
		},
	),
	(
		"cave_air",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"cave_vines",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(90, 109, 40),
		},
	),
	(
		"cave_vines_plant",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(88, 101, 38),
		},
	),
	(
		"chain",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"chain_command_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(131, 161, 147),
		},
	),
	(
		"chest",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"chipped_anvil",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(72, 72, 72),
		},
	),
	(
		"chiseled_deepslate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(54, 54, 54),
		},
	),
	(
		"chiseled_nether_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(47, 23, 28),
		},
	),
	(
		"chiseled_polished_blackstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(53, 48, 56),
		},
	),
	(
		"chiseled_quartz_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(231, 226, 218),
		},
	),
	(
		"chiseled_red_sandstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"chiseled_sandstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"chiseled_stone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(119, 118, 119),
		},
	),
	(
		"chorus_flower",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(151, 120, 151),
		},
	),
	(
		"chorus_plant",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(93, 57, 93),
		},
	),
	(
		"clay",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 166, 179),
		},
	),
	(
		"coal_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(16, 15, 15),
		},
	),
	(
		"coal_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(105, 105, 105),
		},
	),
	(
		"coarse_dirt",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(119, 85, 59),
		},
	),
	(
		"cobbled_deepslate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(77, 77, 80),
		},
	),
	(
		"cobbled_deepslate_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(77, 77, 80),
		},
	),
	(
		"cobbled_deepslate_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(77, 77, 80),
		},
	),
	(
		"cobbled_deepslate_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(77, 77, 80),
		},
	),
	(
		"cobblestone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 127, 127),
		},
	),
	(
		"cobblestone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 127, 127),
		},
	),
	(
		"cobblestone_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 127, 127),
		},
	),
	(
		"cobblestone_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 127, 127),
		},
	),
	(
		"cobweb",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(228, 233, 234),
		},
	),
	(
		"cocoa",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 91, 40),
		},
	),
	(
		"command_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 136, 108),
		},
	),
	(
		"comparator",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(166, 161, 159),
		},
	),
	(
		"composter",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(88, 61, 23),
		},
	),
	(
		"conduit",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(159, 139, 113),
		},
	),
	(
		"copper_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 107, 79),
		},
	),
	(
		"copper_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(124, 125, 120),
		},
	),
	(
		"cornflower",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"cracked_deepslate_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(64, 64, 65),
		},
	),
	(
		"cracked_deepslate_tiles",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(52, 52, 52),
		},
	),
	(
		"cracked_nether_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(40, 20, 23),
		},
	),
	(
		"cracked_polished_blackstone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 37, 43),
		},
	),
	(
		"cracked_stone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(118, 117, 118),
		},
	),
	(
		"crafting_table",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(119, 73, 42),
		},
	),
	(
		"creeper_head",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"creeper_wall_head",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"crimson_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"crimson_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 54, 79),
		},
	),
	(
		"crimson_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(101, 48, 70),
		},
	),
	(
		"crimson_fence_gate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(101, 48, 70),
		},
	),
	(
		"crimson_fungus",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"crimson_hyphae",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(92, 25, 29),
		},
	),
	(
		"crimson_nylium",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(130, 31, 31),
		},
	),
	(
		"crimson_planks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(101, 48, 70),
		},
	),
	(
		"crimson_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(101, 48, 70),
		},
	),
	(
		"crimson_roots",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(126, 8, 41),
		},
	),
	(
		"crimson_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(101, 48, 70),
		},
	),
	(
		"crimson_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(101, 48, 70),
		},
	),
	(
		"crimson_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(101, 48, 70),
		},
	),
	(
		"crimson_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(112, 49, 70),
		},
	),
	(
		"crimson_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(103, 50, 72),
		},
	),
	(
		"crimson_wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"crying_obsidian",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(32, 10, 60),
		},
	),
	(
		"cut_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(191, 106, 80),
		},
	),
	(
		"cut_copper_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(191, 106, 80),
		},
	),
	(
		"cut_copper_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(191, 106, 80),
		},
	),
	(
		"cut_red_sandstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"cut_red_sandstone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"cut_sandstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"cut_sandstone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"cyan_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"cyan_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"cyan_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"cyan_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"cyan_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(21, 137, 145),
		},
	),
	(
		"cyan_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(21, 119, 136),
		},
	),
	(
		"cyan_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(36, 147, 157),
		},
	),
	(
		"cyan_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(52, 118, 125),
		},
	),
	(
		"cyan_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(20, 121, 135),
		},
	),
	(
		"cyan_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(76, 127, 153),
		},
	),
	(
		"cyan_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(73, 122, 147),
		},
	),
	(
		"cyan_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(86, 91, 91),
		},
	),
	(
		"cyan_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"cyan_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(21, 137, 145),
		},
	),
	(
		"damaged_anvil",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(72, 72, 72),
		},
	),
	(
		"dandelion",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dark_oak_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dark_oak_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(76, 51, 25),
		},
	),
	(
		"dark_oak_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(66, 43, 20),
		},
	),
	(
		"dark_oak_fence_gate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(66, 43, 20),
		},
	),
	(
		"dark_oak_leaves",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Foliage}),
			color: BlockColor(150, 150, 150),
		},
	),
	(
		"dark_oak_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(67, 45, 22),
		},
	),
	(
		"dark_oak_planks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(66, 43, 20),
		},
	),
	(
		"dark_oak_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(66, 43, 20),
		},
	),
	(
		"dark_oak_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(61, 90, 30),
		},
	),
	(
		"dark_oak_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(66, 43, 20),
		},
	),
	(
		"dark_oak_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(66, 43, 20),
		},
	),
	(
		"dark_oak_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(66, 43, 20),
		},
	),
	(
		"dark_oak_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(75, 49, 23),
		},
	),
	(
		"dark_oak_wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dark_oak_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(60, 46, 26),
		},
	),
	(
		"dark_prismarine",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(51, 91, 75),
		},
	),
	(
		"dark_prismarine_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(51, 91, 75),
		},
	),
	(
		"dark_prismarine_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(51, 91, 75),
		},
	),
	(
		"daylight_detector",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(130, 116, 94),
		},
	),
	(
		"dead_brain_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dead_brain_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(124, 117, 114),
		},
	),
	(
		"dead_brain_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dead_brain_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dead_bubble_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dead_bubble_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(131, 123, 119),
		},
	),
	(
		"dead_bubble_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dead_bubble_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dead_bush",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(107, 78, 40),
		},
	),
	(
		"dead_fire_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dead_fire_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(131, 123, 119),
		},
	),
	(
		"dead_fire_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dead_fire_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dead_horn_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dead_horn_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(133, 126, 122),
		},
	),
	(
		"dead_horn_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dead_horn_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dead_tube_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dead_tube_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(130, 123, 119),
		},
	),
	(
		"dead_tube_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dead_tube_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"deepslate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(80, 80, 82),
		},
	),
	(
		"deepslate_brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(70, 70, 71),
		},
	),
	(
		"deepslate_brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(70, 70, 71),
		},
	),
	(
		"deepslate_brick_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(70, 70, 71),
		},
	),
	(
		"deepslate_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(70, 70, 71),
		},
	),
	(
		"deepslate_coal_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(74, 74, 76),
		},
	),
	(
		"deepslate_copper_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(92, 93, 89),
		},
	),
	(
		"deepslate_diamond_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(83, 106, 106),
		},
	),
	(
		"deepslate_emerald_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(78, 104, 87),
		},
	),
	(
		"deepslate_gold_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(115, 102, 78),
		},
	),
	(
		"deepslate_iron_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(106, 99, 94),
		},
	),
	(
		"deepslate_lapis_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(79, 90, 115),
		},
	),
	(
		"deepslate_redstone_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(104, 73, 74),
		},
	),
	(
		"deepslate_tile_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(54, 54, 55),
		},
	),
	(
		"deepslate_tile_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(54, 54, 55),
		},
	),
	(
		"deepslate_tile_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(54, 54, 55),
		},
	),
	(
		"deepslate_tiles",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(54, 54, 55),
		},
	),
	(
		"detector_rail",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(123, 104, 90),
		},
	),
	(
		"diamond_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(98, 237, 228),
		},
	),
	(
		"diamond_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(121, 141, 140),
		},
	),
	(
		"diorite",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(188, 188, 188),
		},
	),
	(
		"diorite_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(188, 188, 188),
		},
	),
	(
		"diorite_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(188, 188, 188),
		},
	),
	(
		"diorite_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(188, 188, 188),
		},
	),
	(
		"dirt",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(134, 96, 67),
		},
	),
	(
		"dirt_path",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(148, 121, 65),
		},
	),
	(
		"dispenser",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(110, 109, 109),
		},
	),
	(
		"dragon_egg",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(12, 9, 15),
		},
	),
	(
		"dragon_head",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dragon_wall_head",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"dried_kelp_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(50, 58, 38),
		},
	),
	(
		"dripstone_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(134, 107, 92),
		},
	),
	(
		"dropper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(110, 109, 109),
		},
	),
	(
		"emerald_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(42, 203, 87),
		},
	),
	(
		"emerald_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(108, 136, 115),
		},
	),
	(
		"enchanting_table",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(128, 75, 85),
		},
	),
	(
		"end_gateway",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(15, 10, 24),
		},
	),
	(
		"end_portal",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(15, 10, 24),
		},
	),
	(
		"end_portal_frame",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(91, 120, 97),
		},
	),
	(
		"end_rod",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"end_stone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(219, 222, 158),
		},
	),
	(
		"end_stone_brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(218, 224, 162),
		},
	),
	(
		"end_stone_brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(218, 224, 162),
		},
	),
	(
		"end_stone_brick_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(218, 224, 162),
		},
	),
	(
		"end_stone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(218, 224, 162),
		},
	),
	(
		"ender_chest",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(15, 10, 24),
		},
	),
	(
		"exposed_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(161, 125, 103),
		},
	),
	(
		"exposed_cut_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 121, 101),
		},
	),
	(
		"exposed_cut_copper_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 121, 101),
		},
	),
	(
		"exposed_cut_copper_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 121, 101),
		},
	),
	(
		"farmland",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(81, 44, 15),
		},
	),
	(
		"fern",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"fire",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(211, 140, 53),
		},
	),
	(
		"fire_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"fire_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(163, 35, 46),
		},
	),
	(
		"fire_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"fire_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"fletching_table",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(197, 180, 133),
		},
	),
	(
		"flower_pot",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(124, 68, 53),
		},
	),
	(
		"flowering_azalea",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(112, 121, 64),
		},
	),
	(
		"flowering_azalea_leaves",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 111, 60),
		},
	),
	(
		"frogspawn",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(105, 90, 82),
		},
	),
	(
		"frosted_ice",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(140, 181, 252),
		},
	),
	(
		"furnace",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(110, 109, 109),
		},
	),
	(
		"gilded_blackstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(55, 42, 38),
		},
	),
	(
		"glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(175, 213, 219),
		},
	),
	(
		"glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(170, 210, 217),
		},
	),
	(
		"glow_item_frame",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"glow_lichen",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"glowstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(171, 131, 84),
		},
	),
	(
		"gold_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(246, 208, 61),
		},
	),
	(
		"gold_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(145, 133, 106),
		},
	),
	(
		"granite",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(149, 103, 85),
		},
	),
	(
		"granite_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(149, 103, 85),
		},
	),
	(
		"granite_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(149, 103, 85),
		},
	),
	(
		"granite_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(149, 103, 85),
		},
	),
	(
		"grass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"grass_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(147, 147, 147),
		},
	),
	(
		"grass_path",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(148, 121, 65),
		},
	),
	(
		"gravel",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(131, 127, 126),
		},
	),
	(
		"gray_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"gray_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"gray_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"gray_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"gray_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(62, 68, 71),
		},
	),
	(
		"gray_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(54, 57, 61),
		},
	),
	(
		"gray_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(76, 81, 84),
		},
	),
	(
		"gray_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(83, 90, 93),
		},
	),
	(
		"gray_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(55, 58, 62),
		},
	),
	(
		"gray_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(76, 76, 76),
		},
	),
	(
		"gray_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(73, 73, 73),
		},
	),
	(
		"gray_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(57, 42, 35),
		},
	),
	(
		"gray_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"gray_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(62, 68, 71),
		},
	),
	(
		"green_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"green_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"green_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"green_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"green_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(84, 109, 27),
		},
	),
	(
		"green_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(73, 91, 36),
		},
	),
	(
		"green_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(97, 119, 44),
		},
	),
	(
		"green_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 142, 67),
		},
	),
	(
		"green_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(79, 100, 31),
		},
	),
	(
		"green_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(102, 127, 51),
		},
	),
	(
		"green_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(97, 122, 48),
		},
	),
	(
		"green_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(76, 83, 42),
		},
	),
	(
		"green_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"green_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(84, 109, 27),
		},
	),
	(
		"grindstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(142, 142, 142),
		},
	),
	(
		"hanging_roots",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(161, 115, 91),
		},
	),
	(
		"hay_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(165, 139, 12),
		},
	),
	(
		"heavy_weighted_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(220, 220, 220),
		},
	),
	(
		"honey_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(251, 185, 52),
		},
	),
	(
		"honeycomb_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(229, 148, 29),
		},
	),
	(
		"hopper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(75, 74, 75),
		},
	),
	(
		"horn_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"horn_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(216, 199, 66),
		},
	),
	(
		"horn_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"horn_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"ice",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(145, 183, 253),
		},
	),
	(
		"infested_chiseled_stone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(119, 118, 119),
		},
	),
	(
		"infested_cobblestone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 127, 127),
		},
	),
	(
		"infested_cracked_stone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(118, 117, 118),
		},
	),
	(
		"infested_deepslate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(80, 80, 82),
		},
	),
	(
		"infested_mossy_stone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(115, 121, 105),
		},
	),
	(
		"infested_stone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(125, 125, 125),
		},
	),
	(
		"infested_stone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(122, 121, 122),
		},
	),
	(
		"iron_bars",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(136, 139, 135),
		},
	),
	(
		"iron_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(220, 220, 220),
		},
	),
	(
		"iron_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(194, 193, 193),
		},
	),
	(
		"iron_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(136, 129, 122),
		},
	),
	(
		"iron_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(202, 202, 202),
		},
	),
	(
		"item_frame",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"jack_o_lantern",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(214, 152, 52),
		},
	),
	(
		"jigsaw",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(80, 69, 81),
		},
	),
	(
		"jukebox",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(93, 64, 47),
		},
	),
	(
		"jungle_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"jungle_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(163, 119, 84),
		},
	),
	(
		"jungle_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 115, 80),
		},
	),
	(
		"jungle_fence_gate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 115, 80),
		},
	),
	(
		"jungle_leaves",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Foliage}),
			color: BlockColor(156, 154, 143),
		},
	),
	(
		"jungle_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(149, 109, 70),
		},
	),
	(
		"jungle_planks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 115, 80),
		},
	),
	(
		"jungle_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 115, 80),
		},
	),
	(
		"jungle_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(47, 81, 16),
		},
	),
	(
		"jungle_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 115, 80),
		},
	),
	(
		"jungle_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 115, 80),
		},
	),
	(
		"jungle_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 115, 80),
		},
	),
	(
		"jungle_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(152, 110, 77),
		},
	),
	(
		"jungle_wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"jungle_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(85, 67, 25),
		},
	),
	(
		"kelp",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"kelp_plant",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(86, 130, 42),
		},
	),
	(
		"ladder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"lantern",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(106, 91, 83),
		},
	),
	(
		"lapis_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(30, 67, 140),
		},
	),
	(
		"lapis_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(107, 117, 141),
		},
	),
	(
		"large_amethyst_bud",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"large_fern",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(125, 125, 125),
		},
	),
	(
		"lava",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(212, 90, 18),
		},
	),
	(
		"lava_cauldron",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(73, 72, 74),
		},
	),
	(
		"lectern",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(173, 137, 83),
		},
	),
	(
		"lever",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"light",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"light_blue_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"light_blue_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"light_blue_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"light_blue_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"light_blue_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(58, 175, 217),
		},
	),
	(
		"light_blue_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(35, 137, 198),
		},
	),
	(
		"light_blue_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(74, 180, 213),
		},
	),
	(
		"light_blue_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(94, 164, 208),
		},
	),
	(
		"light_blue_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(49, 163, 212),
		},
	),
	(
		"light_blue_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(102, 153, 216),
		},
	),
	(
		"light_blue_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(97, 147, 208),
		},
	),
	(
		"light_blue_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(113, 108, 137),
		},
	),
	(
		"light_blue_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"light_blue_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(58, 175, 217),
		},
	),
	(
		"light_gray_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"light_gray_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"light_gray_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"light_gray_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"light_gray_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(142, 142, 134),
		},
	),
	(
		"light_gray_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(125, 125, 115),
		},
	),
	(
		"light_gray_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 154, 148),
		},
	),
	(
		"light_gray_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(144, 166, 167),
		},
	),
	(
		"light_gray_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(124, 124, 115),
		},
	),
	(
		"light_gray_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(153, 153, 153),
		},
	),
	(
		"light_gray_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(147, 147, 147),
		},
	),
	(
		"light_gray_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(135, 106, 97),
		},
	),
	(
		"light_gray_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"light_gray_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(142, 142, 134),
		},
	),
	(
		"light_weighted_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(246, 208, 61),
		},
	),
	(
		"lightning_rod",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"lilac",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 125, 147),
		},
	),
	(
		"lily_of_the_valley",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"lily_pad",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(133, 133, 133),
		},
	),
	(
		"lime_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"lime_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"lime_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"lime_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"lime_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(112, 185, 25),
		},
	),
	(
		"lime_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(94, 168, 24),
		},
	),
	(
		"lime_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(125, 189, 41),
		},
	),
	(
		"lime_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 197, 55),
		},
	),
	(
		"lime_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 172, 23),
		},
	),
	(
		"lime_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 204, 25),
		},
	),
	(
		"lime_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(122, 196, 24),
		},
	),
	(
		"lime_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(103, 117, 52),
		},
	),
	(
		"lime_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"lime_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(112, 185, 25),
		},
	),
	(
		"lodestone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(147, 149, 152),
		},
	),
	(
		"loom",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(142, 119, 91),
		},
	),
	(
		"magenta_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"magenta_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"magenta_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"magenta_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"magenta_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(189, 68, 179),
		},
	),
	(
		"magenta_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(169, 48, 159),
		},
	),
	(
		"magenta_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 83, 184),
		},
	),
	(
		"magenta_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(208, 100, 191),
		},
	),
	(
		"magenta_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(173, 54, 163),
		},
	),
	(
		"magenta_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(178, 76, 216),
		},
	),
	(
		"magenta_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(171, 73, 208),
		},
	),
	(
		"magenta_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(149, 88, 108),
		},
	),
	(
		"magenta_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"magenta_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(189, 68, 179),
		},
	),
	(
		"magma_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(142, 63, 31),
		},
	),
	(
		"mangrove_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"mangrove_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(112, 48, 46),
		},
	),
	(
		"mangrove_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 54, 48),
		},
	),
	(
		"mangrove_fence_gate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 54, 48),
		},
	),
	(
		"mangrove_leaves",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Foliage}),
			color: BlockColor(129, 128, 128),
		},
	),
	(
		"mangrove_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(102, 48, 42),
		},
	),
	(
		"mangrove_planks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 54, 48),
		},
	),
	(
		"mangrove_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 54, 48),
		},
	),
	(
		"mangrove_propagule",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(96, 174, 83),
		},
	),
	(
		"mangrove_roots",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(74, 59, 38),
		},
	),
	(
		"mangrove_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 54, 48),
		},
	),
	(
		"mangrove_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 54, 48),
		},
	),
	(
		"mangrove_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 54, 48),
		},
	),
	(
		"mangrove_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(110, 46, 42),
		},
	),
	(
		"mangrove_wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"mangrove_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(83, 66, 41),
		},
	),
	(
		"medium_amethyst_bud",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"melon",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(111, 144, 30),
		},
	),
	(
		"melon_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(153, 153, 153),
		},
	),
	(
		"moss_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(89, 109, 45),
		},
	),
	(
		"moss_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(89, 109, 45),
		},
	),
	(
		"mossy_cobblestone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(110, 118, 94),
		},
	),
	(
		"mossy_cobblestone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(110, 118, 94),
		},
	),
	(
		"mossy_cobblestone_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(110, 118, 94),
		},
	),
	(
		"mossy_cobblestone_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(110, 118, 94),
		},
	),
	(
		"mossy_stone_brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(115, 121, 105),
		},
	),
	(
		"mossy_stone_brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(115, 121, 105),
		},
	),
	(
		"mossy_stone_brick_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(115, 121, 105),
		},
	),
	(
		"mossy_stone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(115, 121, 105),
		},
	),
	(
		"moving_piston",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"mud",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(60, 57, 60),
		},
	),
	(
		"mud_brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(137, 103, 79),
		},
	),
	(
		"mud_brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(137, 103, 79),
		},
	),
	(
		"mud_brick_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(137, 103, 79),
		},
	),
	(
		"mud_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(137, 103, 79),
		},
	),
	(
		"muddy_mangrove_roots",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(70, 58, 45),
		},
	),
	(
		"mushroom_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(203, 196, 185),
		},
	),
	(
		"mycelium",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(111, 98, 101),
		},
	),
	(
		"nether_brick_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 21, 26),
		},
	),
	(
		"nether_brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 21, 26),
		},
	),
	(
		"nether_brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 21, 26),
		},
	),
	(
		"nether_brick_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 21, 26),
		},
	),
	(
		"nether_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 21, 26),
		},
	),
	(
		"nether_gold_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(115, 54, 42),
		},
	),
	(
		"nether_portal",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(89, 11, 192),
		},
	),
	(
		"nether_quartz_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(117, 65, 62),
		},
	),
	(
		"nether_sprouts",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(19, 151, 133),
		},
	),
	(
		"nether_wart",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(111, 18, 19),
		},
	),
	(
		"nether_wart_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 2, 2),
		},
	),
	(
		"netherite_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(66, 61, 63),
		},
	),
	(
		"netherrack",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(97, 38, 38),
		},
	),
	(
		"note_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(88, 58, 40),
		},
	),
	(
		"oak_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"oak_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(140, 110, 66),
		},
	),
	(
		"oak_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"oak_fence_gate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"oak_leaves",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Foliage}),
			color: BlockColor(144, 144, 144),
		},
	),
	(
		"oak_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(151, 121, 73),
		},
	),
	(
		"oak_planks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"oak_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"oak_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(77, 106, 40),
		},
	),
	(
		"oak_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"oak_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"oak_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"oak_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(124, 99, 56),
		},
	),
	(
		"oak_wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"oak_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 85, 50),
		},
	),
	(
		"observer",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(98, 98, 98),
		},
	),
	(
		"obsidian",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(15, 10, 24),
		},
	),
	(
		"ochre_froglight",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(250, 245, 206),
		},
	),
	(
		"orange_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"orange_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"orange_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"orange_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"orange_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(240, 118, 19),
		},
	),
	(
		"orange_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(224, 97, 0),
		},
	),
	(
		"orange_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(227, 131, 31),
		},
	),
	(
		"orange_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 147, 91),
		},
	),
	(
		"orange_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(234, 106, 8),
		},
	),
	(
		"orange_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(216, 127, 51),
		},
	),
	(
		"orange_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(208, 122, 48),
		},
	),
	(
		"orange_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(161, 83, 37),
		},
	),
	(
		"orange_tulip",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"orange_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"orange_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(240, 118, 19),
		},
	),
	(
		"oxeye_daisy",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"oxidized_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(82, 162, 132),
		},
	),
	(
		"oxidized_cut_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(79, 153, 126),
		},
	),
	(
		"oxidized_cut_copper_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(79, 153, 126),
		},
	),
	(
		"oxidized_cut_copper_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(79, 153, 126),
		},
	),
	(
		"packed_ice",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(141, 180, 250),
		},
	),
	(
		"packed_mud",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(142, 106, 79),
		},
	),
	(
		"pearlescent_froglight",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(245, 240, 239),
		},
	),
	(
		"peony",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(129, 126, 139),
		},
	),
	(
		"petrified_oak_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"pink_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"pink_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"pink_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"pink_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"pink_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(237, 141, 172),
		},
	),
	(
		"pink_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(213, 101, 142),
		},
	),
	(
		"pink_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(228, 153, 181),
		},
	),
	(
		"pink_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(235, 154, 181),
		},
	),
	(
		"pink_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(230, 121, 157),
		},
	),
	(
		"pink_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(242, 127, 165),
		},
	),
	(
		"pink_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(233, 122, 159),
		},
	),
	(
		"pink_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(161, 78, 78),
		},
	),
	(
		"pink_tulip",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"pink_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"pink_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(237, 141, 172),
		},
	),
	(
		"piston",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 104, 96),
		},
	),
	(
		"piston_head",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(153, 127, 85),
		},
	),
	(
		"player_head",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"player_wall_head",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"podzol",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(91, 63, 24),
		},
	),
	(
		"pointed_dripstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(129, 102, 89),
		},
	),
	(
		"polished_andesite",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(132, 134, 133),
		},
	),
	(
		"polished_andesite_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(132, 134, 133),
		},
	),
	(
		"polished_andesite_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(132, 134, 133),
		},
	),
	(
		"polished_basalt",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 98, 100),
		},
	),
	(
		"polished_blackstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(53, 48, 56),
		},
	),
	(
		"polished_blackstone_brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(48, 42, 49),
		},
	),
	(
		"polished_blackstone_brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(48, 42, 49),
		},
	),
	(
		"polished_blackstone_brick_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(48, 42, 49),
		},
	),
	(
		"polished_blackstone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(48, 42, 49),
		},
	),
	(
		"polished_blackstone_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"polished_blackstone_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(53, 48, 56),
		},
	),
	(
		"polished_blackstone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(53, 48, 56),
		},
	),
	(
		"polished_blackstone_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(53, 48, 56),
		},
	),
	(
		"polished_blackstone_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(53, 48, 56),
		},
	),
	(
		"polished_deepslate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(72, 72, 73),
		},
	),
	(
		"polished_deepslate_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(72, 72, 73),
		},
	),
	(
		"polished_deepslate_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(72, 72, 73),
		},
	),
	(
		"polished_deepslate_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(72, 72, 73),
		},
	),
	(
		"polished_diorite",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 193, 194),
		},
	),
	(
		"polished_diorite_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 193, 194),
		},
	),
	(
		"polished_diorite_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 193, 194),
		},
	),
	(
		"polished_granite",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 106, 89),
		},
	),
	(
		"polished_granite_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 106, 89),
		},
	),
	(
		"polished_granite_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 106, 89),
		},
	),
	(
		"poppy",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"potatoes",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(84, 135, 47),
		},
	),
	(
		"potted_acacia_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(118, 117, 23),
		},
	),
	(
		"potted_allium",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(158, 137, 183),
		},
	),
	(
		"potted_azalea_bush",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(101, 124, 47),
		},
	),
	(
		"potted_azure_bluet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(169, 204, 127),
		},
	),
	(
		"potted_bamboo",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(93, 144, 19),
		},
	),
	(
		"potted_birch_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 160, 79),
		},
	),
	(
		"potted_blue_orchid",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(47, 162, 168),
		},
	),
	(
		"potted_brown_mushroom",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(153, 116, 92),
		},
	),
	(
		"potted_cactus",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(85, 127, 43),
		},
	),
	(
		"potted_cornflower",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(79, 121, 146),
		},
	),
	(
		"potted_crimson_fungus",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(141, 44, 29),
		},
	),
	(
		"potted_crimson_roots",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 8, 41),
		},
	),
	(
		"potted_dandelion",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(147, 172, 43),
		},
	),
	(
		"potted_dark_oak_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(61, 90, 30),
		},
	),
	(
		"potted_dead_bush",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(107, 78, 40),
		},
	),
	(
		"potted_fern",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(124, 124, 124),
		},
	),
	(
		"potted_flowering_azalea_bush",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(112, 121, 64),
		},
	),
	(
		"potted_jungle_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(47, 81, 16),
		},
	),
	(
		"potted_lily_of_the_valley",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(123, 174, 95),
		},
	),
	(
		"potted_mangrove_propagule",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(96, 174, 83),
		},
	),
	(
		"potted_oak_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(77, 106, 40),
		},
	),
	(
		"potted_orange_tulip",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(93, 142, 30),
		},
	),
	(
		"potted_oxeye_daisy",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(179, 202, 143),
		},
	),
	(
		"potted_pink_tulip",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 157, 78),
		},
	),
	(
		"potted_poppy",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(128, 64, 37),
		},
	),
	(
		"potted_red_mushroom",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(216, 75, 67),
		},
	),
	(
		"potted_red_tulip",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(89, 128, 32),
		},
	),
	(
		"potted_spruce_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 60, 36),
		},
	),
	(
		"potted_warped_fungus",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(74, 109, 87),
		},
	),
	(
		"potted_warped_roots",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(20, 136, 123),
		},
	),
	(
		"potted_white_tulip",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(93, 164, 71),
		},
	),
	(
		"potted_wither_rose",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(41, 44, 23),
		},
	),
	(
		"powder_snow",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 253, 253),
		},
	),
	(
		"powder_snow_cauldron",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(73, 72, 74),
		},
	),
	(
		"powered_rail",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(137, 109, 74),
		},
	),
	(
		"prismarine",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 156, 151),
		},
	),
	(
		"prismarine_brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 171, 158),
		},
	),
	(
		"prismarine_brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 171, 158),
		},
	),
	(
		"prismarine_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 171, 158),
		},
	),
	(
		"prismarine_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 156, 151),
		},
	),
	(
		"prismarine_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 156, 151),
		},
	),
	(
		"prismarine_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(99, 156, 151),
		},
	),
	(
		"pumpkin",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(198, 118, 24),
		},
	),
	(
		"pumpkin_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(154, 154, 154),
		},
	),
	(
		"purple_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"purple_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"purple_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"purple_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"purple_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(121, 42, 172),
		},
	),
	(
		"purple_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(100, 31, 156),
		},
	),
	(
		"purple_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(131, 55, 177),
		},
	),
	(
		"purple_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 48, 152),
		},
	),
	(
		"purple_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(103, 32, 156),
		},
	),
	(
		"purple_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(127, 63, 178),
		},
	),
	(
		"purple_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(122, 61, 171),
		},
	),
	(
		"purple_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(118, 70, 86),
		},
	),
	(
		"purple_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"purple_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(121, 42, 172),
		},
	),
	(
		"purpur_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(169, 125, 169),
		},
	),
	(
		"purpur_pillar",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(171, 129, 171),
		},
	),
	(
		"purpur_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(169, 125, 169),
		},
	),
	(
		"purpur_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(169, 125, 169),
		},
	),
	(
		"quartz_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(235, 229, 222),
		},
	),
	(
		"quartz_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(234, 229, 221),
		},
	),
	(
		"quartz_pillar",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(235, 230, 224),
		},
	),
	(
		"quartz_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(235, 229, 222),
		},
	),
	(
		"quartz_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(235, 229, 222),
		},
	),
	(
		"rail",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(125, 111, 88),
		},
	),
	(
		"raw_copper_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 105, 79),
		},
	),
	(
		"raw_gold_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(221, 169, 46),
		},
	),
	(
		"raw_iron_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(166, 135, 107),
		},
	),
	(
		"red_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"red_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"red_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"red_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"red_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 39, 34),
		},
	),
	(
		"red_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(142, 32, 32),
		},
	),
	(
		"red_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(168, 54, 50),
		},
	),
	(
		"red_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 59, 53),
		},
	),
	(
		"red_mushroom",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"red_mushroom_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(200, 46, 45),
		},
	),
	(
		"red_nether_brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(69, 7, 9),
		},
	),
	(
		"red_nether_brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(69, 7, 9),
		},
	),
	(
		"red_nether_brick_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(69, 7, 9),
		},
	),
	(
		"red_nether_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(69, 7, 9),
		},
	),
	(
		"red_sand",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(190, 102, 33),
		},
	),
	(
		"red_sandstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"red_sandstone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"red_sandstone_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"red_sandstone_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"red_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(140, 31, 30),
		},
	),
	(
		"red_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(153, 51, 51),
		},
	),
	(
		"red_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(147, 48, 48),
		},
	),
	(
		"red_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(143, 61, 46),
		},
	),
	(
		"red_tulip",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"red_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"red_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 39, 34),
		},
	),
	(
		"redstone_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(175, 24, 5),
		},
	),
	(
		"redstone_lamp",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(95, 54, 30),
		},
	),
	(
		"redstone_ore",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(140, 109, 109),
		},
	),
	(
		"redstone_torch",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"redstone_wall_torch",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"redstone_wire",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(175, 24, 5),
		},
	),
	(
		"reinforced_deepslate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(80, 82, 78),
		},
	),
	(
		"repeater",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 157, 156),
		},
	),
	(
		"repeating_command_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(129, 111, 176),
		},
	),
	(
		"respawn_anchor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(75, 26, 144),
		},
	),
	(
		"rooted_dirt",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(144, 103, 76),
		},
	),
	(
		"rose_bush",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(131, 66, 37),
		},
	),
	(
		"sand",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(219, 207, 163),
		},
	),
	(
		"sandstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"sandstone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"sandstone_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"sandstone_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"scaffolding",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(174, 134, 80),
		},
	),
	(
		"sculk",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(12, 29, 36),
		},
	),
	(
		"sculk_catalyst",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(15, 31, 38),
		},
	),
	(
		"sculk_sensor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(7, 70, 84),
		},
	),
	(
		"sculk_shrieker",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(198, 205, 169),
		},
	),
	(
		"sculk_vein",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(7, 48, 57),
		},
	),
	(
		"sea_lantern",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(172, 199, 190),
		},
	),
	(
		"sea_pickle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(90, 97, 39),
		},
	),
	(
		"seagrass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"shroomlight",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(240, 146, 70),
		},
	),
	(
		"shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(139, 96, 139),
		},
	),
	(
		"sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"skeleton_skull",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"skeleton_wall_skull",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"slime_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(111, 192, 91),
		},
	),
	(
		"small_amethyst_bud",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"small_dripleaf",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"smithing_table",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(57, 58, 70),
		},
	),
	(
		"smoker",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(85, 83, 81),
		},
	),
	(
		"smooth_basalt",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(72, 72, 78),
		},
	),
	(
		"smooth_quartz",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(235, 229, 222),
		},
	),
	(
		"smooth_quartz_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(235, 229, 222),
		},
	),
	(
		"smooth_quartz_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(235, 229, 222),
		},
	),
	(
		"smooth_red_sandstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"smooth_red_sandstone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"smooth_red_sandstone_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(181, 97, 31),
		},
	),
	(
		"smooth_sandstone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"smooth_sandstone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"smooth_sandstone_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(223, 214, 170),
		},
	),
	(
		"smooth_stone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(158, 158, 158),
		},
	),
	(
		"smooth_stone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(158, 158, 158),
		},
	),
	(
		"snow",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(249, 254, 254),
		},
	),
	(
		"snow_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(249, 254, 254),
		},
	),
	(
		"soul_campfire",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(80, 204, 208),
		},
	),
	(
		"soul_fire",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(51, 192, 197),
		},
	),
	(
		"soul_lantern",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(71, 99, 114),
		},
	),
	(
		"soul_sand",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(81, 62, 50),
		},
	),
	(
		"soul_soil",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(75, 57, 46),
		},
	),
	(
		"soul_torch",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"soul_wall_torch",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"spawner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(36, 46, 62),
		},
	),
	(
		"sponge",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(195, 192, 74),
		},
	),
	(
		"spore_blossom",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(206, 96, 158),
		},
	),
	(
		"spruce_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"spruce_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(106, 80, 48),
		},
	),
	(
		"spruce_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 84, 48),
		},
	),
	(
		"spruce_fence_gate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 84, 48),
		},
	),
	(
		"spruce_leaves",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Spruce}),
			color: BlockColor(126, 126, 126),
		},
	),
	(
		"spruce_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(108, 80, 46),
		},
	),
	(
		"spruce_planks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 84, 48),
		},
	),
	(
		"spruce_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 84, 48),
		},
	),
	(
		"spruce_sapling",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 60, 36),
		},
	),
	(
		"spruce_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 84, 48),
		},
	),
	(
		"spruce_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 84, 48),
		},
	),
	(
		"spruce_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(114, 84, 48),
		},
	),
	(
		"spruce_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(103, 79, 47),
		},
	),
	(
		"spruce_wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"spruce_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(58, 37, 16),
		},
	),
	(
		"sticky_piston",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 104, 96),
		},
	),
	(
		"stone",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(125, 125, 125),
		},
	),
	(
		"stone_brick_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(122, 121, 122),
		},
	),
	(
		"stone_brick_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(122, 121, 122),
		},
	),
	(
		"stone_brick_wall",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(122, 121, 122),
		},
	),
	(
		"stone_bricks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(122, 121, 122),
		},
	),
	(
		"stone_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"stone_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(125, 125, 125),
		},
	),
	(
		"stone_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(125, 125, 125),
		},
	),
	(
		"stone_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(125, 125, 125),
		},
	),
	(
		"stonecutter",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(123, 118, 111),
		},
	),
	(
		"stripped_acacia_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(166, 91, 51),
		},
	),
	(
		"stripped_acacia_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(174, 92, 59),
		},
	),
	(
		"stripped_birch_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(191, 171, 116),
		},
	),
	(
		"stripped_birch_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(196, 176, 118),
		},
	),
	(
		"stripped_crimson_hyphae",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(137, 57, 90),
		},
	),
	(
		"stripped_crimson_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(121, 56, 82),
		},
	),
	(
		"stripped_dark_oak_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(65, 44, 22),
		},
	),
	(
		"stripped_dark_oak_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(72, 56, 36),
		},
	),
	(
		"stripped_jungle_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(165, 122, 81),
		},
	),
	(
		"stripped_jungle_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(171, 132, 84),
		},
	),
	(
		"stripped_mangrove_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 43, 43),
		},
	),
	(
		"stripped_mangrove_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(119, 54, 47),
		},
	),
	(
		"stripped_oak_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(160, 129, 77),
		},
	),
	(
		"stripped_oak_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(177, 144, 86),
		},
	),
	(
		"stripped_spruce_log",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(105, 80, 46),
		},
	),
	(
		"stripped_spruce_wood",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(115, 89, 52),
		},
	),
	(
		"stripped_warped_hyphae",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(57, 150, 147),
		},
	),
	(
		"stripped_warped_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(52, 128, 124),
		},
	),
	(
		"structure_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(88, 74, 90),
		},
	),
	(
		"structure_void",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"sugar_cane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(148, 192, 101),
		},
	),
	(
		"sunflower",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(246, 196, 54),
		},
	),
	(
		"sweet_berry_bush",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(68, 77, 50),
		},
	),
	(
		"tall_grass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(151, 149, 151),
		},
	),
	(
		"tall_seagrass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(59, 139, 14),
		},
	),
	(
		"target",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(226, 170, 157),
		},
	),
	(
		"terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(152, 94, 67),
		},
	),
	(
		"tinted_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 38, 46),
		},
	),
	(
		"tnt",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(142, 62, 53),
		},
	),
	(
		"torch",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"trapped_chest",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(162, 130, 78),
		},
	),
	(
		"tripwire",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"tripwire_hook",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"tube_coral",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"tube_coral_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(49, 87, 206),
		},
	),
	(
		"tube_coral_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"tube_coral_wall_fan",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"tuff",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(108, 109, 102),
		},
	),
	(
		"turtle_egg",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(228, 226, 191),
		},
	),
	(
		"twisting_vines",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(20, 143, 124),
		},
	),
	(
		"twisting_vines_plant",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(20, 135, 122),
		},
	),
	(
		"verdant_froglight",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(229, 244, 228),
		},
	),
	(
		"vine",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
			color: BlockColor(116, 116, 116),
		},
	),
	(
		"void_air",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"wall_torch",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"warped_button",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"warped_door",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(44, 126, 120),
		},
	),
	(
		"warped_fence",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(43, 104, 99),
		},
	),
	(
		"warped_fence_gate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(43, 104, 99),
		},
	),
	(
		"warped_fungus",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"warped_hyphae",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(58, 58, 77),
		},
	),
	(
		"warped_nylium",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(43, 114, 101),
		},
	),
	(
		"warped_planks",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(43, 104, 99),
		},
	),
	(
		"warped_pressure_plate",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(43, 104, 99),
		},
	),
	(
		"warped_roots",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(20, 138, 124),
		},
	),
	(
		"warped_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(43, 104, 99),
		},
	),
	(
		"warped_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(43, 104, 99),
		},
	),
	(
		"warped_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(43, 104, 99),
		},
	),
	(
		"warped_stem",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(53, 109, 110),
		},
	),
	(
		"warped_trapdoor",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(47, 119, 111),
		},
	),
	(
		"warped_wall_sign",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"warped_wart_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(22, 119, 121),
		},
	),
	(
		"water",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque|Water}),
			color: BlockColor(177, 177, 177),
		},
	),
	(
		"water_cauldron",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(73, 72, 74),
		},
	),
	(
		"waxed_copper_block",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(192, 107, 79),
		},
	),
	(
		"waxed_cut_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(191, 106, 80),
		},
	),
	(
		"waxed_cut_copper_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(191, 106, 80),
		},
	),
	(
		"waxed_cut_copper_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(191, 106, 80),
		},
	),
	(
		"waxed_exposed_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(161, 125, 103),
		},
	),
	(
		"waxed_exposed_cut_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 121, 101),
		},
	),
	(
		"waxed_exposed_cut_copper_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 121, 101),
		},
	),
	(
		"waxed_exposed_cut_copper_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(154, 121, 101),
		},
	),
	(
		"waxed_oxidized_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(82, 162, 132),
		},
	),
	(
		"waxed_oxidized_cut_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(79, 153, 126),
		},
	),
	(
		"waxed_oxidized_cut_copper_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(79, 153, 126),
		},
	),
	(
		"waxed_oxidized_cut_copper_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(79, 153, 126),
		},
	),
	(
		"waxed_weathered_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(108, 153, 110),
		},
	),
	(
		"waxed_weathered_cut_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 145, 107),
		},
	),
	(
		"waxed_weathered_cut_copper_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 145, 107),
		},
	),
	(
		"waxed_weathered_cut_copper_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 145, 107),
		},
	),
	(
		"weathered_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(108, 153, 110),
		},
	),
	(
		"weathered_cut_copper",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 145, 107),
		},
	),
	(
		"weathered_cut_copper_slab",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 145, 107),
		},
	),
	(
		"weathered_cut_copper_stairs",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(109, 145, 107),
		},
	),
	(
		"weeping_vines",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(104, 1, 0),
		},
	),
	(
		"weeping_vines_plant",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(132, 16, 12),
		},
	),
	(
		"wet_sponge",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(171, 181, 70),
		},
	),
	(
		"wheat",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(166, 151, 73),
		},
	),
	(
		"white_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"white_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"white_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"white_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"white_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(233, 236, 236),
		},
	),
	(
		"white_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(207, 213, 214),
		},
	),
	(
		"white_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(225, 227, 227),
		},
	),
	(
		"white_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(188, 212, 202),
		},
	),
	(
		"white_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(215, 220, 221),
		},
	),
	(
		"white_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(255, 255, 255),
		},
	),
	(
		"white_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(246, 246, 246),
		},
	),
	(
		"white_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(209, 178, 161),
		},
	),
	(
		"white_tulip",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"white_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"white_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(233, 236, 236),
		},
	),
	(
		"wither_rose",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"wither_skeleton_skull",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"wither_skeleton_wall_skull",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"yellow_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"yellow_bed",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"yellow_candle",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"yellow_candle_cake",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 222, 214),
		},
	),
	(
		"yellow_carpet",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 197, 39),
		},
	),
	(
		"yellow_concrete",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(240, 175, 21),
		},
	),
	(
		"yellow_concrete_powder",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(232, 199, 54),
		},
	),
	(
		"yellow_glazed_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(234, 192, 88),
		},
	),
	(
		"yellow_shulker_box",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 188, 29),
		},
	),
	(
		"yellow_stained_glass",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(229, 229, 51),
		},
	),
	(
		"yellow_stained_glass_pane",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(221, 221, 48),
		},
	),
	(
		"yellow_terracotta",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(186, 133, 35),
		},
	),
	(
		"yellow_wall_banner",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"yellow_wool",
		BlockType {
			flags: make_bitflags!(BlockFlag::{Opaque}),
			color: BlockColor(248, 197, 39),
		},
	),
	(
		"zombie_head",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
	(
		"zombie_wall_head",
		BlockType {
			flags: make_bitflags!(BlockFlag::{}),
			color: BlockColor(0, 0, 0),
		},
	),
];
