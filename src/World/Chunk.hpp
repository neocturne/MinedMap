// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015-2021, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once


#include "Block.hpp"
#include "ChunkData.hpp"
#include "Section.hpp"
#include "../NBT/ByteArrayTag.hpp"
#include "../NBT/IntArrayTag.hpp"
#include "../Resource/BlockType.hpp"
#include "../Util.hpp"

#include <cstdint>


namespace MinedMap {
namespace World {

class Chunk {
public:
	// Number of blocks in a chunk in x/z dimensions
	static const uint32_t SIZE = Section::SIZE;
	// Maximum Y value for pre-1.18 chunks
	static const y_idx_t MAXY = 256;

	// Shift to get from height to section index
	static const unsigned HSHIFT = 4;
	// Mask to get from height to y index inside section
	static const block_idx_t HMASK = 0xf;

	// Since Minecraft 1.15, biome information is stored for
	// 4x4x4 block groups
	static const unsigned BSHIFT = 2;
	// Number of biome values in a chunk in x/z dimensions
	static const uint32_t BSIZE = SIZE >> BSHIFT;
	// Number of biome values in a chunk in y dimension
	static const uint32_t BMAXY = MAXY >> BSHIFT;

	// Flags
	static const int WITH_DEPTH = (1 << 0);

	struct Height {
		y_idx_t y;
		y_idx_t depth;
	};

	struct Heightmap {
		Height v[SIZE][SIZE];
	};

private:
	section_idx_t sectionOffset;
	std::vector<std::unique_ptr<Section>> sections;

	enum BiomeFormat {
		UNKNOWN = 0,
		BYTE_ARRAY,
		INT_ARRAY_PRE1_15,
		INT_ARRAY,
		SECTION,
	} biomeFormat = UNKNOWN;

	std::shared_ptr<const NBT::ByteArrayTag> biomeBytes;
	std::shared_ptr<const NBT::IntArrayTag> biomeInts;

	bool getHeight(
		Height *height, const Section *section,
		block_idx_t x, block_idx_t y, block_idx_t z, int flags
	) const;

	const Resource::BlockType * getBlockStateAt(block_idx_t x, y_idx_t y, block_idx_t z) const {
		section_idx_t Y = (y >> HSHIFT) - sectionOffset;

		if (Y < 0 || size_t(Y) >= sections.size() || !sections[Y])
			return nullptr;

		return sections[Y]->getBlockStateAt(x, y & HMASK, z);
	}


public:
	Chunk(const ChunkData *data);

	uint8_t getBiome(block_idx_t x, y_idx_t y, block_idx_t z) const;
	Block getBlock(block_idx_t x, Height y, block_idx_t z) const;

	Heightmap getTopLayer(int flags) const;
};

}
}
