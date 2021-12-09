// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015-2021, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include "../NBT/CompoundTag.hpp"
#include "../Resource/Biome.hpp"
#include "../Resource/BlockType.hpp"


namespace MinedMap {
namespace World {

struct Block {
	const Resource::BlockType *type;
	y_idx_t depth;
	uint8_t blockLight;

	bool isVisible() const {
		return type && (type->flags & BLOCK_OPAQUE);
	}

	Resource::FloatColor getColor(uint8_t biome) const {
		if (!isVisible())
			return Resource::FloatColor {};

		return (Resource::Biome::Biomes[biome] ?: Resource::Biome::Default)->getBlockColor(type, depth);
	}

	operator bool() const {
		return type;
	}
};

}
}
