// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015-2021, Matthias Schiffer <mschiffer@universe-factory.net>
  Copyright (c) 2019, Roman Shishkin <spark@uwtech.org>
  All rights reserved.
*/


#include "Biome.hpp"

#include "BlockType.hpp"

namespace MinedMap {
namespace Resource {

static FloatColor colorFromParams(float temp, float rain, bool grass) {
	const FloatColor grassColors[3] = {
		{0.502f, 0.706f, 0.592f}, // lower right
		{0.247f, 0.012f, -0.259f}, // lower left - lower right
		{-0.471f, 0.086f, -0.133f}, // upper left - lower left
	};
	const FloatColor foliageColors[3] = {
		{0.376f, 0.631f, 0.482f}, // lower right
		{0.306f, 0.012f, -0.317f}, // lower left - lower right
		{-0.580f, 0.106f, -0.165f}, // upper left - lower left
	};

	const FloatColor *colors = grass ? grassColors : foliageColors;

	return colors[0] + temp*colors[1] + rain*colors[2];
}


FloatColor Biome::getGrassColor(float temp, float rain) const {
	return colorFromParams(temp, rain, true);
}

FloatColor Biome::getFoliageColor(float temp, float rain) const {
	return colorFromParams(temp, rain, false);
}


FloatColor Biome::getBlockColor(const BlockType *type, y_idx_t height) const {
	FloatColor c = {
		float(type->color.r),
		float(type->color.g),
		float(type->color.b),
	};

	float t = clamp(temp - std::max(0.0f, (int(height)-64)/600.0f), 0, 1);
	float r = clamp(rain, 0, 1) * t;

	if (type->flags & BLOCK_GRASS)
		c *= getGrassColor(t, r);
	if (type->flags & BLOCK_FOLIAGE)
		c *= getFoliageColor(t, r);
	if (type->flags & BLOCK_BIRCH)
		c *= FloatColor {0.380f, 0.600f, 0.380f};
	if (type->flags & BLOCK_SPRUCE)
		c *= FloatColor {0.502f, 0.655f, 0.333f};
	if (type->flags & BLOCK_WATER)
		c *= getWaterColor();

	float h = 0.5f + height * 0.005f;

	c.r = clamp(c.r * h, 0, 255);
	c.g = clamp(c.g * h, 0, 255);
	c.b = clamp(c.b * h, 0, 255);

	return c;
}


class SwampBiome : public Biome {
protected:
	virtual FloatColor getGrassColor(float, float) const {
		return {0.417f, 0.439f, 0.224f};
	}
	virtual FloatColor getFoliageColor(float temp, float rain) const {
		return getGrassColor(temp, rain);
	}

public:
	SwampBiome(float temp0, float rain0, FloatColor water0) :
	Biome(temp0, rain0, water0) {}
};

class DarkForestBiome : public Biome {
private:
	const FloatColor darkGreen = {0.157f, 0.204f, 0.039f};

protected:
	virtual FloatColor getGrassColor(float temp, float rain) const {
		return 0.5 * (darkGreen + colorFromParams(temp, rain, true));
	}

	virtual FloatColor getFoliageColor(float temp, float rain) const {
		return 0.5 * (darkGreen + colorFromParams(temp, rain, false));
	}

public:
	DarkForestBiome(float temp0, float rain0) : Biome(temp0, rain0) {}
};

class BadlandsBiome : public Biome {
protected:
	virtual FloatColor getGrassColor(float, float) const {
		return {0.565f, 0.506f, 0.302f};
	}
	virtual FloatColor getFoliageColor(float, float) const {
		return {0.620f, 0.506f, 0.302f};
	}

public:
	BadlandsBiome(float temp0, float rain0) : Biome(temp0, rain0) {}
};


/* Values from https://github.com/erich666/Mineways/blob/master/Win/biomes.cpp or
 * extracted from Minecraft code decompiled using https://github.com/Hexeption/MCP-Reborn */

static const Biome BiomeDefault(0.5f,  0.5f);
static const Biome BiomePlains(0.8f, 0.4f);
static const Biome BiomeDesert(2.0f, 0.0f);
static const Biome BiomeMountains(0.2f, 0.3f);
static const Biome BiomeForest(0.7f, 0.8f);
static const Biome BiomeTaiga(0.25f, 0.8f);
static const SwampBiome BiomeSwamp(0.8f, 0.9f, {0.380f, 0.482f, 0.392f});
static const Biome BiomeFrozen(0.0f, 0.5f);
static const Biome BiomeMushroomFields(0.9f, 1.0f);
static const Biome BiomeJungle(0.95f, 0.9f);
static const Biome BiomeJungleEdge(0.95f, 0.8f);
static const Biome BiomeSnowyBeach(0.05f, 0.3f);
static const Biome BiomeBirchForest(0.6f, 0.6f);
static const DarkForestBiome BiomeDarkForest(0.7f, 0.8f);
static const Biome BiomeSnowyTaiga(-0.5f, 0.4f);
static const Biome BiomeGiantTreeTaiga(0.3f, 0.8f);
static const Biome BiomeSavanna(1.2f, 0.0f);
static const Biome BiomeSavannaPlateau(1.0f, 0.0f);
static const Biome BiomeShatteredSavanna(1.1f, 0.0f);
static const BadlandsBiome BiomeBadlands(2.0f, 0.0f);

static const Biome BiomeFrozenOcean(0.0f, 0.5f, {0.224f, 0.220f, 0.788f});
static const Biome BiomeWarmOcean(0.8f, 0.5f, {0.263f, 0.835f, 0.933f});
static const Biome BiomeLukewarmOcean(0.8f, 0.5f, {0.271f, 0.678f, 0.949f});
static const Biome BiomeColdOcean(0.8f, 0.5f, {0.239f, 0.341f, 0.839f});

static const Biome BiomeMeadow(0.5f, 0.8f);
static const Biome BiomeGrove(-0.2f, 0.8f);
static const Biome BiomeJaggedPeaks(-0.7f, 0.9f);
static const Biome BiomeStonyPeaks(1.0f, 0.3f);
static const Biome BiomeSnowySlopes(-0.3f, 0.9f);
static const SwampBiome BiomeMangroveSwamp(0.8f, 0.9f, {0.227f, 0.478f, 0.416f});

const Biome *const Biome::Default = &BiomeDefault;

/* Minecraft 1.18 does not use numerical IDs for biomes anymore.
 * Previously unused biome IDs are assigned to the new biome types of
 * Minecraft 1.18 for storage in MinedMap's biome data cache. */

const Biome *const Biome::Biomes[256] = {
	/*   0 */ &BiomeDefault, /* Ocean */
	/*   1 */ &BiomePlains,
	/*   2 */ &BiomeDesert,
	/*   3 */ &BiomeMountains,
	/*   4 */ &BiomeForest,
	/*   5 */ &BiomeTaiga,
	/*   6 */ &BiomeSwamp,
	/*   7 */ &BiomeDefault, /* River */
	/*   8 */ &BiomeDesert, /* Nether */
	/*   9 */ &BiomeDefault, /* The End */
	/*  10 */ &BiomeFrozenOcean,
	/*  11 */ &BiomeFrozenOcean, /* Frozen River */
	/*  12 */ &BiomeFrozen, /* Snowy Tundra */
	/*  13 */ &BiomeFrozen, /* Snowy Mountains */
	/*  14 */ &BiomeMushroomFields,
	/*  15 */ &BiomeMushroomFields, /* Mushroom Field Shore */
	/*  16 */ &BiomePlains, /* Beach */
	/*  17 */ &BiomeDesert, /* Desert Hills */
	/*  18 */ &BiomeForest, /* Wooded Hiils */
	/*  19 */ &BiomeTaiga, /* Taiga Hills */
	/*  20 */ &BiomeMountains, /* Moutain Edge */
	/*  21 */ &BiomeJungle,
	/*  22 */ &BiomeJungle, /* Jungle Hills */
	/*  23 */ &BiomeJungleEdge,
	/*  24 */ &BiomeDefault, /* Deep Ocean */
	/*  25 */ &BiomeMountains, /* Stone Shore */
	/*  26 */ &BiomeSnowyBeach,
	/*  27 */ &BiomeBirchForest,
	/*  28 */ &BiomeBirchForest, /* Birch Forest Hills */
	/*  29 */ &BiomeDarkForest,
	/*  30 */ &BiomeSnowyTaiga,
	/*  31 */ &BiomeSnowyTaiga, /* Snowy Taiga Hills */
	/*  32 */ &BiomeGiantTreeTaiga,
	/*  33 */ &BiomeGiantTreeTaiga, /* Giant Tree Taiga Hills */
	/*  34 */ &BiomeMountains, /* Wooded Mountains */
	/*  35 */ &BiomeSavanna,
	/*  36 */ &BiomeSavanna, /* Savanna Plateau */
	/*  37 */ &BiomeBadlands,
	/*  38 */ &BiomeBadlands, /* Wooded Badlands Plateau */
	/*  39 */ &BiomeBadlands, /* Badlands Plateau */
	/*  40 */ &BiomeDefault, /* Small End Islands */
	/*  41 */ &BiomeDefault, /* End Midlands */
	/*  42 */ &BiomeDefault, /* End Highlands */
	/*  43 */ &BiomeDefault, /* End Barrens */
	/*  44 */ &BiomeWarmOcean,
	/*  45 */ &BiomeLukewarmOcean,
	/*  46 */ &BiomeColdOcean,
	/*  47 */ &BiomeWarmOcean, /* Deep Warm Ocean */
	/*  48 */ &BiomeLukewarmOcean, /* Deep Lukewarm Ocean */
	/*  49 */ &BiomeColdOcean, /* Deep Cold Ocean */
	/*  50 */ &BiomeFrozenOcean, /* Deep Frozen Ocean */
	/*  51 */ &BiomeMeadow, /* MinedMap assignment */
	/*  52 */ &BiomeGrove, /* MinedMap assignment */
	/*  53 */ &BiomeJaggedPeaks, /* MinedMap assignment */
	/*  54 */ &BiomeStonyPeaks, /* MinedMap assignment */
	/*  55 */ &BiomeSnowySlopes, /* MinedMap assignment */
	/*  56 */ &BiomeMangroveSwamp, /* MinedMap assignment */
	/*  57 */ nullptr,
	/*  58 */ nullptr,
	/*  59 */ nullptr,
	/*  60 */ nullptr,
	/*  61 */ nullptr,
	/*  62 */ nullptr,
	/*  63 */ nullptr,
	/*  64 */ nullptr,
	/*  65 */ nullptr,
	/*  66 */ nullptr,
	/*  67 */ nullptr,
	/*  68 */ nullptr,
	/*  69 */ nullptr,
	/*  70 */ nullptr,
	/*  71 */ nullptr,
	/*  72 */ nullptr,
	/*  73 */ nullptr,
	/*  74 */ nullptr,
	/*  75 */ nullptr,
	/*  76 */ nullptr,
	/*  77 */ nullptr,
	/*  78 */ nullptr,
	/*  79 */ nullptr,
	/*  80 */ nullptr,
	/*  81 */ nullptr,
	/*  82 */ nullptr,
	/*  83 */ nullptr,
	/*  84 */ nullptr,
	/*  85 */ nullptr,
	/*  86 */ nullptr,
	/*  87 */ nullptr,
	/*  88 */ nullptr,
	/*  89 */ nullptr,
	/*  90 */ nullptr,
	/*  91 */ nullptr,
	/*  92 */ nullptr,
	/*  93 */ nullptr,
	/*  94 */ nullptr,
	/*  95 */ nullptr,
	/*  96 */ nullptr,
	/*  97 */ nullptr,
	/*  98 */ nullptr,
	/*  99 */ nullptr,
	/* 100 */ nullptr,
	/* 101 */ nullptr,
	/* 102 */ nullptr,
	/* 103 */ nullptr,
	/* 104 */ nullptr,
	/* 105 */ nullptr,
	/* 106 */ nullptr,
	/* 107 */ nullptr,
	/* 108 */ nullptr,
	/* 109 */ nullptr,
	/* 110 */ nullptr,
	/* 111 */ nullptr,
	/* 112 */ nullptr,
	/* 113 */ nullptr,
	/* 114 */ nullptr,
	/* 115 */ nullptr,
	/* 116 */ nullptr,
	/* 117 */ nullptr,
	/* 118 */ nullptr,
	/* 119 */ nullptr,
	/* 120 */ nullptr,
	/* 121 */ nullptr,
	/* 122 */ nullptr,
	/* 123 */ nullptr,
	/* 124 */ nullptr,
	/* 125 */ nullptr,
	/* 126 */ nullptr,
	/* 127 */ &BiomeDefault, /* The Void */
	/* 128 */ nullptr,
	/* 129 */ &BiomeDefault, /* Sunflower Plains */
	/* 130 */ &BiomeDesert, /* Desert Lakes */
	/* 131 */ &BiomeMountains, /* Gravelly Mountains */
	/* 132 */ &BiomeForest, /* Flower Forest */
	/* 133 */ &BiomeTaiga, /* Taiga Mountains */
	/* 134 */ &BiomeSwamp, /* Swamp Hills */
	/* 135 */ nullptr,
	/* 136 */ nullptr,
	/* 137 */ nullptr,
	/* 138 */ nullptr,
	/* 139 */ nullptr,
	/* 140 */ &BiomeFrozen, /* Ice Spikes */
	/* 141 */ nullptr,
	/* 142 */ nullptr,
	/* 143 */ nullptr,
	/* 144 */ nullptr,
	/* 145 */ nullptr,
	/* 146 */ nullptr,
	/* 147 */ nullptr,
	/* 148 */ nullptr,
	/* 149 */ &BiomeJungle, /* Modified Jungle */
	/* 150 */ nullptr,
	/* 151 */ &BiomeJungleEdge, /* Modified Jungle Edge */
	/* 152 */ nullptr,
	/* 153 */ nullptr,
	/* 154 */ nullptr,
	/* 155 */ &BiomeBirchForest, /* Tall Birch Forest */
	/* 156 */ &BiomeBirchForest, /* Tall Birch Hills */
	/* 157 */ &BiomeDarkForest, /* Dark Forest Hills */
	/* 158 */ &BiomeSnowyTaiga, /* Snowy Taiga Mountains */
	/* 159 */ nullptr,
	/* 160 */ &BiomeTaiga, /* Giant Spruce Taiga */
	/* 161 */ &BiomeTaiga, /* Giant Spruce Taiga Hills */
	/* 162 */ &BiomeMountains, /* Gravelly Mountains+ */
	/* 163 */ &BiomeShatteredSavanna,
	/* 164 */ &BiomeSavannaPlateau, /* Shattered Savanna Plateau */
	/* 165 */ &BiomeBadlands, /* Eroded Badlands */
	/* 166 */ &BiomeBadlands, /* Modified Wooded Badlands Plateau */
	/* 167 */ &BiomeBadlands, /* Modified Badlands Plateau */
	/* 168 */ &BiomeJungle, /* Bamboo Jungle */
	/* 169 */ &BiomeJungle, /* Bamboo Jungle Hills */
	/* 170 */ &BiomeDesert, /* Soul Sand Valley */
	/* 171 */ &BiomeDesert, /* Crimson Forest */
	/* 172 */ &BiomeDesert, /* Warped Forest */
	/* 173 */ &BiomeDesert, /* Basalt Deltas */
	/* 174 */ &BiomePlains, /* Dripstone Caves */
	/* 175 */ &BiomeDefault, /* Lush Caves */
};

/* It is unclear which of the renamed/merged biome IDs can appear in practice,
 * but it shouldn't hurt to support them anyways */

const std::unordered_map<std::string, uint8_t> Biome::Names = {
	{ "minecraft:badlands", 37 },
	{ "minecraft:badlands_plateau", 39 }, /* 1.18: Merged into badlands */
	{ "minecraft:bamboo_jungle", 168 },
	{ "minecraft:bamboo_jungle_hills", 169 }, /* 1.18: Merged into bamboo_jungle */
	{ "minecraft:basalt_deltas", 173 },
	{ "minecraft:beach", 16 },
	{ "minecraft:birch_forest", 27 },
	{ "minecraft:birch_forest_hills", 28 }, /* 1.18: Merged into birch_forest */
	{ "minecraft:cold_ocean", 46 },
	{ "minecraft:crimson_forest", 171 },
	{ "minecraft:dark_forest", 29 },
	{ "minecraft:dark_forest_hills", 157 }, /* 1.18: Merged into dark_forest */
	{ "minecraft:deep_cold_ocean", 49 },
	{ "minecraft:deep_dark", 1 },
	{ "minecraft:deep_frozen_ocean", 50 },
	{ "minecraft:deep_lukewarm_ocean", 48 },
	{ "minecraft:deep_ocean", 24 },
	{ "minecraft:deep_warm_ocean", 47 }, /* 1.18: Merged into warm_ocean */
	{ "minecraft:desert", 2 },
	{ "minecraft:desert_hills", 17 }, /* 1.18: Merged into desert */
	{ "minecraft:desert_lakes", 130 }, /* 1.18: Merged into desert */
	{ "minecraft:dripstone_caves", 174 },
	{ "minecraft:end_barrens", 43 },
	{ "minecraft:end_highlands", 42 },
	{ "minecraft:end_midlands", 41 },
	{ "minecraft:eroded_badlands", 165 },
	{ "minecraft:extreme_hills", 3 }, /* 1.18: Renamed to windswept_hills (after rename from mountains) */
	{ "minecraft:flower_forest", 132 },
	{ "minecraft:forest", 4 },
	{ "minecraft:frozen_ocean", 10 },
	{ "minecraft:frozen_peaks", 53 }, /* 1.18: New */
	{ "minecraft:frozen_river", 11 },
	{ "minecraft:giant_spruce_taiga", 160 }, /* 1.18: Renamed to old_growth_spruce_taiga */
	{ "minecraft:giant_spruce_taiga_hills", 161 }, /* 1.18: Merged into giant_spruce_taiga */
	{ "minecraft:giant_tree_taiga", 32 }, /* 1.18: Renamed to old_growth_pine_taiga */
	{ "minecraft:giant_tree_taiga_hills", 33 }, /* 1.18: Merged into giant_tree_taiga */
	{ "minecraft:gravelly_mountains", 131 }, /* 1.18: Renamed to windswept_gravelly_hills */
	{ "minecraft:grove", 52 }, /* 1.18: New */
	{ "minecraft:ice_spikes", 140 },
	{ "minecraft:jagged_peaks", 53 }, /* 1.18: New */
	{ "minecraft:jungle", 21 },
	{ "minecraft:jungle_edge", 23 }, /* 1.18: Renamed to sparse_jungle */
	{ "minecraft:jungle_hills", 22 }, /* 1.18: Merged into jungle */
	{ "minecraft:lukewarm_ocean", 45 },
	{ "minecraft:lush_caves", 175 },
	{ "minecraft:mangrove_swamp", 56 },
	{ "minecraft:meadow", 51 }, /* 1.18: New */
	{ "minecraft:modified_badlands_plateau", 167 }, /* 1.18: Merged into badlands */
	{ "minecraft:modified_gravelly_mountains", 162 }, /* 1.18: Merged into gravelly_mountains */
	{ "minecraft:modified_jungle", 149 }, /* 1.18: Merged into jungle */
	{ "minecraft:modified_jungle_edge", 151 }, /* 1.18: Merged into jungle_edge */
	{ "minecraft:modified_wooded_badlands_plateau", 166 }, /* 1.18: Merged into wooded_badlands */
	{ "minecraft:mountain_edge", 20 }, /* 1.18: Merged into mountains */
	{ "minecraft:mountains", 3 }, /* 1.18: Renamed to windswept_hills */
	{ "minecraft:mushroom_field_shore", 15 }, /* 1.18: Merged into mushroom_fields */
	{ "minecraft:mushroom_fields", 14 },
	{ "minecraft:nether_wastes", 8 },
	{ "minecraft:ocean", 0 },
	{ "minecraft:old_growth_birch_forest", 155 },
	{ "minecraft:old_growth_pine_taiga", 32 },
	{ "minecraft:old_growth_spruce_taiga", 160 },
	{ "minecraft:plains", 1 },
	{ "minecraft:river", 7 },
	{ "minecraft:savanna", 35 },
	{ "minecraft:savanna_plateau", 36 },
	{ "minecraft:shattered_savanna", 163 },
	{ "minecraft:shattered_savanna_plateau", 164 }, /* 1.18: Merged into shattered_savanna */
	{ "minecraft:small_end_islands", 40 },
	{ "minecraft:snowy_beach", 26 },
	{ "minecraft:snowy_mountains", 13 }, /* 1.18: Merged into snowy_tundra */
	{ "minecraft:snowy_plains", 12 },
	{ "minecraft:snowy_slopes", 55 },
	{ "minecraft:snowy_taiga", 30 },
	{ "minecraft:snowy_taiga_hills", 31 }, /* 1.18: Merged into snowy_taiga */
	{ "minecraft:snowy_taiga_mountains", 158 }, /* 1.18: Merged into snowy_taiga */
	{ "minecraft:snowy_tundra", 12 },
	{ "minecraft:soul_sand_valley", 170 },
	{ "minecraft:sparse_jungle", 23 },
	{ "minecraft:stone_shore", 25 },
	{ "minecraft:stony_peaks", 54 }, /* 1.18: New */
	{ "minecraft:stony_shore", 25 },
	{ "minecraft:sunflower_plains", 129 },
	{ "minecraft:swamp", 6 },
	{ "minecraft:swamp_hills", 134 }, /* 1.18: Merged into swamp */
	{ "minecraft:taiga", 5 },
	{ "minecraft:taiga_hills", 19 }, /* 1.18: Merged into taiga */
	{ "minecraft:taiga_mountains", 133 }, /* 1.18: Merged into taiga */
	{ "minecraft:tall_birch_forest", 155 }, /* 1.18: Renamed to old_growth_birch_forest */
	{ "minecraft:tall_birch_hills", 156 }, /* 1.18: Merged into tall_birch_forest */
	{ "minecraft:the_end", 9 },
	{ "minecraft:the_void", 127 },
	{ "minecraft:warm_ocean", 44 },
	{ "minecraft:warped_forest", 172 },
	{ "minecraft:windswept_forest", 34 },
	{ "minecraft:windswept_gravelly_hills", 131 },
	{ "minecraft:windswept_hills", 3 },
	{ "minecraft:windswept_savanna", 163 },
	{ "minecraft:wooded_badlands", 38 },
	{ "minecraft:wooded_badlands_plateau", 38 }, /* 1.18: Renamed to wooded_badlands */
	{ "minecraft:wooded_hills", 18 }, /* 1.18: Merged into forest */
	{ "minecraft:wooded_mountains", 34 /* 1.18: Renamed to windswept_forest */},
};

}
}
