// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include <cstdint>
#include <string>
#include <unordered_map>

namespace MinedMap {
namespace Resource {

#define BLOCK_OPAQUE	(1u << 0)
#define BLOCK_GRASS	(1u << 1)
#define BLOCK_FOLIAGE	(1u << 2)
#define BLOCK_BIRCH	(1u << 3)
#define BLOCK_SPRUCE	(1u << 4)
#define BLOCK_WATER	(1u << 5)

class BlockType {
private:
	static const std::unordered_map<std::string, BlockType> Types;

public:
	static const BlockType * lookup(const std::string &name);

	uint8_t flags;
	struct {
		uint8_t r, g, b;
	} color;
};


struct LegacyPalette {
	const BlockType *types[256][16];
};

extern const LegacyPalette LEGACY_BLOCK_TYPES;

}
}
