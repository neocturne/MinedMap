/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
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
#include "../Buffer.hpp"
#include "../UniqueCPtr.hpp"
#include "../Util.hpp"
#include "../NBT/CompoundTag.hpp"
#include "../NBT/ListTag.hpp"
#include "../NBT/ByteTag.hpp"
#include "../NBT/ByteArrayTag.hpp"

#include <cstdint>
#include <tuple>


namespace MinedMap {
namespace World {

class Chunk {
public:
	static const size_t SIZE = 16;

	struct Blocks {
		Block blocks[SIZE][SIZE];
	};


private:
	size_t len;
	UniqueCPtr<uint8_t[]> data;

	std::shared_ptr<const NBT::CompoundTag> root;
	std::shared_ptr<const NBT::CompoundTag> level;
	std::shared_ptr<const NBT::ListTag<NBT::CompoundTag>> sections;


	unsigned maxY;

	std::unique_ptr<uint8_t[]> blockIDs;
	std::unique_ptr<uint8_t[]> blockData;
	std::unique_ptr<uint8_t[]> blockSkyLight;
	std::unique_ptr<uint8_t[]> blockBlockLight;
	const uint8_t *biomes;


	size_t getIndex(size_t x, size_t y, size_t z) const {
		if (x >= SIZE || y >= maxY || z >= SIZE)
			throw std::range_error("Chunk::getIndex(): bad coordinates");

		return SIZE*SIZE*y + SIZE*z + x;
	}

	uint8_t getHalf(const uint8_t *v, size_t x, size_t y, size_t z) const {
		size_t i = getIndex(x, y, z);

		if (i % 2)
			return (v[i/2] >> 4);
		else
			return (v[i/2] & 0xf);
	}

	uint8_t getBlockAt(size_t x, size_t y, size_t z) const {
		return blockIDs[getIndex(x, y, z)];
	}

	uint8_t getDataAt(size_t x, size_t y, size_t z) const {
		return getHalf(blockData.get(), x, y, z);
	}

	uint8_t getBlockLightAt(size_t x, size_t y, size_t z) const {
		return getHalf(blockBlockLight.get(), x, y, z);
	}

	uint8_t getSkyLightAt(size_t x, size_t y, size_t z) const {
		return getHalf(blockSkyLight.get(), x, y, z);
	}

	uint8_t getBiomeAt(size_t x, size_t z) const {
		return biomes[z*SIZE + x];
	}


	void inflateChunk(Buffer buffer);
	void parseChunk();
	void analyzeChunk();

public:
	Chunk(Buffer buffer);

	const NBT::CompoundTag & getLevel() const {
		return *level;
	}

	const NBT::ListTag<NBT::CompoundTag> & getSections() const {
		return *sections;
	}

	Block getBlock(size_t x, size_t y, size_t z) const;
	Blocks getTopLayer() const;
};

}
}
