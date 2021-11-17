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
	// Maximum Y value
	static const y_idx_t MAXY = 256;

	// Since Minecraft 1.15, biome information is stored for
	// 4x4x4 block groups
	static const uint32_t BGROUP = 4;
	// Number of biome values in a chunk in x/z dimensions
	static const uint32_t BSIZE = SIZE / BGROUP;
	// Number of biome values in a chunk in y dimension
	static const uint32_t BMAXY = MAXY / BGROUP;

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
	std::shared_ptr<const NBT::CompoundTag> level;
	std::vector<std::unique_ptr<Section>> sections;

	std::shared_ptr<const NBT::ByteArrayTag> biomeBytes;
	std::shared_ptr<const NBT::IntArrayTag> biomeIntsPre115;
	std::shared_ptr<const NBT::IntArrayTag> biomeInts;

	bool getHeight(
		Height *height, const Section *section,
		block_idx_t x, block_idx_t y, block_idx_t z, int flags
	) const;

	const Resource::BlockType * getBlockStateAt(block_idx_t x, y_idx_t y, block_idx_t z) const {
		section_idx_t Y = y / SIZE;

		if (Y >= sections.size() || !sections[Y])
			return nullptr;

		return sections[Y]->getBlockStateAt(x, y % SIZE, z);
	}


public:
	Chunk(const ChunkData *data);

	const NBT::CompoundTag & getLevel() const {
		return *level;
	}

	uint8_t getBiome(block_idx_t x, y_idx_t y, block_idx_t z) const;
	Block getBlock(block_idx_t x, Height y, block_idx_t z) const;

	Heightmap getTopLayer(int flags) const;
};

}
}
