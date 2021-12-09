// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015-2021, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include "Color.hpp"
#include "../Util.hpp"

#include <unordered_map>

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
	static const Biome *const Default;
	static const Biome *const Biomes[256];
	static const std::unordered_map<std::string, uint8_t> Names;

	Biome(float temp0, float rain0, FloatColor water0 = {0.247f, 0.463f, 0.894f})
	: temp(temp0), rain(rain0), water(water0) {}

	FloatColor getBlockColor(const BlockType *type, y_idx_t height) const;
};

}
}
