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


#include "Chunk.hpp"
#include "../Resource/BlockType.hpp"

#include <iostream>
#include <stdexcept>
#include <cstdlib>
#include <cstring>

#include <zlib.h>


namespace MinedMap {
namespace World {

Chunk::Chunk(const ChunkData *data) {
	level = assertValue(data->getRoot().get<NBT::CompoundTag>("Level"));

	std::shared_ptr<const NBT::ByteTag> lightPopulatedTag = level->get<NBT::ByteTag>("LightPopulated");
	bool lightPopulated = lightPopulatedTag && lightPopulatedTag->getValue();

	sections = assertValue(level->get<NBT::ListTag<NBT::CompoundTag>>("Sections"));
	maxY = (assertValue(sections->back()->get<NBT::ByteTag>("Y"))->getValue() + 1) * SIZE;


	std::shared_ptr<const NBT::ByteArrayTag> biomeTag = assertValue(level->get<NBT::ByteArrayTag>("Biomes"));
	if (biomeTag->getLength() != SIZE*SIZE)
		throw std::invalid_argument("corrupt biome data");

	biomes = biomeTag->getValue();

	blockIDs.reset(new uint8_t[maxY * SIZE * SIZE]);
	blockData.reset(new uint8_t[maxY * SIZE * SIZE / 2]);
	blockSkyLight.reset(new uint8_t[maxY * SIZE * SIZE / 2]);
	blockBlockLight.reset(new uint8_t[maxY * SIZE * SIZE / 2]);

	std::memset(blockIDs.get(), 0, maxY * SIZE * SIZE);
	std::memset(blockData.get(), 0, maxY * SIZE * SIZE / 2);
	std::memset(blockSkyLight.get(), 0xff, maxY * SIZE * SIZE / 2);
	std::memset(blockBlockLight.get(), 0, maxY * SIZE * SIZE / 2);


	for (auto &section : *sections) {
		std::shared_ptr<const NBT::ByteArrayTag> blocks = assertValue(section->get<NBT::ByteArrayTag>("Blocks"));
		std::shared_ptr<const NBT::ByteArrayTag> data = assertValue(section->get<NBT::ByteArrayTag>("Data"));
		size_t Y = assertValue(section->get<NBT::ByteTag>("Y"))->getValue();

		if (blocks->getLength() != SIZE*SIZE*SIZE || data->getLength() != SIZE*SIZE*SIZE/2)
			throw std::invalid_argument("corrupt chunk data");

		if (lightPopulated) {
			std::shared_ptr<const NBT::ByteArrayTag> blockLight = assertValue(section->get<NBT::ByteArrayTag>("BlockLight"));
			std::shared_ptr<const NBT::ByteArrayTag> skyLight = assertValue(section->get<NBT::ByteArrayTag>("SkyLight"));

			if (blockLight->getLength() != SIZE*SIZE*SIZE/2 || skyLight->getLength() != SIZE*SIZE*SIZE/2)
				throw std::invalid_argument("corrupt chunk data");

			std::memcpy(blockBlockLight.get() + Y*SIZE*SIZE*SIZE/2, blockLight->getValue(), SIZE*SIZE*SIZE/2);
			std::memcpy(blockSkyLight.get() + Y*SIZE*SIZE*SIZE/2, skyLight->getValue(), SIZE*SIZE*SIZE/2);
		}

		std::memcpy(blockIDs.get() + Y*SIZE*SIZE*SIZE, blocks->getValue(), SIZE*SIZE*SIZE);
		std::memcpy(blockData.get() + Y*SIZE*SIZE*SIZE/2, data->getValue(), SIZE*SIZE*SIZE/2);
	}
}

Block Chunk::getBlock(size_t x, size_t y, size_t z) const {
	size_t y2 = y;
	if (y2 < maxY-1)
		y2++;

	unsigned h;
	for (h = y; h > 0; h--) {
		uint8_t id2 = getBlockAt(x, h, z);
		if (id2 != 8 && id2 != 9)
			break;
	}

	return Block(
		getBlockAt(x, y, z),
		getDataAt(x, y, z),
		h,
		getBlockLightAt(x, y2, z),
		getSkyLightAt(x, y2, z),
		getBiomeAt(x, z)
	);
}

Chunk::Blocks Chunk::getTopLayer() const {
	size_t done = 0;
	Blocks ret;

	for (ssize_t y = maxY-1; y >= 0; y--) {
		if (done == SIZE*SIZE)
			break;

		for (size_t z = 0; z < SIZE; z++) {
			for (size_t x = 0; x < SIZE; x++) {
				if (ret.blocks[x][z].id)
					continue;

				uint8_t id = getBlockAt(x, y, z);
				uint8_t data = getDataAt(x, y, z);
				if (!Resource::BLOCK_TYPES[id][data].opaque)
					continue;

				ret.blocks[x][z] = getBlock(x, y, z);
				done++;
			}
		}
	}

	return ret;
}

}
}
