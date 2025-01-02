//! Block type information
//!
//! This file is generated using resource/generate.py, do not edit

use enumflags2::make_bitflags;

use super::*;

/// List if known block types and their properties
pub const BLOCK_TYPES: &[(&str, ConstBlockType)] = &[
	(
		"acacia_button",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"acacia_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([167, 95, 60]),
			},
			sign_material: None,
		},
	),
	(
		"acacia_fence",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([168, 90, 50]),
			},
			sign_material: None,
		},
	),
	(
		"acacia_fence_gate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([168, 90, 50]),
			},
			sign_material: None,
		},
	),
	(
		"acacia_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("acacia"),
		},
	),
	(
		"acacia_leaves",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Foliage}),
				color: Color([149, 148, 148]),
			},
			sign_material: None,
		},
	),
	(
		"acacia_log",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([150, 88, 55]),
			},
			sign_material: None,
		},
	),
	(
		"acacia_planks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([168, 90, 50]),
			},
			sign_material: None,
		},
	),
	(
		"acacia_pressure_plate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([168, 90, 50]),
			},
			sign_material: None,
		},
	),
	(
		"acacia_sapling",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([118, 117, 23]),
			},
			sign_material: None,
		},
	),
	(
		"acacia_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("acacia"),
		},
	),
	(
		"acacia_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([168, 90, 50]),
			},
			sign_material: None,
		},
	),
	(
		"acacia_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([168, 90, 50]),
			},
			sign_material: None,
		},
	),
	(
		"acacia_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([156, 87, 51]),
			},
			sign_material: None,
		},
	),
	(
		"acacia_wall_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("acacia"),
		},
	),
	(
		"acacia_wall_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("acacia"),
		},
	),
	(
		"acacia_wood",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([103, 96, 86]),
			},
			sign_material: None,
		},
	),
	(
		"activator_rail",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([115, 87, 74]),
			},
			sign_material: None,
		},
	),
	(
		"air",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"allium",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"amethyst_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([133, 97, 191]),
			},
			sign_material: None,
		},
	),
	(
		"amethyst_cluster",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([163, 126, 207]),
			},
			sign_material: None,
		},
	),
	(
		"ancient_debris",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([94, 66, 58]),
			},
			sign_material: None,
		},
	),
	(
		"andesite",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([136, 136, 136]),
			},
			sign_material: None,
		},
	),
	(
		"andesite_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([136, 136, 136]),
			},
			sign_material: None,
		},
	),
	(
		"andesite_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([136, 136, 136]),
			},
			sign_material: None,
		},
	),
	(
		"andesite_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([136, 136, 136]),
			},
			sign_material: None,
		},
	),
	(
		"anvil",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([72, 72, 72]),
			},
			sign_material: None,
		},
	),
	(
		"attached_melon_stem",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
				color: Color([141, 142, 141]),
			},
			sign_material: None,
		},
	),
	(
		"attached_pumpkin_stem",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
				color: Color([139, 139, 139]),
			},
			sign_material: None,
		},
	),
	(
		"azalea",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([101, 124, 47]),
			},
			sign_material: None,
		},
	),
	(
		"azalea_leaves",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([90, 114, 44]),
			},
			sign_material: None,
		},
	),
	(
		"azure_bluet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"bamboo",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([93, 144, 19]),
			},
			sign_material: None,
		},
	),
	(
		"bamboo_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([139, 141, 62]),
			},
			sign_material: None,
		},
	),
	(
		"bamboo_button",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"bamboo_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([191, 171, 81]),
			},
			sign_material: None,
		},
	),
	(
		"bamboo_fence",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([193, 173, 80]),
			},
			sign_material: None,
		},
	),
	(
		"bamboo_fence_gate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([193, 173, 80]),
			},
			sign_material: None,
		},
	),
	(
		"bamboo_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("bamboo"),
		},
	),
	(
		"bamboo_mosaic",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([190, 170, 78]),
			},
			sign_material: None,
		},
	),
	(
		"bamboo_mosaic_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([190, 170, 78]),
			},
			sign_material: None,
		},
	),
	(
		"bamboo_mosaic_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([190, 170, 78]),
			},
			sign_material: None,
		},
	),
	(
		"bamboo_planks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([193, 173, 80]),
			},
			sign_material: None,
		},
	),
	(
		"bamboo_pressure_plate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([193, 173, 80]),
			},
			sign_material: None,
		},
	),
	(
		"bamboo_sapling",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"bamboo_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("bamboo"),
		},
	),
	(
		"bamboo_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([193, 173, 80]),
			},
			sign_material: None,
		},
	),
	(
		"bamboo_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([193, 173, 80]),
			},
			sign_material: None,
		},
	),
	(
		"bamboo_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([198, 179, 85]),
			},
			sign_material: None,
		},
	),
	(
		"bamboo_wall_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("bamboo"),
		},
	),
	(
		"bamboo_wall_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("bamboo"),
		},
	),
	(
		"barrel",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([134, 100, 58]),
			},
			sign_material: None,
		},
	),
	(
		"barrier",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"basalt",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([80, 81, 86]),
			},
			sign_material: None,
		},
	),
	(
		"beacon",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([117, 220, 215]),
			},
			sign_material: None,
		},
	),
	(
		"bedrock",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([85, 85, 85]),
			},
			sign_material: None,
		},
	),
	(
		"bee_nest",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([202, 160, 74]),
			},
			sign_material: None,
		},
	),
	(
		"beehive",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([180, 146, 90]),
			},
			sign_material: None,
		},
	),
	(
		"beetroots",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([93, 91, 30]),
			},
			sign_material: None,
		},
	),
	(
		"bell",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([253, 235, 110]),
			},
			sign_material: None,
		},
	),
	(
		"big_dripleaf",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([111, 141, 51]),
			},
			sign_material: None,
		},
	),
	(
		"big_dripleaf_stem",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"birch_button",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"birch_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([220, 209, 176]),
			},
			sign_material: None,
		},
	),
	(
		"birch_fence",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([192, 175, 121]),
			},
			sign_material: None,
		},
	),
	(
		"birch_fence_gate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([192, 175, 121]),
			},
			sign_material: None,
		},
	),
	(
		"birch_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("birch"),
		},
	),
	(
		"birch_leaves",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Birch}),
				color: Color([130, 129, 130]),
			},
			sign_material: None,
		},
	),
	(
		"birch_log",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([193, 179, 135]),
			},
			sign_material: None,
		},
	),
	(
		"birch_planks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([192, 175, 121]),
			},
			sign_material: None,
		},
	),
	(
		"birch_pressure_plate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([192, 175, 121]),
			},
			sign_material: None,
		},
	),
	(
		"birch_sapling",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([127, 160, 79]),
			},
			sign_material: None,
		},
	),
	(
		"birch_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("birch"),
		},
	),
	(
		"birch_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([192, 175, 121]),
			},
			sign_material: None,
		},
	),
	(
		"birch_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([192, 175, 121]),
			},
			sign_material: None,
		},
	),
	(
		"birch_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([207, 194, 157]),
			},
			sign_material: None,
		},
	),
	(
		"birch_wall_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("birch"),
		},
	),
	(
		"birch_wall_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("birch"),
		},
	),
	(
		"birch_wood",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([216, 215, 210]),
			},
			sign_material: None,
		},
	),
	(
		"black_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"black_bed",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"black_candle",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"black_candle_cake",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 222, 214]),
			},
			sign_material: None,
		},
	),
	(
		"black_carpet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([20, 21, 25]),
			},
			sign_material: None,
		},
	),
	(
		"black_concrete",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([8, 10, 15]),
			},
			sign_material: None,
		},
	),
	(
		"black_concrete_powder",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([25, 26, 31]),
			},
			sign_material: None,
		},
	),
	(
		"black_glazed_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([67, 30, 32]),
			},
			sign_material: None,
		},
	),
	(
		"black_shulker_box",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([25, 25, 29]),
			},
			sign_material: None,
		},
	),
	(
		"black_stained_glass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([25, 25, 25]),
			},
			sign_material: None,
		},
	),
	(
		"black_stained_glass_pane",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([24, 24, 24]),
			},
			sign_material: None,
		},
	),
	(
		"black_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([37, 22, 16]),
			},
			sign_material: None,
		},
	),
	(
		"black_wall_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"black_wool",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([20, 21, 25]),
			},
			sign_material: None,
		},
	),
	(
		"blackstone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([42, 36, 41]),
			},
			sign_material: None,
		},
	),
	(
		"blackstone_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([42, 36, 41]),
			},
			sign_material: None,
		},
	),
	(
		"blackstone_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([42, 36, 41]),
			},
			sign_material: None,
		},
	),
	(
		"blackstone_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([42, 36, 41]),
			},
			sign_material: None,
		},
	),
	(
		"blast_furnace",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([80, 80, 81]),
			},
			sign_material: None,
		},
	),
	(
		"blue_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"blue_bed",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"blue_candle",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"blue_candle_cake",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 222, 214]),
			},
			sign_material: None,
		},
	),
	(
		"blue_carpet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([53, 57, 157]),
			},
			sign_material: None,
		},
	),
	(
		"blue_concrete",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([44, 46, 143]),
			},
			sign_material: None,
		},
	),
	(
		"blue_concrete_powder",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([70, 73, 166]),
			},
			sign_material: None,
		},
	),
	(
		"blue_glazed_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([47, 64, 139]),
			},
			sign_material: None,
		},
	),
	(
		"blue_ice",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([116, 167, 253]),
			},
			sign_material: None,
		},
	),
	(
		"blue_orchid",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"blue_shulker_box",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([43, 45, 140]),
			},
			sign_material: None,
		},
	),
	(
		"blue_stained_glass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([51, 76, 178]),
			},
			sign_material: None,
		},
	),
	(
		"blue_stained_glass_pane",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([48, 73, 171]),
			},
			sign_material: None,
		},
	),
	(
		"blue_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([74, 59, 91]),
			},
			sign_material: None,
		},
	),
	(
		"blue_wall_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"blue_wool",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([53, 57, 157]),
			},
			sign_material: None,
		},
	),
	(
		"bone_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([209, 206, 179]),
			},
			sign_material: None,
		},
	),
	(
		"bookshelf",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([162, 130, 78]),
			},
			sign_material: None,
		},
	),
	(
		"brain_coral",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"brain_coral_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([207, 91, 159]),
			},
			sign_material: None,
		},
	),
	(
		"brain_coral_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"brain_coral_wall_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"brewing_stand",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([122, 100, 80]),
			},
			sign_material: None,
		},
	),
	(
		"brick_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([150, 97, 83]),
			},
			sign_material: None,
		},
	),
	(
		"brick_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([150, 97, 83]),
			},
			sign_material: None,
		},
	),
	(
		"brick_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([150, 97, 83]),
			},
			sign_material: None,
		},
	),
	(
		"bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([150, 97, 83]),
			},
			sign_material: None,
		},
	),
	(
		"brown_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"brown_bed",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"brown_candle",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"brown_candle_cake",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 222, 214]),
			},
			sign_material: None,
		},
	),
	(
		"brown_carpet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([114, 71, 40]),
			},
			sign_material: None,
		},
	),
	(
		"brown_concrete",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([96, 59, 31]),
			},
			sign_material: None,
		},
	),
	(
		"brown_concrete_powder",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([125, 84, 53]),
			},
			sign_material: None,
		},
	),
	(
		"brown_glazed_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([119, 106, 85]),
			},
			sign_material: None,
		},
	),
	(
		"brown_mushroom",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"brown_mushroom_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([149, 111, 81]),
			},
			sign_material: None,
		},
	),
	(
		"brown_shulker_box",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([106, 66, 35]),
			},
			sign_material: None,
		},
	),
	(
		"brown_stained_glass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([102, 76, 51]),
			},
			sign_material: None,
		},
	),
	(
		"brown_stained_glass_pane",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([97, 73, 48]),
			},
			sign_material: None,
		},
	),
	(
		"brown_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([77, 51, 35]),
			},
			sign_material: None,
		},
	),
	(
		"brown_wall_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"brown_wool",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([114, 71, 40]),
			},
			sign_material: None,
		},
	),
	(
		"bubble_column",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Water}),
				color: Color([177, 177, 177]),
			},
			sign_material: None,
		},
	),
	(
		"bubble_coral",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"bubble_coral_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([165, 26, 162]),
			},
			sign_material: None,
		},
	),
	(
		"bubble_coral_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"bubble_coral_wall_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"budding_amethyst",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([132, 96, 186]),
			},
			sign_material: None,
		},
	),
	(
		"cactus",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([85, 127, 43]),
			},
			sign_material: None,
		},
	),
	(
		"cake",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 222, 214]),
			},
			sign_material: None,
		},
	),
	(
		"calcite",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([223, 224, 220]),
			},
			sign_material: None,
		},
	),
	(
		"calibrated_sculk_sensor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([27, 79, 100]),
			},
			sign_material: None,
		},
	),
	(
		"campfire",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([110, 88, 54]),
			},
			sign_material: None,
		},
	),
	(
		"candle",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"candle_cake",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 222, 214]),
			},
			sign_material: None,
		},
	),
	(
		"carrots",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([81, 124, 37]),
			},
			sign_material: None,
		},
	),
	(
		"cartography_table",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([103, 87, 67]),
			},
			sign_material: None,
		},
	),
	(
		"carved_pumpkin",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([198, 118, 24]),
			},
			sign_material: None,
		},
	),
	(
		"cauldron",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([73, 72, 74]),
			},
			sign_material: None,
		},
	),
	(
		"cave_air",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"cave_vines",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([90, 109, 40]),
			},
			sign_material: None,
		},
	),
	(
		"cave_vines_plant",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([88, 101, 38]),
			},
			sign_material: None,
		},
	),
	(
		"chain",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"chain_command_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([131, 161, 147]),
			},
			sign_material: None,
		},
	),
	(
		"cherry_button",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"cherry_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([223, 170, 164]),
			},
			sign_material: None,
		},
	),
	(
		"cherry_fence",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([226, 178, 172]),
			},
			sign_material: None,
		},
	),
	(
		"cherry_fence_gate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([226, 178, 172]),
			},
			sign_material: None,
		},
	),
	(
		"cherry_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("cherry"),
		},
	),
	(
		"cherry_leaves",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([229, 172, 194]),
			},
			sign_material: None,
		},
	),
	(
		"cherry_log",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([185, 141, 137]),
			},
			sign_material: None,
		},
	),
	(
		"cherry_planks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([226, 178, 172]),
			},
			sign_material: None,
		},
	),
	(
		"cherry_pressure_plate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([226, 178, 172]),
			},
			sign_material: None,
		},
	),
	(
		"cherry_sapling",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"cherry_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("cherry"),
		},
	),
	(
		"cherry_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([226, 178, 172]),
			},
			sign_material: None,
		},
	),
	(
		"cherry_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([226, 178, 172]),
			},
			sign_material: None,
		},
	),
	(
		"cherry_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([226, 178, 172]),
			},
			sign_material: None,
		},
	),
	(
		"cherry_wall_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("cherry"),
		},
	),
	(
		"cherry_wall_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("cherry"),
		},
	),
	(
		"cherry_wood",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([54, 33, 44]),
			},
			sign_material: None,
		},
	),
	(
		"chest",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([162, 130, 78]),
			},
			sign_material: None,
		},
	),
	(
		"chipped_anvil",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([72, 72, 72]),
			},
			sign_material: None,
		},
	),
	(
		"chiseled_bookshelf",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([178, 144, 88]),
			},
			sign_material: None,
		},
	),
	(
		"chiseled_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([184, 100, 73]),
			},
			sign_material: None,
		},
	),
	(
		"chiseled_deepslate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([54, 54, 54]),
			},
			sign_material: None,
		},
	),
	(
		"chiseled_nether_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([45, 22, 27]),
			},
			sign_material: None,
		},
	),
	(
		"chiseled_polished_blackstone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([53, 48, 56]),
			},
			sign_material: None,
		},
	),
	(
		"chiseled_quartz_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([231, 226, 218]),
			},
			sign_material: None,
		},
	),
	(
		"chiseled_red_sandstone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([181, 97, 31]),
			},
			sign_material: None,
		},
	),
	(
		"chiseled_resin_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([200, 84, 24]),
			},
			sign_material: None,
		},
	),
	(
		"chiseled_sandstone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([223, 214, 170]),
			},
			sign_material: None,
		},
	),
	(
		"chiseled_stone_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([119, 118, 119]),
			},
			sign_material: None,
		},
	),
	(
		"chiseled_tuff",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([89, 94, 86]),
			},
			sign_material: None,
		},
	),
	(
		"chiseled_tuff_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([110, 113, 106]),
			},
			sign_material: None,
		},
	),
	(
		"chorus_flower",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([151, 120, 151]),
			},
			sign_material: None,
		},
	),
	(
		"chorus_plant",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([93, 57, 93]),
			},
			sign_material: None,
		},
	),
	(
		"clay",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([160, 166, 179]),
			},
			sign_material: None,
		},
	),
	(
		"closed_eyeblossom",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"coal_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([16, 15, 15]),
			},
			sign_material: None,
		},
	),
	(
		"coal_ore",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([105, 105, 105]),
			},
			sign_material: None,
		},
	),
	(
		"coarse_dirt",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([119, 85, 59]),
			},
			sign_material: None,
		},
	),
	(
		"cobbled_deepslate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([77, 77, 80]),
			},
			sign_material: None,
		},
	),
	(
		"cobbled_deepslate_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([77, 77, 80]),
			},
			sign_material: None,
		},
	),
	(
		"cobbled_deepslate_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([77, 77, 80]),
			},
			sign_material: None,
		},
	),
	(
		"cobbled_deepslate_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([77, 77, 80]),
			},
			sign_material: None,
		},
	),
	(
		"cobblestone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([127, 127, 127]),
			},
			sign_material: None,
		},
	),
	(
		"cobblestone_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([127, 127, 127]),
			},
			sign_material: None,
		},
	),
	(
		"cobblestone_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([127, 127, 127]),
			},
			sign_material: None,
		},
	),
	(
		"cobblestone_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([127, 127, 127]),
			},
			sign_material: None,
		},
	),
	(
		"cobweb",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([228, 233, 234]),
			},
			sign_material: None,
		},
	),
	(
		"cocoa",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([154, 91, 40]),
			},
			sign_material: None,
		},
	),
	(
		"command_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([181, 136, 108]),
			},
			sign_material: None,
		},
	),
	(
		"comparator",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([166, 161, 159]),
			},
			sign_material: None,
		},
	),
	(
		"composter",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([88, 61, 23]),
			},
			sign_material: None,
		},
	),
	(
		"conduit",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([159, 139, 113]),
			},
			sign_material: None,
		},
	),
	(
		"copper_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([192, 107, 79]),
			},
			sign_material: None,
		},
	),
	(
		"copper_bulb",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([156, 86, 57]),
			},
			sign_material: None,
		},
	),
	(
		"copper_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([192, 109, 82]),
			},
			sign_material: None,
		},
	),
	(
		"copper_grate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([191, 107, 79]),
			},
			sign_material: None,
		},
	),
	(
		"copper_ore",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([124, 125, 120]),
			},
			sign_material: None,
		},
	),
	(
		"copper_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([190, 106, 79]),
			},
			sign_material: None,
		},
	),
	(
		"cornflower",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"cracked_deepslate_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([64, 64, 65]),
			},
			sign_material: None,
		},
	),
	(
		"cracked_deepslate_tiles",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([52, 52, 52]),
			},
			sign_material: None,
		},
	),
	(
		"cracked_nether_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([40, 20, 23]),
			},
			sign_material: None,
		},
	),
	(
		"cracked_polished_blackstone_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([44, 37, 43]),
			},
			sign_material: None,
		},
	),
	(
		"cracked_stone_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([118, 117, 118]),
			},
			sign_material: None,
		},
	),
	(
		"crafter",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([112, 98, 99]),
			},
			sign_material: None,
		},
	),
	(
		"crafting_table",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([119, 73, 42]),
			},
			sign_material: None,
		},
	),
	(
		"creaking_heart",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([73, 59, 54]),
			},
			sign_material: None,
		},
	),
	(
		"creeper_head",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"creeper_wall_head",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"crimson_button",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"crimson_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([114, 54, 79]),
			},
			sign_material: None,
		},
	),
	(
		"crimson_fence",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([101, 48, 70]),
			},
			sign_material: None,
		},
	),
	(
		"crimson_fence_gate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([101, 48, 70]),
			},
			sign_material: None,
		},
	),
	(
		"crimson_fungus",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"crimson_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("crimson"),
		},
	),
	(
		"crimson_hyphae",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([92, 25, 29]),
			},
			sign_material: None,
		},
	),
	(
		"crimson_nylium",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([130, 31, 31]),
			},
			sign_material: None,
		},
	),
	(
		"crimson_planks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([101, 48, 70]),
			},
			sign_material: None,
		},
	),
	(
		"crimson_pressure_plate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([101, 48, 70]),
			},
			sign_material: None,
		},
	),
	(
		"crimson_roots",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([126, 8, 41]),
			},
			sign_material: None,
		},
	),
	(
		"crimson_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("crimson"),
		},
	),
	(
		"crimson_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([101, 48, 70]),
			},
			sign_material: None,
		},
	),
	(
		"crimson_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([101, 48, 70]),
			},
			sign_material: None,
		},
	),
	(
		"crimson_stem",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([112, 49, 70]),
			},
			sign_material: None,
		},
	),
	(
		"crimson_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([103, 50, 72]),
			},
			sign_material: None,
		},
	),
	(
		"crimson_wall_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("crimson"),
		},
	),
	(
		"crimson_wall_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("crimson"),
		},
	),
	(
		"crying_obsidian",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([32, 10, 60]),
			},
			sign_material: None,
		},
	),
	(
		"cut_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([191, 106, 80]),
			},
			sign_material: None,
		},
	),
	(
		"cut_copper_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([191, 106, 80]),
			},
			sign_material: None,
		},
	),
	(
		"cut_copper_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([191, 106, 80]),
			},
			sign_material: None,
		},
	),
	(
		"cut_red_sandstone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([181, 97, 31]),
			},
			sign_material: None,
		},
	),
	(
		"cut_red_sandstone_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([181, 97, 31]),
			},
			sign_material: None,
		},
	),
	(
		"cut_sandstone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([223, 214, 170]),
			},
			sign_material: None,
		},
	),
	(
		"cut_sandstone_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([223, 214, 170]),
			},
			sign_material: None,
		},
	),
	(
		"cyan_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"cyan_bed",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"cyan_candle",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"cyan_candle_cake",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 222, 214]),
			},
			sign_material: None,
		},
	),
	(
		"cyan_carpet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([21, 137, 145]),
			},
			sign_material: None,
		},
	),
	(
		"cyan_concrete",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([21, 119, 136]),
			},
			sign_material: None,
		},
	),
	(
		"cyan_concrete_powder",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([36, 147, 157]),
			},
			sign_material: None,
		},
	),
	(
		"cyan_glazed_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([52, 118, 125]),
			},
			sign_material: None,
		},
	),
	(
		"cyan_shulker_box",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([20, 121, 135]),
			},
			sign_material: None,
		},
	),
	(
		"cyan_stained_glass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([76, 127, 153]),
			},
			sign_material: None,
		},
	),
	(
		"cyan_stained_glass_pane",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([73, 122, 147]),
			},
			sign_material: None,
		},
	),
	(
		"cyan_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([86, 91, 91]),
			},
			sign_material: None,
		},
	),
	(
		"cyan_wall_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"cyan_wool",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([21, 137, 145]),
			},
			sign_material: None,
		},
	),
	(
		"damaged_anvil",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([72, 72, 72]),
			},
			sign_material: None,
		},
	),
	(
		"dandelion",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"dark_oak_button",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"dark_oak_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([76, 51, 25]),
			},
			sign_material: None,
		},
	),
	(
		"dark_oak_fence",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([66, 43, 20]),
			},
			sign_material: None,
		},
	),
	(
		"dark_oak_fence_gate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([66, 43, 20]),
			},
			sign_material: None,
		},
	),
	(
		"dark_oak_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("dark_oak"),
		},
	),
	(
		"dark_oak_leaves",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Foliage}),
				color: Color([150, 150, 150]),
			},
			sign_material: None,
		},
	),
	(
		"dark_oak_log",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([67, 45, 22]),
			},
			sign_material: None,
		},
	),
	(
		"dark_oak_planks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([66, 43, 20]),
			},
			sign_material: None,
		},
	),
	(
		"dark_oak_pressure_plate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([66, 43, 20]),
			},
			sign_material: None,
		},
	),
	(
		"dark_oak_sapling",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([61, 90, 30]),
			},
			sign_material: None,
		},
	),
	(
		"dark_oak_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("dark_oak"),
		},
	),
	(
		"dark_oak_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([66, 43, 20]),
			},
			sign_material: None,
		},
	),
	(
		"dark_oak_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([66, 43, 20]),
			},
			sign_material: None,
		},
	),
	(
		"dark_oak_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([75, 49, 23]),
			},
			sign_material: None,
		},
	),
	(
		"dark_oak_wall_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("dark_oak"),
		},
	),
	(
		"dark_oak_wall_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("dark_oak"),
		},
	),
	(
		"dark_oak_wood",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([60, 46, 26]),
			},
			sign_material: None,
		},
	),
	(
		"dark_prismarine",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([51, 91, 75]),
			},
			sign_material: None,
		},
	),
	(
		"dark_prismarine_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([51, 91, 75]),
			},
			sign_material: None,
		},
	),
	(
		"dark_prismarine_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([51, 91, 75]),
			},
			sign_material: None,
		},
	),
	(
		"daylight_detector",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([130, 116, 94]),
			},
			sign_material: None,
		},
	),
	(
		"dead_brain_coral",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"dead_brain_coral_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([124, 117, 114]),
			},
			sign_material: None,
		},
	),
	(
		"dead_brain_coral_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"dead_brain_coral_wall_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"dead_bubble_coral",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"dead_bubble_coral_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([131, 123, 119]),
			},
			sign_material: None,
		},
	),
	(
		"dead_bubble_coral_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"dead_bubble_coral_wall_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"dead_bush",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([107, 78, 40]),
			},
			sign_material: None,
		},
	),
	(
		"dead_fire_coral",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"dead_fire_coral_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([131, 123, 119]),
			},
			sign_material: None,
		},
	),
	(
		"dead_fire_coral_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"dead_fire_coral_wall_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"dead_horn_coral",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"dead_horn_coral_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([133, 126, 122]),
			},
			sign_material: None,
		},
	),
	(
		"dead_horn_coral_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"dead_horn_coral_wall_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"dead_tube_coral",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"dead_tube_coral_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([130, 123, 119]),
			},
			sign_material: None,
		},
	),
	(
		"dead_tube_coral_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"dead_tube_coral_wall_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"decorated_pot",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([124, 68, 53]),
			},
			sign_material: None,
		},
	),
	(
		"deepslate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([80, 80, 82]),
			},
			sign_material: None,
		},
	),
	(
		"deepslate_brick_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([70, 70, 71]),
			},
			sign_material: None,
		},
	),
	(
		"deepslate_brick_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([70, 70, 71]),
			},
			sign_material: None,
		},
	),
	(
		"deepslate_brick_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([70, 70, 71]),
			},
			sign_material: None,
		},
	),
	(
		"deepslate_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([70, 70, 71]),
			},
			sign_material: None,
		},
	),
	(
		"deepslate_coal_ore",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([74, 74, 76]),
			},
			sign_material: None,
		},
	),
	(
		"deepslate_copper_ore",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([92, 93, 89]),
			},
			sign_material: None,
		},
	),
	(
		"deepslate_diamond_ore",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([83, 106, 106]),
			},
			sign_material: None,
		},
	),
	(
		"deepslate_emerald_ore",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([78, 104, 87]),
			},
			sign_material: None,
		},
	),
	(
		"deepslate_gold_ore",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([115, 102, 78]),
			},
			sign_material: None,
		},
	),
	(
		"deepslate_iron_ore",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([106, 99, 94]),
			},
			sign_material: None,
		},
	),
	(
		"deepslate_lapis_ore",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([79, 90, 115]),
			},
			sign_material: None,
		},
	),
	(
		"deepslate_redstone_ore",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([104, 73, 74]),
			},
			sign_material: None,
		},
	),
	(
		"deepslate_tile_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([54, 54, 55]),
			},
			sign_material: None,
		},
	),
	(
		"deepslate_tile_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([54, 54, 55]),
			},
			sign_material: None,
		},
	),
	(
		"deepslate_tile_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([54, 54, 55]),
			},
			sign_material: None,
		},
	),
	(
		"deepslate_tiles",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([54, 54, 55]),
			},
			sign_material: None,
		},
	),
	(
		"detector_rail",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([123, 104, 90]),
			},
			sign_material: None,
		},
	),
	(
		"diamond_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([98, 237, 228]),
			},
			sign_material: None,
		},
	),
	(
		"diamond_ore",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([121, 141, 140]),
			},
			sign_material: None,
		},
	),
	(
		"diorite",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([188, 188, 188]),
			},
			sign_material: None,
		},
	),
	(
		"diorite_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([188, 188, 188]),
			},
			sign_material: None,
		},
	),
	(
		"diorite_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([188, 188, 188]),
			},
			sign_material: None,
		},
	),
	(
		"diorite_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([188, 188, 188]),
			},
			sign_material: None,
		},
	),
	(
		"dirt",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([134, 96, 67]),
			},
			sign_material: None,
		},
	),
	(
		"dirt_path",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([148, 121, 65]),
			},
			sign_material: None,
		},
	),
	(
		"dispenser",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([110, 109, 109]),
			},
			sign_material: None,
		},
	),
	(
		"dragon_egg",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([12, 9, 15]),
			},
			sign_material: None,
		},
	),
	(
		"dragon_head",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"dragon_wall_head",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"dried_kelp_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([50, 58, 38]),
			},
			sign_material: None,
		},
	),
	(
		"dripstone_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([134, 107, 92]),
			},
			sign_material: None,
		},
	),
	(
		"dropper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([110, 109, 109]),
			},
			sign_material: None,
		},
	),
	(
		"emerald_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([42, 203, 87]),
			},
			sign_material: None,
		},
	),
	(
		"emerald_ore",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([108, 136, 115]),
			},
			sign_material: None,
		},
	),
	(
		"enchanting_table",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([128, 75, 85]),
			},
			sign_material: None,
		},
	),
	(
		"end_gateway",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([15, 10, 24]),
			},
			sign_material: None,
		},
	),
	(
		"end_portal",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([15, 10, 24]),
			},
			sign_material: None,
		},
	),
	(
		"end_portal_frame",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([91, 120, 97]),
			},
			sign_material: None,
		},
	),
	(
		"end_rod",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"end_stone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([219, 222, 158]),
			},
			sign_material: None,
		},
	),
	(
		"end_stone_brick_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([218, 224, 162]),
			},
			sign_material: None,
		},
	),
	(
		"end_stone_brick_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([218, 224, 162]),
			},
			sign_material: None,
		},
	),
	(
		"end_stone_brick_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([218, 224, 162]),
			},
			sign_material: None,
		},
	),
	(
		"end_stone_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([218, 224, 162]),
			},
			sign_material: None,
		},
	),
	(
		"ender_chest",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([15, 10, 24]),
			},
			sign_material: None,
		},
	),
	(
		"exposed_chiseled_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([154, 119, 100]),
			},
			sign_material: None,
		},
	),
	(
		"exposed_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([161, 125, 103]),
			},
			sign_material: None,
		},
	),
	(
		"exposed_copper_bulb",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([135, 107, 89]),
			},
			sign_material: None,
		},
	),
	(
		"exposed_copper_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([164, 123, 106]),
			},
			sign_material: None,
		},
	),
	(
		"exposed_copper_grate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([161, 125, 104]),
			},
			sign_material: None,
		},
	),
	(
		"exposed_copper_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([161, 125, 104]),
			},
			sign_material: None,
		},
	),
	(
		"exposed_cut_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([154, 121, 101]),
			},
			sign_material: None,
		},
	),
	(
		"exposed_cut_copper_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([154, 121, 101]),
			},
			sign_material: None,
		},
	),
	(
		"exposed_cut_copper_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([154, 121, 101]),
			},
			sign_material: None,
		},
	),
	(
		"farmland",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([81, 44, 15]),
			},
			sign_material: None,
		},
	),
	(
		"fern",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"fire",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([211, 140, 53]),
			},
			sign_material: None,
		},
	),
	(
		"fire_coral",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"fire_coral_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([163, 35, 46]),
			},
			sign_material: None,
		},
	),
	(
		"fire_coral_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"fire_coral_wall_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"fletching_table",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([197, 180, 133]),
			},
			sign_material: None,
		},
	),
	(
		"flower_pot",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([124, 68, 53]),
			},
			sign_material: None,
		},
	),
	(
		"flowering_azalea",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([112, 121, 64]),
			},
			sign_material: None,
		},
	),
	(
		"flowering_azalea_leaves",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([99, 111, 60]),
			},
			sign_material: None,
		},
	),
	(
		"frogspawn",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([105, 90, 82]),
			},
			sign_material: None,
		},
	),
	(
		"frosted_ice",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([140, 181, 252]),
			},
			sign_material: None,
		},
	),
	(
		"furnace",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([110, 109, 109]),
			},
			sign_material: None,
		},
	),
	(
		"gilded_blackstone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([55, 42, 38]),
			},
			sign_material: None,
		},
	),
	(
		"glass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([175, 213, 219]),
			},
			sign_material: None,
		},
	),
	(
		"glass_pane",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([170, 210, 217]),
			},
			sign_material: None,
		},
	),
	(
		"glow_item_frame",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"glow_lichen",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"glowstone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([171, 131, 84]),
			},
			sign_material: None,
		},
	),
	(
		"gold_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([246, 208, 61]),
			},
			sign_material: None,
		},
	),
	(
		"gold_ore",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([145, 133, 106]),
			},
			sign_material: None,
		},
	),
	(
		"granite",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([149, 103, 85]),
			},
			sign_material: None,
		},
	),
	(
		"granite_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([149, 103, 85]),
			},
			sign_material: None,
		},
	),
	(
		"granite_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([149, 103, 85]),
			},
			sign_material: None,
		},
	),
	(
		"granite_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([149, 103, 85]),
			},
			sign_material: None,
		},
	),
	(
		"grass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"grass_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
				color: Color([147, 147, 147]),
			},
			sign_material: None,
		},
	),
	(
		"grass_path",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([148, 121, 65]),
			},
			sign_material: None,
		},
	),
	(
		"gravel",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([131, 127, 126]),
			},
			sign_material: None,
		},
	),
	(
		"gray_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"gray_bed",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"gray_candle",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"gray_candle_cake",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 222, 214]),
			},
			sign_material: None,
		},
	),
	(
		"gray_carpet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([62, 68, 71]),
			},
			sign_material: None,
		},
	),
	(
		"gray_concrete",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([54, 57, 61]),
			},
			sign_material: None,
		},
	),
	(
		"gray_concrete_powder",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([76, 81, 84]),
			},
			sign_material: None,
		},
	),
	(
		"gray_glazed_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([83, 90, 93]),
			},
			sign_material: None,
		},
	),
	(
		"gray_shulker_box",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([55, 58, 62]),
			},
			sign_material: None,
		},
	),
	(
		"gray_stained_glass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([76, 76, 76]),
			},
			sign_material: None,
		},
	),
	(
		"gray_stained_glass_pane",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([73, 73, 73]),
			},
			sign_material: None,
		},
	),
	(
		"gray_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([57, 42, 35]),
			},
			sign_material: None,
		},
	),
	(
		"gray_wall_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"gray_wool",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([62, 68, 71]),
			},
			sign_material: None,
		},
	),
	(
		"green_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"green_bed",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"green_candle",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"green_candle_cake",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 222, 214]),
			},
			sign_material: None,
		},
	),
	(
		"green_carpet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([84, 109, 27]),
			},
			sign_material: None,
		},
	),
	(
		"green_concrete",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([73, 91, 36]),
			},
			sign_material: None,
		},
	),
	(
		"green_concrete_powder",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([97, 119, 44]),
			},
			sign_material: None,
		},
	),
	(
		"green_glazed_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([117, 142, 67]),
			},
			sign_material: None,
		},
	),
	(
		"green_shulker_box",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([79, 100, 31]),
			},
			sign_material: None,
		},
	),
	(
		"green_stained_glass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([102, 127, 51]),
			},
			sign_material: None,
		},
	),
	(
		"green_stained_glass_pane",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([97, 122, 48]),
			},
			sign_material: None,
		},
	),
	(
		"green_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([76, 83, 42]),
			},
			sign_material: None,
		},
	),
	(
		"green_wall_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"green_wool",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([84, 109, 27]),
			},
			sign_material: None,
		},
	),
	(
		"grindstone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([142, 142, 142]),
			},
			sign_material: None,
		},
	),
	(
		"hanging_roots",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([161, 115, 91]),
			},
			sign_material: None,
		},
	),
	(
		"hay_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([165, 139, 12]),
			},
			sign_material: None,
		},
	),
	(
		"heavy_core",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([82, 86, 94]),
			},
			sign_material: None,
		},
	),
	(
		"heavy_weighted_pressure_plate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([220, 220, 220]),
			},
			sign_material: None,
		},
	),
	(
		"honey_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([251, 185, 52]),
			},
			sign_material: None,
		},
	),
	(
		"honeycomb_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([229, 148, 29]),
			},
			sign_material: None,
		},
	),
	(
		"hopper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([75, 74, 75]),
			},
			sign_material: None,
		},
	),
	(
		"horn_coral",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"horn_coral_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([216, 199, 66]),
			},
			sign_material: None,
		},
	),
	(
		"horn_coral_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"horn_coral_wall_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"ice",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([145, 183, 253]),
			},
			sign_material: None,
		},
	),
	(
		"infested_chiseled_stone_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([119, 118, 119]),
			},
			sign_material: None,
		},
	),
	(
		"infested_cobblestone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([127, 127, 127]),
			},
			sign_material: None,
		},
	),
	(
		"infested_cracked_stone_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([118, 117, 118]),
			},
			sign_material: None,
		},
	),
	(
		"infested_deepslate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([80, 80, 82]),
			},
			sign_material: None,
		},
	),
	(
		"infested_mossy_stone_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([115, 121, 105]),
			},
			sign_material: None,
		},
	),
	(
		"infested_stone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([125, 125, 125]),
			},
			sign_material: None,
		},
	),
	(
		"infested_stone_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([122, 121, 122]),
			},
			sign_material: None,
		},
	),
	(
		"iron_bars",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([136, 139, 135]),
			},
			sign_material: None,
		},
	),
	(
		"iron_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([220, 220, 220]),
			},
			sign_material: None,
		},
	),
	(
		"iron_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([194, 193, 193]),
			},
			sign_material: None,
		},
	),
	(
		"iron_ore",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([136, 129, 122]),
			},
			sign_material: None,
		},
	),
	(
		"iron_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([202, 202, 202]),
			},
			sign_material: None,
		},
	),
	(
		"item_frame",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"jack_o_lantern",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([214, 152, 52]),
			},
			sign_material: None,
		},
	),
	(
		"jigsaw",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([80, 69, 81]),
			},
			sign_material: None,
		},
	),
	(
		"jukebox",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([93, 64, 47]),
			},
			sign_material: None,
		},
	),
	(
		"jungle_button",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"jungle_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([163, 119, 84]),
			},
			sign_material: None,
		},
	),
	(
		"jungle_fence",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([160, 115, 80]),
			},
			sign_material: None,
		},
	),
	(
		"jungle_fence_gate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([160, 115, 80]),
			},
			sign_material: None,
		},
	),
	(
		"jungle_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("jungle"),
		},
	),
	(
		"jungle_leaves",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Foliage}),
				color: Color([156, 154, 143]),
			},
			sign_material: None,
		},
	),
	(
		"jungle_log",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([149, 109, 70]),
			},
			sign_material: None,
		},
	),
	(
		"jungle_planks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([160, 115, 80]),
			},
			sign_material: None,
		},
	),
	(
		"jungle_pressure_plate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([160, 115, 80]),
			},
			sign_material: None,
		},
	),
	(
		"jungle_sapling",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([47, 81, 16]),
			},
			sign_material: None,
		},
	),
	(
		"jungle_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("jungle"),
		},
	),
	(
		"jungle_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([160, 115, 80]),
			},
			sign_material: None,
		},
	),
	(
		"jungle_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([160, 115, 80]),
			},
			sign_material: None,
		},
	),
	(
		"jungle_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([152, 110, 77]),
			},
			sign_material: None,
		},
	),
	(
		"jungle_wall_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("jungle"),
		},
	),
	(
		"jungle_wall_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("jungle"),
		},
	),
	(
		"jungle_wood",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([85, 67, 25]),
			},
			sign_material: None,
		},
	),
	(
		"kelp",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"kelp_plant",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([86, 130, 42]),
			},
			sign_material: None,
		},
	),
	(
		"ladder",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"lantern",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([106, 91, 83]),
			},
			sign_material: None,
		},
	),
	(
		"lapis_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([30, 67, 140]),
			},
			sign_material: None,
		},
	),
	(
		"lapis_ore",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([107, 117, 141]),
			},
			sign_material: None,
		},
	),
	(
		"large_amethyst_bud",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"large_fern",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
				color: Color([125, 125, 125]),
			},
			sign_material: None,
		},
	),
	(
		"lava",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([212, 90, 18]),
			},
			sign_material: None,
		},
	),
	(
		"lava_cauldron",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([73, 72, 74]),
			},
			sign_material: None,
		},
	),
	(
		"lectern",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([173, 137, 83]),
			},
			sign_material: None,
		},
	),
	(
		"lever",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"light",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"light_blue_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"light_blue_bed",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"light_blue_candle",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"light_blue_candle_cake",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 222, 214]),
			},
			sign_material: None,
		},
	),
	(
		"light_blue_carpet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([58, 175, 217]),
			},
			sign_material: None,
		},
	),
	(
		"light_blue_concrete",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([35, 137, 198]),
			},
			sign_material: None,
		},
	),
	(
		"light_blue_concrete_powder",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([74, 180, 213]),
			},
			sign_material: None,
		},
	),
	(
		"light_blue_glazed_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([94, 164, 208]),
			},
			sign_material: None,
		},
	),
	(
		"light_blue_shulker_box",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([49, 163, 212]),
			},
			sign_material: None,
		},
	),
	(
		"light_blue_stained_glass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([102, 153, 216]),
			},
			sign_material: None,
		},
	),
	(
		"light_blue_stained_glass_pane",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([97, 147, 208]),
			},
			sign_material: None,
		},
	),
	(
		"light_blue_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([113, 108, 137]),
			},
			sign_material: None,
		},
	),
	(
		"light_blue_wall_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"light_blue_wool",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([58, 175, 217]),
			},
			sign_material: None,
		},
	),
	(
		"light_gray_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"light_gray_bed",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"light_gray_candle",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"light_gray_candle_cake",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 222, 214]),
			},
			sign_material: None,
		},
	),
	(
		"light_gray_carpet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([142, 142, 134]),
			},
			sign_material: None,
		},
	),
	(
		"light_gray_concrete",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([125, 125, 115]),
			},
			sign_material: None,
		},
	),
	(
		"light_gray_concrete_powder",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([154, 154, 148]),
			},
			sign_material: None,
		},
	),
	(
		"light_gray_glazed_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([144, 166, 167]),
			},
			sign_material: None,
		},
	),
	(
		"light_gray_shulker_box",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([124, 124, 115]),
			},
			sign_material: None,
		},
	),
	(
		"light_gray_stained_glass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([153, 153, 153]),
			},
			sign_material: None,
		},
	),
	(
		"light_gray_stained_glass_pane",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([147, 147, 147]),
			},
			sign_material: None,
		},
	),
	(
		"light_gray_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([135, 106, 97]),
			},
			sign_material: None,
		},
	),
	(
		"light_gray_wall_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"light_gray_wool",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([142, 142, 134]),
			},
			sign_material: None,
		},
	),
	(
		"light_weighted_pressure_plate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([246, 208, 61]),
			},
			sign_material: None,
		},
	),
	(
		"lightning_rod",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"lilac",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([154, 125, 147]),
			},
			sign_material: None,
		},
	),
	(
		"lily_of_the_valley",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"lily_pad",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
				color: Color([133, 133, 133]),
			},
			sign_material: None,
		},
	),
	(
		"lime_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"lime_bed",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"lime_candle",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"lime_candle_cake",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 222, 214]),
			},
			sign_material: None,
		},
	),
	(
		"lime_carpet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([112, 185, 25]),
			},
			sign_material: None,
		},
	),
	(
		"lime_concrete",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([94, 168, 24]),
			},
			sign_material: None,
		},
	),
	(
		"lime_concrete_powder",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([125, 189, 41]),
			},
			sign_material: None,
		},
	),
	(
		"lime_glazed_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([162, 197, 55]),
			},
			sign_material: None,
		},
	),
	(
		"lime_shulker_box",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([99, 172, 23]),
			},
			sign_material: None,
		},
	),
	(
		"lime_stained_glass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([127, 204, 25]),
			},
			sign_material: None,
		},
	),
	(
		"lime_stained_glass_pane",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([122, 196, 24]),
			},
			sign_material: None,
		},
	),
	(
		"lime_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([103, 117, 52]),
			},
			sign_material: None,
		},
	),
	(
		"lime_wall_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"lime_wool",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([112, 185, 25]),
			},
			sign_material: None,
		},
	),
	(
		"lodestone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([147, 149, 152]),
			},
			sign_material: None,
		},
	),
	(
		"loom",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([142, 119, 91]),
			},
			sign_material: None,
		},
	),
	(
		"magenta_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"magenta_bed",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"magenta_candle",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"magenta_candle_cake",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 222, 214]),
			},
			sign_material: None,
		},
	),
	(
		"magenta_carpet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([189, 68, 179]),
			},
			sign_material: None,
		},
	),
	(
		"magenta_concrete",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([169, 48, 159]),
			},
			sign_material: None,
		},
	),
	(
		"magenta_concrete_powder",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([192, 83, 184]),
			},
			sign_material: None,
		},
	),
	(
		"magenta_glazed_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([208, 100, 191]),
			},
			sign_material: None,
		},
	),
	(
		"magenta_shulker_box",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([173, 54, 163]),
			},
			sign_material: None,
		},
	),
	(
		"magenta_stained_glass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([178, 76, 216]),
			},
			sign_material: None,
		},
	),
	(
		"magenta_stained_glass_pane",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([171, 73, 208]),
			},
			sign_material: None,
		},
	),
	(
		"magenta_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([149, 88, 108]),
			},
			sign_material: None,
		},
	),
	(
		"magenta_wall_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"magenta_wool",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([189, 68, 179]),
			},
			sign_material: None,
		},
	),
	(
		"magma_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([142, 63, 31]),
			},
			sign_material: None,
		},
	),
	(
		"mangrove_button",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"mangrove_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([112, 48, 46]),
			},
			sign_material: None,
		},
	),
	(
		"mangrove_fence",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([117, 54, 48]),
			},
			sign_material: None,
		},
	),
	(
		"mangrove_fence_gate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([117, 54, 48]),
			},
			sign_material: None,
		},
	),
	(
		"mangrove_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("mangrove"),
		},
	),
	(
		"mangrove_leaves",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Foliage}),
				color: Color([129, 128, 128]),
			},
			sign_material: None,
		},
	),
	(
		"mangrove_log",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([102, 48, 42]),
			},
			sign_material: None,
		},
	),
	(
		"mangrove_planks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([117, 54, 48]),
			},
			sign_material: None,
		},
	),
	(
		"mangrove_pressure_plate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([117, 54, 48]),
			},
			sign_material: None,
		},
	),
	(
		"mangrove_propagule",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([96, 174, 83]),
			},
			sign_material: None,
		},
	),
	(
		"mangrove_roots",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([74, 59, 38]),
			},
			sign_material: None,
		},
	),
	(
		"mangrove_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("mangrove"),
		},
	),
	(
		"mangrove_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([117, 54, 48]),
			},
			sign_material: None,
		},
	),
	(
		"mangrove_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([117, 54, 48]),
			},
			sign_material: None,
		},
	),
	(
		"mangrove_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([110, 46, 42]),
			},
			sign_material: None,
		},
	),
	(
		"mangrove_wall_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("mangrove"),
		},
	),
	(
		"mangrove_wall_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("mangrove"),
		},
	),
	(
		"mangrove_wood",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([83, 66, 41]),
			},
			sign_material: None,
		},
	),
	(
		"medium_amethyst_bud",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"melon",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([111, 144, 30]),
			},
			sign_material: None,
		},
	),
	(
		"melon_stem",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
				color: Color([153, 153, 153]),
			},
			sign_material: None,
		},
	),
	(
		"moss_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([89, 109, 45]),
			},
			sign_material: None,
		},
	),
	(
		"moss_carpet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([89, 109, 45]),
			},
			sign_material: None,
		},
	),
	(
		"mossy_cobblestone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([110, 118, 94]),
			},
			sign_material: None,
		},
	),
	(
		"mossy_cobblestone_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([110, 118, 94]),
			},
			sign_material: None,
		},
	),
	(
		"mossy_cobblestone_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([110, 118, 94]),
			},
			sign_material: None,
		},
	),
	(
		"mossy_cobblestone_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([110, 118, 94]),
			},
			sign_material: None,
		},
	),
	(
		"mossy_stone_brick_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([115, 121, 105]),
			},
			sign_material: None,
		},
	),
	(
		"mossy_stone_brick_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([115, 121, 105]),
			},
			sign_material: None,
		},
	),
	(
		"mossy_stone_brick_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([115, 121, 105]),
			},
			sign_material: None,
		},
	),
	(
		"mossy_stone_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([115, 121, 105]),
			},
			sign_material: None,
		},
	),
	(
		"moving_piston",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"mud",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([60, 57, 60]),
			},
			sign_material: None,
		},
	),
	(
		"mud_brick_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([137, 103, 79]),
			},
			sign_material: None,
		},
	),
	(
		"mud_brick_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([137, 103, 79]),
			},
			sign_material: None,
		},
	),
	(
		"mud_brick_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([137, 103, 79]),
			},
			sign_material: None,
		},
	),
	(
		"mud_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([137, 103, 79]),
			},
			sign_material: None,
		},
	),
	(
		"muddy_mangrove_roots",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([70, 58, 45]),
			},
			sign_material: None,
		},
	),
	(
		"mushroom_stem",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([203, 196, 185]),
			},
			sign_material: None,
		},
	),
	(
		"mycelium",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([111, 98, 101]),
			},
			sign_material: None,
		},
	),
	(
		"nether_brick_fence",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([44, 21, 26]),
			},
			sign_material: None,
		},
	),
	(
		"nether_brick_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([44, 21, 26]),
			},
			sign_material: None,
		},
	),
	(
		"nether_brick_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([44, 21, 26]),
			},
			sign_material: None,
		},
	),
	(
		"nether_brick_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([44, 21, 26]),
			},
			sign_material: None,
		},
	),
	(
		"nether_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([44, 21, 26]),
			},
			sign_material: None,
		},
	),
	(
		"nether_gold_ore",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([115, 54, 42]),
			},
			sign_material: None,
		},
	),
	(
		"nether_portal",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([89, 11, 192]),
			},
			sign_material: None,
		},
	),
	(
		"nether_quartz_ore",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([117, 65, 62]),
			},
			sign_material: None,
		},
	),
	(
		"nether_sprouts",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([19, 151, 133]),
			},
			sign_material: None,
		},
	),
	(
		"nether_wart",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([111, 18, 19]),
			},
			sign_material: None,
		},
	),
	(
		"nether_wart_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([114, 2, 2]),
			},
			sign_material: None,
		},
	),
	(
		"netherite_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([66, 61, 63]),
			},
			sign_material: None,
		},
	),
	(
		"netherrack",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([97, 38, 38]),
			},
			sign_material: None,
		},
	),
	(
		"note_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([88, 58, 40]),
			},
			sign_material: None,
		},
	),
	(
		"oak_button",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"oak_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([140, 110, 66]),
			},
			sign_material: None,
		},
	),
	(
		"oak_fence",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([162, 130, 78]),
			},
			sign_material: None,
		},
	),
	(
		"oak_fence_gate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([162, 130, 78]),
			},
			sign_material: None,
		},
	),
	(
		"oak_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("oak"),
		},
	),
	(
		"oak_leaves",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Foliage}),
				color: Color([144, 144, 144]),
			},
			sign_material: None,
		},
	),
	(
		"oak_log",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([151, 121, 73]),
			},
			sign_material: None,
		},
	),
	(
		"oak_planks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([162, 130, 78]),
			},
			sign_material: None,
		},
	),
	(
		"oak_pressure_plate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([162, 130, 78]),
			},
			sign_material: None,
		},
	),
	(
		"oak_sapling",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([77, 106, 40]),
			},
			sign_material: None,
		},
	),
	(
		"oak_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("oak"),
		},
	),
	(
		"oak_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([162, 130, 78]),
			},
			sign_material: None,
		},
	),
	(
		"oak_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([162, 130, 78]),
			},
			sign_material: None,
		},
	),
	(
		"oak_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([124, 99, 56]),
			},
			sign_material: None,
		},
	),
	(
		"oak_wall_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("oak"),
		},
	),
	(
		"oak_wall_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("oak"),
		},
	),
	(
		"oak_wood",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([109, 85, 50]),
			},
			sign_material: None,
		},
	),
	(
		"observer",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([98, 98, 98]),
			},
			sign_material: None,
		},
	),
	(
		"obsidian",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([15, 10, 24]),
			},
			sign_material: None,
		},
	),
	(
		"ochre_froglight",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([250, 245, 206]),
			},
			sign_material: None,
		},
	),
	(
		"open_eyeblossom",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"orange_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"orange_bed",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"orange_candle",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"orange_candle_cake",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 222, 214]),
			},
			sign_material: None,
		},
	),
	(
		"orange_carpet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([240, 118, 19]),
			},
			sign_material: None,
		},
	),
	(
		"orange_concrete",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([224, 97, 0]),
			},
			sign_material: None,
		},
	),
	(
		"orange_concrete_powder",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([227, 131, 31]),
			},
			sign_material: None,
		},
	),
	(
		"orange_glazed_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([154, 147, 91]),
			},
			sign_material: None,
		},
	),
	(
		"orange_shulker_box",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([234, 106, 8]),
			},
			sign_material: None,
		},
	),
	(
		"orange_stained_glass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([216, 127, 51]),
			},
			sign_material: None,
		},
	),
	(
		"orange_stained_glass_pane",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([208, 122, 48]),
			},
			sign_material: None,
		},
	),
	(
		"orange_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([161, 83, 37]),
			},
			sign_material: None,
		},
	),
	(
		"orange_tulip",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"orange_wall_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"orange_wool",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([240, 118, 19]),
			},
			sign_material: None,
		},
	),
	(
		"oxeye_daisy",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"oxidized_chiseled_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([83, 161, 132]),
			},
			sign_material: None,
		},
	),
	(
		"oxidized_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([82, 162, 132]),
			},
			sign_material: None,
		},
	),
	(
		"oxidized_copper_bulb",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([70, 132, 109]),
			},
			sign_material: None,
		},
	),
	(
		"oxidized_copper_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([82, 160, 132]),
			},
			sign_material: None,
		},
	),
	(
		"oxidized_copper_grate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([82, 161, 131]),
			},
			sign_material: None,
		},
	),
	(
		"oxidized_copper_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([83, 161, 132]),
			},
			sign_material: None,
		},
	),
	(
		"oxidized_cut_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([79, 153, 126]),
			},
			sign_material: None,
		},
	),
	(
		"oxidized_cut_copper_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([79, 153, 126]),
			},
			sign_material: None,
		},
	),
	(
		"oxidized_cut_copper_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([79, 153, 126]),
			},
			sign_material: None,
		},
	),
	(
		"packed_ice",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([141, 180, 250]),
			},
			sign_material: None,
		},
	),
	(
		"packed_mud",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([142, 106, 79]),
			},
			sign_material: None,
		},
	),
	(
		"pale_hanging_moss",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"pale_moss_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([106, 112, 104]),
			},
			sign_material: None,
		},
	),
	(
		"pale_moss_carpet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([106, 112, 104]),
			},
			sign_material: None,
		},
	),
	(
		"pale_oak_button",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"pale_oak_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([216, 208, 206]),
			},
			sign_material: None,
		},
	),
	(
		"pale_oak_fence",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([227, 217, 216]),
			},
			sign_material: None,
		},
	),
	(
		"pale_oak_fence_gate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([227, 217, 216]),
			},
			sign_material: None,
		},
	),
	(
		"pale_oak_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("pale_oak"),
		},
	),
	(
		"pale_oak_leaves",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([116, 121, 114]),
			},
			sign_material: None,
		},
	),
	(
		"pale_oak_log",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([198, 189, 187]),
			},
			sign_material: None,
		},
	),
	(
		"pale_oak_planks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([227, 217, 216]),
			},
			sign_material: None,
		},
	),
	(
		"pale_oak_pressure_plate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([227, 217, 216]),
			},
			sign_material: None,
		},
	),
	(
		"pale_oak_sapling",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([109, 105, 99]),
			},
			sign_material: None,
		},
	),
	(
		"pale_oak_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("pale_oak"),
		},
	),
	(
		"pale_oak_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([227, 217, 216]),
			},
			sign_material: None,
		},
	),
	(
		"pale_oak_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([227, 217, 216]),
			},
			sign_material: None,
		},
	),
	(
		"pale_oak_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([229, 220, 218]),
			},
			sign_material: None,
		},
	),
	(
		"pale_oak_wall_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("pale_oak"),
		},
	),
	(
		"pale_oak_wall_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("pale_oak"),
		},
	),
	(
		"pale_oak_wood",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([87, 77, 75]),
			},
			sign_material: None,
		},
	),
	(
		"pearlescent_froglight",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([245, 240, 239]),
			},
			sign_material: None,
		},
	),
	(
		"peony",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([129, 126, 139]),
			},
			sign_material: None,
		},
	),
	(
		"petrified_oak_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([162, 130, 78]),
			},
			sign_material: None,
		},
	),
	(
		"piglin_head",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"piglin_wall_head",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"pink_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"pink_bed",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"pink_candle",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"pink_candle_cake",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 222, 214]),
			},
			sign_material: None,
		},
	),
	(
		"pink_carpet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([237, 141, 172]),
			},
			sign_material: None,
		},
	),
	(
		"pink_concrete",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([213, 101, 142]),
			},
			sign_material: None,
		},
	),
	(
		"pink_concrete_powder",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([228, 153, 181]),
			},
			sign_material: None,
		},
	),
	(
		"pink_glazed_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([235, 154, 181]),
			},
			sign_material: None,
		},
	),
	(
		"pink_petals",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"pink_shulker_box",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([230, 121, 157]),
			},
			sign_material: None,
		},
	),
	(
		"pink_stained_glass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([242, 127, 165]),
			},
			sign_material: None,
		},
	),
	(
		"pink_stained_glass_pane",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([233, 122, 159]),
			},
			sign_material: None,
		},
	),
	(
		"pink_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([161, 78, 78]),
			},
			sign_material: None,
		},
	),
	(
		"pink_tulip",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"pink_wall_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"pink_wool",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([237, 141, 172]),
			},
			sign_material: None,
		},
	),
	(
		"piston",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([109, 104, 96]),
			},
			sign_material: None,
		},
	),
	(
		"piston_head",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([153, 127, 85]),
			},
			sign_material: None,
		},
	),
	(
		"pitcher_crop",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([193, 165, 103]),
			},
			sign_material: None,
		},
	),
	(
		"pitcher_plant",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([122, 144, 189]),
			},
			sign_material: None,
		},
	),
	(
		"player_head",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"player_wall_head",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"podzol",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([91, 63, 24]),
			},
			sign_material: None,
		},
	),
	(
		"pointed_dripstone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([129, 102, 89]),
			},
			sign_material: None,
		},
	),
	(
		"polished_andesite",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([132, 134, 133]),
			},
			sign_material: None,
		},
	),
	(
		"polished_andesite_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([132, 134, 133]),
			},
			sign_material: None,
		},
	),
	(
		"polished_andesite_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([132, 134, 133]),
			},
			sign_material: None,
		},
	),
	(
		"polished_basalt",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([99, 98, 100]),
			},
			sign_material: None,
		},
	),
	(
		"polished_blackstone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([53, 48, 56]),
			},
			sign_material: None,
		},
	),
	(
		"polished_blackstone_brick_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([48, 42, 49]),
			},
			sign_material: None,
		},
	),
	(
		"polished_blackstone_brick_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([48, 42, 49]),
			},
			sign_material: None,
		},
	),
	(
		"polished_blackstone_brick_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([48, 42, 49]),
			},
			sign_material: None,
		},
	),
	(
		"polished_blackstone_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([48, 42, 49]),
			},
			sign_material: None,
		},
	),
	(
		"polished_blackstone_button",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"polished_blackstone_pressure_plate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([53, 48, 56]),
			},
			sign_material: None,
		},
	),
	(
		"polished_blackstone_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([53, 48, 56]),
			},
			sign_material: None,
		},
	),
	(
		"polished_blackstone_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([53, 48, 56]),
			},
			sign_material: None,
		},
	),
	(
		"polished_blackstone_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([53, 48, 56]),
			},
			sign_material: None,
		},
	),
	(
		"polished_deepslate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([72, 72, 73]),
			},
			sign_material: None,
		},
	),
	(
		"polished_deepslate_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([72, 72, 73]),
			},
			sign_material: None,
		},
	),
	(
		"polished_deepslate_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([72, 72, 73]),
			},
			sign_material: None,
		},
	),
	(
		"polished_deepslate_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([72, 72, 73]),
			},
			sign_material: None,
		},
	),
	(
		"polished_diorite",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([192, 193, 194]),
			},
			sign_material: None,
		},
	),
	(
		"polished_diorite_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([192, 193, 194]),
			},
			sign_material: None,
		},
	),
	(
		"polished_diorite_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([192, 193, 194]),
			},
			sign_material: None,
		},
	),
	(
		"polished_granite",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([154, 106, 89]),
			},
			sign_material: None,
		},
	),
	(
		"polished_granite_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([154, 106, 89]),
			},
			sign_material: None,
		},
	),
	(
		"polished_granite_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([154, 106, 89]),
			},
			sign_material: None,
		},
	),
	(
		"polished_tuff",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([97, 104, 99]),
			},
			sign_material: None,
		},
	),
	(
		"polished_tuff_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([97, 104, 99]),
			},
			sign_material: None,
		},
	),
	(
		"polished_tuff_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([97, 104, 99]),
			},
			sign_material: None,
		},
	),
	(
		"polished_tuff_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([97, 104, 99]),
			},
			sign_material: None,
		},
	),
	(
		"poppy",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"potatoes",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([84, 135, 47]),
			},
			sign_material: None,
		},
	),
	(
		"potted_acacia_sapling",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([118, 117, 23]),
			},
			sign_material: None,
		},
	),
	(
		"potted_allium",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([158, 137, 183]),
			},
			sign_material: None,
		},
	),
	(
		"potted_azalea_bush",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([101, 124, 47]),
			},
			sign_material: None,
		},
	),
	(
		"potted_azure_bluet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([169, 204, 127]),
			},
			sign_material: None,
		},
	),
	(
		"potted_bamboo",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([93, 144, 19]),
			},
			sign_material: None,
		},
	),
	(
		"potted_birch_sapling",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([127, 160, 79]),
			},
			sign_material: None,
		},
	),
	(
		"potted_blue_orchid",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([47, 162, 168]),
			},
			sign_material: None,
		},
	),
	(
		"potted_brown_mushroom",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([153, 116, 92]),
			},
			sign_material: None,
		},
	),
	(
		"potted_cactus",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([85, 127, 43]),
			},
			sign_material: None,
		},
	),
	(
		"potted_cherry_sapling",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([164, 117, 143]),
			},
			sign_material: None,
		},
	),
	(
		"potted_closed_eyeblossom",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([108, 98, 101]),
			},
			sign_material: None,
		},
	),
	(
		"potted_cornflower",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([79, 121, 146]),
			},
			sign_material: None,
		},
	),
	(
		"potted_crimson_fungus",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([141, 44, 29]),
			},
			sign_material: None,
		},
	),
	(
		"potted_crimson_roots",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([127, 8, 41]),
			},
			sign_material: None,
		},
	),
	(
		"potted_dandelion",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([147, 172, 43]),
			},
			sign_material: None,
		},
	),
	(
		"potted_dark_oak_sapling",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([61, 90, 30]),
			},
			sign_material: None,
		},
	),
	(
		"potted_dead_bush",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([107, 78, 40]),
			},
			sign_material: None,
		},
	),
	(
		"potted_fern",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
				color: Color([124, 124, 124]),
			},
			sign_material: None,
		},
	),
	(
		"potted_flowering_azalea_bush",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([112, 121, 64]),
			},
			sign_material: None,
		},
	),
	(
		"potted_jungle_sapling",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([47, 81, 16]),
			},
			sign_material: None,
		},
	),
	(
		"potted_lily_of_the_valley",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([123, 174, 95]),
			},
			sign_material: None,
		},
	),
	(
		"potted_mangrove_propagule",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([96, 174, 83]),
			},
			sign_material: None,
		},
	),
	(
		"potted_oak_sapling",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([77, 106, 40]),
			},
			sign_material: None,
		},
	),
	(
		"potted_open_eyeblossom",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([133, 124, 127]),
			},
			sign_material: None,
		},
	),
	(
		"potted_orange_tulip",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([93, 142, 30]),
			},
			sign_material: None,
		},
	),
	(
		"potted_oxeye_daisy",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([179, 202, 143]),
			},
			sign_material: None,
		},
	),
	(
		"potted_pale_oak_sapling",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([109, 105, 99]),
			},
			sign_material: None,
		},
	),
	(
		"potted_pink_tulip",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([99, 157, 78]),
			},
			sign_material: None,
		},
	),
	(
		"potted_poppy",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([128, 64, 37]),
			},
			sign_material: None,
		},
	),
	(
		"potted_red_mushroom",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([216, 75, 67]),
			},
			sign_material: None,
		},
	),
	(
		"potted_red_tulip",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([89, 128, 32]),
			},
			sign_material: None,
		},
	),
	(
		"potted_spruce_sapling",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([44, 60, 36]),
			},
			sign_material: None,
		},
	),
	(
		"potted_torchflower",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([100, 101, 77]),
			},
			sign_material: None,
		},
	),
	(
		"potted_warped_fungus",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([74, 109, 87]),
			},
			sign_material: None,
		},
	),
	(
		"potted_warped_roots",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([20, 136, 123]),
			},
			sign_material: None,
		},
	),
	(
		"potted_white_tulip",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([93, 164, 71]),
			},
			sign_material: None,
		},
	),
	(
		"potted_wither_rose",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([41, 44, 23]),
			},
			sign_material: None,
		},
	),
	(
		"powder_snow",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 253, 253]),
			},
			sign_material: None,
		},
	),
	(
		"powder_snow_cauldron",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([73, 72, 74]),
			},
			sign_material: None,
		},
	),
	(
		"powered_rail",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([137, 109, 74]),
			},
			sign_material: None,
		},
	),
	(
		"prismarine",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([99, 156, 151]),
			},
			sign_material: None,
		},
	),
	(
		"prismarine_brick_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([99, 171, 158]),
			},
			sign_material: None,
		},
	),
	(
		"prismarine_brick_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([99, 171, 158]),
			},
			sign_material: None,
		},
	),
	(
		"prismarine_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([99, 171, 158]),
			},
			sign_material: None,
		},
	),
	(
		"prismarine_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([99, 156, 151]),
			},
			sign_material: None,
		},
	),
	(
		"prismarine_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([99, 156, 151]),
			},
			sign_material: None,
		},
	),
	(
		"prismarine_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([99, 156, 151]),
			},
			sign_material: None,
		},
	),
	(
		"pumpkin",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([198, 118, 24]),
			},
			sign_material: None,
		},
	),
	(
		"pumpkin_stem",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
				color: Color([154, 154, 154]),
			},
			sign_material: None,
		},
	),
	(
		"purple_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"purple_bed",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"purple_candle",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"purple_candle_cake",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 222, 214]),
			},
			sign_material: None,
		},
	),
	(
		"purple_carpet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([121, 42, 172]),
			},
			sign_material: None,
		},
	),
	(
		"purple_concrete",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([100, 31, 156]),
			},
			sign_material: None,
		},
	),
	(
		"purple_concrete_powder",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([131, 55, 177]),
			},
			sign_material: None,
		},
	),
	(
		"purple_glazed_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([109, 47, 152]),
			},
			sign_material: None,
		},
	),
	(
		"purple_shulker_box",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([103, 32, 156]),
			},
			sign_material: None,
		},
	),
	(
		"purple_stained_glass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([127, 63, 178]),
			},
			sign_material: None,
		},
	),
	(
		"purple_stained_glass_pane",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([122, 61, 171]),
			},
			sign_material: None,
		},
	),
	(
		"purple_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([118, 70, 86]),
			},
			sign_material: None,
		},
	),
	(
		"purple_wall_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"purple_wool",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([121, 42, 172]),
			},
			sign_material: None,
		},
	),
	(
		"purpur_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([169, 125, 169]),
			},
			sign_material: None,
		},
	),
	(
		"purpur_pillar",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([171, 129, 171]),
			},
			sign_material: None,
		},
	),
	(
		"purpur_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([169, 125, 169]),
			},
			sign_material: None,
		},
	),
	(
		"purpur_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([169, 125, 169]),
			},
			sign_material: None,
		},
	),
	(
		"quartz_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([235, 229, 222]),
			},
			sign_material: None,
		},
	),
	(
		"quartz_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([234, 229, 221]),
			},
			sign_material: None,
		},
	),
	(
		"quartz_pillar",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([235, 230, 224]),
			},
			sign_material: None,
		},
	),
	(
		"quartz_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([235, 229, 222]),
			},
			sign_material: None,
		},
	),
	(
		"quartz_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([235, 229, 222]),
			},
			sign_material: None,
		},
	),
	(
		"rail",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([125, 111, 88]),
			},
			sign_material: None,
		},
	),
	(
		"raw_copper_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([154, 105, 79]),
			},
			sign_material: None,
		},
	),
	(
		"raw_gold_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([221, 169, 46]),
			},
			sign_material: None,
		},
	),
	(
		"raw_iron_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([166, 135, 107]),
			},
			sign_material: None,
		},
	),
	(
		"red_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"red_bed",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"red_candle",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"red_candle_cake",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 222, 214]),
			},
			sign_material: None,
		},
	),
	(
		"red_carpet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([160, 39, 34]),
			},
			sign_material: None,
		},
	),
	(
		"red_concrete",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([142, 32, 32]),
			},
			sign_material: None,
		},
	),
	(
		"red_concrete_powder",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([168, 54, 50]),
			},
			sign_material: None,
		},
	),
	(
		"red_glazed_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([181, 59, 53]),
			},
			sign_material: None,
		},
	),
	(
		"red_mushroom",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"red_mushroom_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([200, 46, 45]),
			},
			sign_material: None,
		},
	),
	(
		"red_nether_brick_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([69, 7, 9]),
			},
			sign_material: None,
		},
	),
	(
		"red_nether_brick_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([69, 7, 9]),
			},
			sign_material: None,
		},
	),
	(
		"red_nether_brick_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([69, 7, 9]),
			},
			sign_material: None,
		},
	),
	(
		"red_nether_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([69, 7, 9]),
			},
			sign_material: None,
		},
	),
	(
		"red_sand",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([190, 102, 33]),
			},
			sign_material: None,
		},
	),
	(
		"red_sandstone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([181, 97, 31]),
			},
			sign_material: None,
		},
	),
	(
		"red_sandstone_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([181, 97, 31]),
			},
			sign_material: None,
		},
	),
	(
		"red_sandstone_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([181, 97, 31]),
			},
			sign_material: None,
		},
	),
	(
		"red_sandstone_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([181, 97, 31]),
			},
			sign_material: None,
		},
	),
	(
		"red_shulker_box",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([140, 31, 30]),
			},
			sign_material: None,
		},
	),
	(
		"red_stained_glass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([153, 51, 51]),
			},
			sign_material: None,
		},
	),
	(
		"red_stained_glass_pane",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([147, 48, 48]),
			},
			sign_material: None,
		},
	),
	(
		"red_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([143, 61, 46]),
			},
			sign_material: None,
		},
	),
	(
		"red_tulip",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"red_wall_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"red_wool",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([160, 39, 34]),
			},
			sign_material: None,
		},
	),
	(
		"redstone_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([175, 24, 5]),
			},
			sign_material: None,
		},
	),
	(
		"redstone_lamp",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([95, 54, 30]),
			},
			sign_material: None,
		},
	),
	(
		"redstone_ore",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([140, 109, 109]),
			},
			sign_material: None,
		},
	),
	(
		"redstone_torch",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"redstone_wall_torch",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"redstone_wire",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([175, 24, 5]),
			},
			sign_material: None,
		},
	),
	(
		"reinforced_deepslate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([80, 82, 78]),
			},
			sign_material: None,
		},
	),
	(
		"repeater",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([160, 157, 156]),
			},
			sign_material: None,
		},
	),
	(
		"repeating_command_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([129, 111, 176]),
			},
			sign_material: None,
		},
	),
	(
		"resin_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([217, 99, 25]),
			},
			sign_material: None,
		},
	),
	(
		"resin_brick_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([205, 88, 24]),
			},
			sign_material: None,
		},
	),
	(
		"resin_brick_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([205, 88, 24]),
			},
			sign_material: None,
		},
	),
	(
		"resin_brick_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([205, 88, 24]),
			},
			sign_material: None,
		},
	),
	(
		"resin_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([205, 88, 24]),
			},
			sign_material: None,
		},
	),
	(
		"resin_clump",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"respawn_anchor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([75, 26, 144]),
			},
			sign_material: None,
		},
	),
	(
		"rooted_dirt",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([144, 103, 76]),
			},
			sign_material: None,
		},
	),
	(
		"rose_bush",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([131, 66, 37]),
			},
			sign_material: None,
		},
	),
	(
		"sand",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([219, 207, 163]),
			},
			sign_material: None,
		},
	),
	(
		"sandstone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([223, 214, 170]),
			},
			sign_material: None,
		},
	),
	(
		"sandstone_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([223, 214, 170]),
			},
			sign_material: None,
		},
	),
	(
		"sandstone_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([223, 214, 170]),
			},
			sign_material: None,
		},
	),
	(
		"sandstone_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([223, 214, 170]),
			},
			sign_material: None,
		},
	),
	(
		"scaffolding",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([170, 131, 73]),
			},
			sign_material: None,
		},
	),
	(
		"sculk",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([12, 29, 36]),
			},
			sign_material: None,
		},
	),
	(
		"sculk_catalyst",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([15, 31, 38]),
			},
			sign_material: None,
		},
	),
	(
		"sculk_sensor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([7, 70, 84]),
			},
			sign_material: None,
		},
	),
	(
		"sculk_shrieker",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([198, 205, 169]),
			},
			sign_material: None,
		},
	),
	(
		"sculk_vein",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([7, 48, 57]),
			},
			sign_material: None,
		},
	),
	(
		"sea_lantern",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([172, 199, 190]),
			},
			sign_material: None,
		},
	),
	(
		"sea_pickle",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([90, 97, 39]),
			},
			sign_material: None,
		},
	),
	(
		"seagrass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"short_grass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"shroomlight",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([240, 146, 70]),
			},
			sign_material: None,
		},
	),
	(
		"shulker_box",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([139, 96, 139]),
			},
			sign_material: None,
		},
	),
	(
		"sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("oak"),
		},
	),
	(
		"skeleton_skull",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"skeleton_wall_skull",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"slime_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([111, 192, 91]),
			},
			sign_material: None,
		},
	),
	(
		"small_amethyst_bud",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"small_dripleaf",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"smithing_table",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([57, 58, 70]),
			},
			sign_material: None,
		},
	),
	(
		"smoker",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([85, 83, 81]),
			},
			sign_material: None,
		},
	),
	(
		"smooth_basalt",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([72, 72, 78]),
			},
			sign_material: None,
		},
	),
	(
		"smooth_quartz",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([235, 229, 222]),
			},
			sign_material: None,
		},
	),
	(
		"smooth_quartz_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([235, 229, 222]),
			},
			sign_material: None,
		},
	),
	(
		"smooth_quartz_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([235, 229, 222]),
			},
			sign_material: None,
		},
	),
	(
		"smooth_red_sandstone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([181, 97, 31]),
			},
			sign_material: None,
		},
	),
	(
		"smooth_red_sandstone_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([181, 97, 31]),
			},
			sign_material: None,
		},
	),
	(
		"smooth_red_sandstone_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([181, 97, 31]),
			},
			sign_material: None,
		},
	),
	(
		"smooth_sandstone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([223, 214, 170]),
			},
			sign_material: None,
		},
	),
	(
		"smooth_sandstone_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([223, 214, 170]),
			},
			sign_material: None,
		},
	),
	(
		"smooth_sandstone_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([223, 214, 170]),
			},
			sign_material: None,
		},
	),
	(
		"smooth_stone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([158, 158, 158]),
			},
			sign_material: None,
		},
	),
	(
		"smooth_stone_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([158, 158, 158]),
			},
			sign_material: None,
		},
	),
	(
		"sniffer_egg",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([135, 105, 67]),
			},
			sign_material: None,
		},
	),
	(
		"snow",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([249, 254, 254]),
			},
			sign_material: None,
		},
	),
	(
		"snow_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([249, 254, 254]),
			},
			sign_material: None,
		},
	),
	(
		"soul_campfire",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([80, 204, 208]),
			},
			sign_material: None,
		},
	),
	(
		"soul_fire",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([51, 192, 197]),
			},
			sign_material: None,
		},
	),
	(
		"soul_lantern",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([71, 99, 114]),
			},
			sign_material: None,
		},
	),
	(
		"soul_sand",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([81, 62, 50]),
			},
			sign_material: None,
		},
	),
	(
		"soul_soil",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([75, 57, 46]),
			},
			sign_material: None,
		},
	),
	(
		"soul_torch",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"soul_wall_torch",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"spawner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([36, 46, 62]),
			},
			sign_material: None,
		},
	),
	(
		"sponge",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([195, 192, 74]),
			},
			sign_material: None,
		},
	),
	(
		"spore_blossom",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([206, 96, 158]),
			},
			sign_material: None,
		},
	),
	(
		"spruce_button",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"spruce_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([106, 80, 48]),
			},
			sign_material: None,
		},
	),
	(
		"spruce_fence",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([114, 84, 48]),
			},
			sign_material: None,
		},
	),
	(
		"spruce_fence_gate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([114, 84, 48]),
			},
			sign_material: None,
		},
	),
	(
		"spruce_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("spruce"),
		},
	),
	(
		"spruce_leaves",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Spruce}),
				color: Color([126, 126, 126]),
			},
			sign_material: None,
		},
	),
	(
		"spruce_log",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([108, 80, 46]),
			},
			sign_material: None,
		},
	),
	(
		"spruce_planks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([114, 84, 48]),
			},
			sign_material: None,
		},
	),
	(
		"spruce_pressure_plate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([114, 84, 48]),
			},
			sign_material: None,
		},
	),
	(
		"spruce_sapling",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([44, 60, 36]),
			},
			sign_material: None,
		},
	),
	(
		"spruce_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("spruce"),
		},
	),
	(
		"spruce_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([114, 84, 48]),
			},
			sign_material: None,
		},
	),
	(
		"spruce_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([114, 84, 48]),
			},
			sign_material: None,
		},
	),
	(
		"spruce_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([103, 79, 47]),
			},
			sign_material: None,
		},
	),
	(
		"spruce_wall_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("spruce"),
		},
	),
	(
		"spruce_wall_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("spruce"),
		},
	),
	(
		"spruce_wood",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([58, 37, 16]),
			},
			sign_material: None,
		},
	),
	(
		"sticky_piston",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([109, 104, 96]),
			},
			sign_material: None,
		},
	),
	(
		"stone",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([125, 125, 125]),
			},
			sign_material: None,
		},
	),
	(
		"stone_brick_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([122, 121, 122]),
			},
			sign_material: None,
		},
	),
	(
		"stone_brick_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([122, 121, 122]),
			},
			sign_material: None,
		},
	),
	(
		"stone_brick_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([122, 121, 122]),
			},
			sign_material: None,
		},
	),
	(
		"stone_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([122, 121, 122]),
			},
			sign_material: None,
		},
	),
	(
		"stone_button",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"stone_pressure_plate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([125, 125, 125]),
			},
			sign_material: None,
		},
	),
	(
		"stone_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([125, 125, 125]),
			},
			sign_material: None,
		},
	),
	(
		"stone_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([125, 125, 125]),
			},
			sign_material: None,
		},
	),
	(
		"stonecutter",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([123, 118, 111]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_acacia_log",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([166, 91, 51]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_acacia_wood",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([174, 92, 59]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_bamboo_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([178, 158, 72]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_birch_log",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([191, 171, 116]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_birch_wood",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([196, 176, 118]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_cherry_log",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([221, 164, 157]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_cherry_wood",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([215, 145, 148]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_crimson_hyphae",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([137, 57, 90]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_crimson_stem",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([121, 56, 82]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_dark_oak_log",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([65, 44, 22]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_dark_oak_wood",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([72, 56, 36]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_jungle_log",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([165, 122, 81]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_jungle_wood",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([171, 132, 84]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_mangrove_log",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([109, 43, 43]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_mangrove_wood",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([119, 54, 47]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_oak_log",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([160, 129, 77]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_oak_wood",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([177, 144, 86]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_pale_oak_log",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([235, 226, 225]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_pale_oak_wood",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([245, 238, 236]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_spruce_log",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([105, 80, 46]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_spruce_wood",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([115, 89, 52]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_warped_hyphae",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([57, 150, 147]),
			},
			sign_material: None,
		},
	),
	(
		"stripped_warped_stem",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([52, 128, 124]),
			},
			sign_material: None,
		},
	),
	(
		"structure_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([88, 74, 90]),
			},
			sign_material: None,
		},
	),
	(
		"structure_void",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"sugar_cane",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([148, 192, 101]),
			},
			sign_material: None,
		},
	),
	(
		"sunflower",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([246, 196, 54]),
			},
			sign_material: None,
		},
	),
	(
		"suspicious_gravel",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([129, 125, 124]),
			},
			sign_material: None,
		},
	),
	(
		"suspicious_sand",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([217, 204, 159]),
			},
			sign_material: None,
		},
	),
	(
		"sweet_berry_bush",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([68, 77, 50]),
			},
			sign_material: None,
		},
	),
	(
		"tall_grass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
				color: Color([151, 149, 151]),
			},
			sign_material: None,
		},
	),
	(
		"tall_seagrass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([59, 139, 14]),
			},
			sign_material: None,
		},
	),
	(
		"target",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([226, 170, 157]),
			},
			sign_material: None,
		},
	),
	(
		"terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([152, 94, 67]),
			},
			sign_material: None,
		},
	),
	(
		"tinted_glass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([44, 38, 46]),
			},
			sign_material: None,
		},
	),
	(
		"tnt",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([142, 62, 53]),
			},
			sign_material: None,
		},
	),
	(
		"torch",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"torchflower",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"torchflower_crop",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"trapped_chest",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([162, 130, 78]),
			},
			sign_material: None,
		},
	),
	(
		"trial_spawner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([56, 82, 98]),
			},
			sign_material: None,
		},
	),
	(
		"tripwire",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"tripwire_hook",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"tube_coral",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"tube_coral_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([49, 87, 206]),
			},
			sign_material: None,
		},
	),
	(
		"tube_coral_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"tube_coral_wall_fan",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"tuff",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([108, 109, 102]),
			},
			sign_material: None,
		},
	),
	(
		"tuff_brick_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([98, 102, 95]),
			},
			sign_material: None,
		},
	),
	(
		"tuff_brick_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([98, 102, 95]),
			},
			sign_material: None,
		},
	),
	(
		"tuff_brick_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([98, 102, 95]),
			},
			sign_material: None,
		},
	),
	(
		"tuff_bricks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([98, 102, 95]),
			},
			sign_material: None,
		},
	),
	(
		"tuff_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([108, 109, 102]),
			},
			sign_material: None,
		},
	),
	(
		"tuff_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([108, 109, 102]),
			},
			sign_material: None,
		},
	),
	(
		"tuff_wall",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([108, 109, 102]),
			},
			sign_material: None,
		},
	),
	(
		"turtle_egg",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([228, 226, 191]),
			},
			sign_material: None,
		},
	),
	(
		"twisting_vines",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([20, 143, 124]),
			},
			sign_material: None,
		},
	),
	(
		"twisting_vines_plant",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([20, 135, 122]),
			},
			sign_material: None,
		},
	),
	(
		"vault",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([54, 69, 79]),
			},
			sign_material: None,
		},
	),
	(
		"verdant_froglight",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([229, 244, 228]),
			},
			sign_material: None,
		},
	),
	(
		"vine",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Grass}),
				color: Color([116, 116, 116]),
			},
			sign_material: None,
		},
	),
	(
		"void_air",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"wall_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("oak"),
		},
	),
	(
		"wall_torch",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"warped_button",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"warped_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([44, 126, 120]),
			},
			sign_material: None,
		},
	),
	(
		"warped_fence",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([43, 104, 99]),
			},
			sign_material: None,
		},
	),
	(
		"warped_fence_gate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([43, 104, 99]),
			},
			sign_material: None,
		},
	),
	(
		"warped_fungus",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"warped_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("warped"),
		},
	),
	(
		"warped_hyphae",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([58, 58, 77]),
			},
			sign_material: None,
		},
	),
	(
		"warped_nylium",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([43, 114, 101]),
			},
			sign_material: None,
		},
	),
	(
		"warped_planks",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([43, 104, 99]),
			},
			sign_material: None,
		},
	),
	(
		"warped_pressure_plate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([43, 104, 99]),
			},
			sign_material: None,
		},
	),
	(
		"warped_roots",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([20, 138, 124]),
			},
			sign_material: None,
		},
	),
	(
		"warped_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("warped"),
		},
	),
	(
		"warped_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([43, 104, 99]),
			},
			sign_material: None,
		},
	),
	(
		"warped_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([43, 104, 99]),
			},
			sign_material: None,
		},
	),
	(
		"warped_stem",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([53, 109, 110]),
			},
			sign_material: None,
		},
	),
	(
		"warped_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([47, 119, 111]),
			},
			sign_material: None,
		},
	),
	(
		"warped_wall_hanging_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("warped"),
		},
	),
	(
		"warped_wall_sign",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{WallSign}),
				color: Color([0, 0, 0]),
			},
			sign_material: Some("warped"),
		},
	),
	(
		"warped_wart_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([22, 119, 121]),
			},
			sign_material: None,
		},
	),
	(
		"water",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque|Water}),
				color: Color([177, 177, 177]),
			},
			sign_material: None,
		},
	),
	(
		"water_cauldron",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([73, 72, 74]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_chiseled_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([184, 100, 73]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_copper_block",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([192, 107, 79]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_copper_bulb",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([156, 86, 57]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_copper_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([192, 109, 82]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_copper_grate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([191, 107, 79]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_copper_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([190, 106, 79]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_cut_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([191, 106, 80]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_cut_copper_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([191, 106, 80]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_cut_copper_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([191, 106, 80]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_exposed_chiseled_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([154, 119, 100]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_exposed_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([161, 125, 103]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_exposed_copper_bulb",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([135, 107, 89]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_exposed_copper_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([164, 123, 106]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_exposed_copper_grate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([161, 125, 104]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_exposed_copper_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([161, 125, 104]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_exposed_cut_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([154, 121, 101]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_exposed_cut_copper_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([154, 121, 101]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_exposed_cut_copper_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([154, 121, 101]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_oxidized_chiseled_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([83, 161, 132]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_oxidized_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([82, 162, 132]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_oxidized_copper_bulb",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([70, 132, 109]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_oxidized_copper_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([82, 160, 132]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_oxidized_copper_grate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([82, 161, 131]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_oxidized_copper_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([83, 161, 132]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_oxidized_cut_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([79, 153, 126]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_oxidized_cut_copper_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([79, 153, 126]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_oxidized_cut_copper_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([79, 153, 126]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_weathered_chiseled_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([104, 150, 111]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_weathered_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([108, 153, 110]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_weathered_copper_bulb",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([92, 126, 99]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_weathered_copper_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([110, 150, 109]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_weathered_copper_grate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([106, 152, 110]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_weathered_copper_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([108, 153, 110]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_weathered_cut_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([109, 145, 107]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_weathered_cut_copper_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([109, 145, 107]),
			},
			sign_material: None,
		},
	),
	(
		"waxed_weathered_cut_copper_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([109, 145, 107]),
			},
			sign_material: None,
		},
	),
	(
		"weathered_chiseled_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([104, 150, 111]),
			},
			sign_material: None,
		},
	),
	(
		"weathered_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([108, 153, 110]),
			},
			sign_material: None,
		},
	),
	(
		"weathered_copper_bulb",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([92, 126, 99]),
			},
			sign_material: None,
		},
	),
	(
		"weathered_copper_door",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([110, 150, 109]),
			},
			sign_material: None,
		},
	),
	(
		"weathered_copper_grate",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([106, 152, 110]),
			},
			sign_material: None,
		},
	),
	(
		"weathered_copper_trapdoor",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([108, 153, 110]),
			},
			sign_material: None,
		},
	),
	(
		"weathered_cut_copper",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([109, 145, 107]),
			},
			sign_material: None,
		},
	),
	(
		"weathered_cut_copper_slab",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([109, 145, 107]),
			},
			sign_material: None,
		},
	),
	(
		"weathered_cut_copper_stairs",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([109, 145, 107]),
			},
			sign_material: None,
		},
	),
	(
		"weeping_vines",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([104, 1, 0]),
			},
			sign_material: None,
		},
	),
	(
		"weeping_vines_plant",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([132, 16, 12]),
			},
			sign_material: None,
		},
	),
	(
		"wet_sponge",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([171, 181, 70]),
			},
			sign_material: None,
		},
	),
	(
		"wheat",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([166, 151, 73]),
			},
			sign_material: None,
		},
	),
	(
		"white_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"white_bed",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"white_candle",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"white_candle_cake",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 222, 214]),
			},
			sign_material: None,
		},
	),
	(
		"white_carpet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([233, 236, 236]),
			},
			sign_material: None,
		},
	),
	(
		"white_concrete",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([207, 213, 214]),
			},
			sign_material: None,
		},
	),
	(
		"white_concrete_powder",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([225, 227, 227]),
			},
			sign_material: None,
		},
	),
	(
		"white_glazed_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([188, 212, 202]),
			},
			sign_material: None,
		},
	),
	(
		"white_shulker_box",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([215, 220, 221]),
			},
			sign_material: None,
		},
	),
	(
		"white_stained_glass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([255, 255, 255]),
			},
			sign_material: None,
		},
	),
	(
		"white_stained_glass_pane",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([246, 246, 246]),
			},
			sign_material: None,
		},
	),
	(
		"white_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([209, 178, 161]),
			},
			sign_material: None,
		},
	),
	(
		"white_tulip",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"white_wall_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"white_wool",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([233, 236, 236]),
			},
			sign_material: None,
		},
	),
	(
		"wither_rose",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"wither_skeleton_skull",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"wither_skeleton_wall_skull",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"yellow_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"yellow_bed",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"yellow_candle",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"yellow_candle_cake",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 222, 214]),
			},
			sign_material: None,
		},
	),
	(
		"yellow_carpet",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 197, 39]),
			},
			sign_material: None,
		},
	),
	(
		"yellow_concrete",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([240, 175, 21]),
			},
			sign_material: None,
		},
	),
	(
		"yellow_concrete_powder",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([232, 199, 54]),
			},
			sign_material: None,
		},
	),
	(
		"yellow_glazed_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([234, 192, 88]),
			},
			sign_material: None,
		},
	),
	(
		"yellow_shulker_box",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 188, 29]),
			},
			sign_material: None,
		},
	),
	(
		"yellow_stained_glass",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([229, 229, 51]),
			},
			sign_material: None,
		},
	),
	(
		"yellow_stained_glass_pane",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([221, 221, 48]),
			},
			sign_material: None,
		},
	),
	(
		"yellow_terracotta",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([186, 133, 35]),
			},
			sign_material: None,
		},
	),
	(
		"yellow_wall_banner",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"yellow_wool",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{Opaque}),
				color: Color([248, 197, 39]),
			},
			sign_material: None,
		},
	),
	(
		"zombie_head",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
	(
		"zombie_wall_head",
		ConstBlockType {
			block_color: BlockColor {
				flags: make_bitflags!(BlockFlag::{}),
				color: Color([0, 0, 0]),
			},
			sign_material: None,
		},
	),
];
