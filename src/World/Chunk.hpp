/*
  Copyright (c) 2015-2018, Matthias Schiffer <mschiffer@universe-factory.net>
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

	struct Blocks {
		Block blocks[SIZE][SIZE];
	};

private:
	std::shared_ptr<const NBT::CompoundTag> level;
	std::vector<std::unique_ptr<Section>> sections;

	std::shared_ptr<const NBT::ByteArrayTag> biomeBytes;
	std::shared_ptr<const NBT::IntArrayTag> biomeIntsPre115;
	std::shared_ptr<const NBT::IntArrayTag> biomeInts;

	bool getBlock(Block *block, const Section *section, size_t x, size_t y, size_t z, uint8_t prev_light) const;

public:
	Chunk(const ChunkData *data);

	const NBT::CompoundTag & getLevel() const {
		return *level;
	}

	uint8_t getBiome(size_t x, size_t y, size_t z) const;

	const Resource::BlockType * getBlockStateAt(size_t x, size_t y, size_t z) const {
		size_t Y = y / SIZE;

		if (Y >= sections.size() || !sections[Y])
			return nullptr;

		return sections[Y]->getBlockStateAt(x, y % SIZE, z);
	}

	Blocks getTopLayer() const;
};

}
}
