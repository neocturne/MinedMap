// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015-2018, Matthias Schiffer <mschiffer@universe-factory.net>
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
	static const size_t SIZE = Section::SIZE;
	static const size_t MAXY = 256;

	static const size_t BGROUP = 4;
	static const size_t BSIZE = SIZE / BGROUP;
	static const size_t BMAXY = MAXY / BGROUP;

	// Flags
	static const int WITH_DEPTH = (1 << 0);

	struct Height {
		unsigned y;
		unsigned depth;
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

	bool getHeight(Height *height, const Section *section, size_t x, size_t y, size_t z, int flags) const;

	const Resource::BlockType * getBlockStateAt(size_t x, size_t y, size_t z) const {
		size_t Y = y / SIZE;

		if (Y >= sections.size() || !sections[Y])
			return nullptr;

		return sections[Y]->getBlockStateAt(x, y % SIZE, z);
	}


public:
	Chunk(const ChunkData *data);

	const NBT::CompoundTag & getLevel() const {
		return *level;
	}

	uint8_t getBiome(size_t x, size_t y, size_t z) const;
	Block getBlock(size_t x, Height y, size_t z) const;

	Heightmap getTopLayer(int flags) const;
};

}
}
