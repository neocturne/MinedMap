/*
  Copyright (c) 2015, 2018, Matthias Schiffer <mschiffer@universe-factory.net>
  Copyright (c) 2019, Roman Shishkin <spark@uwtech.org>
  All rights reserved.

  Redistribution and use in source and binary forms, with or without
  modification, are permitted provided that the following conditions are met:

    1. Redistributions of source code must retain the above copyright notice,
       this list of conditions and the following disclaimer.
    2. Redistributions in binary form must reproduce the above copyright notice,
       this list of conditions and the following disclaimer in the documentation
       and/or other materials provided with the distribution.

  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
  AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
  IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
  FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
  DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
  SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
  CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
  OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
  OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/


#include "Biome.hpp"

#include "BlockType.hpp"
#include "../Util.hpp"


namespace MinedMap {
namespace Resource {

static Biome::FloatColor operator+(const Biome::FloatColor &a, const Biome::FloatColor &b){
	return Biome::FloatColor {
		a.r+b.r,
		a.g+b.g,
		a.b+b.b,
	};
}

static Biome::FloatColor & operator*=(Biome::FloatColor &a, const Biome::FloatColor &b) {
	a.r *= b.r;
	a.g *= b.g;
	a.b *= b.b;

	return a;
}

static Biome::FloatColor operator*(float s, const Biome::FloatColor &c) {
	return Biome::FloatColor {
		s*c.r,
		s*c.g,
		s*c.b,
	};
}

static Biome::FloatColor colorFromParams(float temp, float rain, bool grass) {
	const Biome::FloatColor grassColors[3] = {
		{0.502f, 0.706f, 0.592f}, // lower right
		{0.247f, 0.012f, -0.259f}, // lower left - lower right
		{-0.471f, 0.086f, -0.133f}, // upper left - lower left
	};
	const Biome::FloatColor foliageColors[3] = {
		{0.376f, 0.631f, 0.482f}, // lower right
		{0.306f, 0.012f, -0.317f}, // lower left - lower right
		{-0.580f, 0.106f, -0.165f}, // upper left - lower left
	};

	const Biome::FloatColor *colors = grass ? grassColors : foliageColors;

	return colors[0] + temp*colors[1] + rain*colors[2];
}


Biome::FloatColor Biome::getGrassColor(float temp, float rain) const {
	return colorFromParams(temp, rain, true);
}

Biome::FloatColor Biome::getFoliageColor(float temp, float rain) const {
	return colorFromParams(temp, rain, false);
}

Biome::FloatColor Biome::getWaterColor(float, float) const {
	return {0.161f, 0.365f, 0.996f};
}


Color Biome::getBlockColor(const BlockType *type, unsigned height) const {
	Biome::FloatColor c = {
		float(type->color.r),
		float(type->color.g),
		float(type->color.b),
	};

	float t = clamp(temp - std::max(0.0f, (height-64)/600.0f), 0, 1);
	float r = clamp(rain, 0, 1) * t;

	if (type->flags & BLOCK_GRASS)
		c *= getGrassColor(t, r);
	if (type->flags & BLOCK_FOLIAGE)
		c *= getFoliageColor(t, r);
	if (type->flags & BLOCK_WATER)
		c *= getWaterColor(t, r);

	float h = 0.5f + height * 0.005f;

	return Color {
		uint8_t(clamp(c.r * h, 0, 255)),
		uint8_t(clamp(c.g * h, 0, 255)),
		uint8_t(clamp(c.b * h, 0, 255)),
		0xff,
	};
}


class SwampBiome : public Biome {
protected:
	virtual FloatColor getGrassColor(float, float) const {
		return {0.417f, 0.439f, 0.224f};
	}
	virtual FloatColor getFoliageColor(float temp, float rain) const {
		return getGrassColor(temp, rain);
	}
	virtual FloatColor getWaterColor(float, float) const {
		return {0.141f, 0.365f, 0.680f};
	}

public:
	SwampBiome(float temp0, float rain0) : Biome(temp0, rain0) {}
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


/* Values from https://github.com/erich666/Mineways/blob/master/Win/biomes.cpp */

static const Biome BiomeDefault(0.5f,  0.5f);
static const Biome BiomePlains(0.8f, 0.4f);
static const Biome BiomeDesert(2.0f, 0.0f);
static const Biome BiomeMountains(0.2f, 0.3f);
static const Biome BiomeForest(0.7f, 0.8f);
static const Biome BiomeTaiga(0.25f, 0.8f);
static const SwampBiome BiomeSwamp(0.8f, 0.9f);
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
static const Biome BiomeWarmOcean(0.8f,  0.5f);

extern const Biome *const BIOME_DEFAULT = &BiomeDefault;

const Biome *const BIOMES[256] = {
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
	/*  10 */ &BiomeFrozen, /* Frozen Ocean */
	/*  11 */ &BiomeFrozen, /* Frozen River */
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
	/*  45 */ &BiomeWarmOcean, /* Lukewarm Ocean */
	/*  46 */ &BiomeWarmOcean, /* Cold Ocean */
	/*  47 */ &BiomeWarmOcean, /* Deep Warm Ocean */
	/*  48 */ &BiomeWarmOcean, /* Deep Lukewarm Ocean */
	/*  49 */ &BiomeWarmOcean, /* Deep Cold Ocean */
	/*  50 */ &BiomeWarmOcean, /* Deep Frozen Ocean */
	/*  51 */ nullptr,
	/*  52 */ nullptr,
	/*  53 */ nullptr,
	/*  54 */ nullptr,
	/*  55 */ nullptr,
	/*  56 */ nullptr,
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
};

}
}
