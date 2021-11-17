// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include "Color.hpp"


namespace MinedMap {
namespace Resource {

class BlockType;

class Biome {
private:
	float temp, rain;
	FloatColor water;

protected:
	virtual FloatColor getGrassColor(float temp, float rain) const;
	virtual FloatColor getFoliageColor(float temp, float rain) const;

	FloatColor getWaterColor() const { return water; };

public:
	Biome(float temp0, float rain0, FloatColor water0 = {0.247f, 0.463f, 0.894f})
	: temp(temp0), rain(rain0), water(water0) {}

	FloatColor getBlockColor(const BlockType *type, unsigned height) const;
};

extern const Biome *const BIOME_DEFAULT;
extern const Biome *const BIOMES[256];

}
}
