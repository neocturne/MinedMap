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

	class Section {
	private:
		static size_t getIndex(size_t x, size_t y, size_t z) {
			if (x >= SIZE || y >= SIZE || z >= SIZE)
				throw std::range_error("Chunk::getIndex(): bad coordinates");

			return SIZE*SIZE*y + SIZE*z + x;
		}

		std::shared_ptr<const NBT::ByteArrayTag> blocks;
		std::shared_ptr<const NBT::ByteArrayTag> data;

		std::shared_ptr<const NBT::ByteArrayTag> blockLight;
		std::shared_ptr<const NBT::ByteArrayTag> skyLight;

		unsigned Y;

	public:
		Section(const NBT::CompoundTag &s) {
			blocks = assertValue(s.get<NBT::ByteArrayTag>("Blocks"));
			data = assertValue(s.get<NBT::ByteArrayTag>("Data"));
			blockLight = assertValue(s.get<NBT::ByteArrayTag>("BlockLight"));
			skyLight = assertValue(s.get<NBT::ByteArrayTag>("SkyLight"));

			if (blocks->getLength() != SIZE*SIZE*SIZE || data->getLength() != SIZE*SIZE*SIZE/2
			    || blockLight->getLength() != SIZE*SIZE*SIZE/2 || skyLight->getLength() != SIZE*SIZE*SIZE/2)
				throw std::invalid_argument("corrupt chunk data");

			Y = assertValue(s.get<NBT::ByteTag>("Y"))->getValue() * SIZE;
		}

		uint8_t getBlockAt(size_t x, size_t y, size_t z) const {
			return blocks->get(getIndex(x, y, z));
		}

		uint8_t getDataAt(size_t x, size_t y, size_t z) const {
			return data->getHalf(getIndex(x, y, z));
		}

		uint8_t getBlockLightAt(size_t x, size_t y, size_t z) const {
			return blockLight->getHalf(getIndex(x, y, z));
		}

		uint8_t getSkyLightAt(size_t x, size_t y, size_t z) const {
			return skyLight->getHalf(getIndex(x, y, z));
		}

		unsigned getY() const {
			return Y;
		}
	};

	struct Blocks {
		Block blocks[SIZE][SIZE];
	};

private:
	size_t len;
	UniqueCPtr<uint8_t[]> data;

	std::shared_ptr<const NBT::CompoundTag> root;
	std::shared_ptr<const NBT::CompoundTag> level;

	std::vector<Section> sections;

	unsigned maxY;

	std::unique_ptr<Block[]> blocks;

	Block & getBlock(size_t x, size_t y, size_t z) {
		return blocks[SIZE*SIZE*y + SIZE*z + x];
	}

	const Block & getBlock(size_t x, size_t y, size_t z) const {
		return blocks[SIZE*SIZE*y + SIZE*z + x];
	}

	void inflateChunk(Buffer buffer);
	void parseChunk();
	void analyzeChunk();

public:
	Chunk(Buffer buffer);

	const NBT::CompoundTag & getLevel() const {
		return *level;
	}

	const std::vector<Section> & getSections() const {
		return sections;
	}

	Blocks getTopLayer() const;
};

}
}
