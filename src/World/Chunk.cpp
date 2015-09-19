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


#include "Chunk.hpp"
#include "../Resource/BlockType.hpp"

#include <iostream>
#include <stdexcept>
#include <cstdlib>
#include <cstring>

#include <zlib.h>


namespace MinedMap {
namespace World {

Chunk::Chunk(Buffer buffer) {
	size_t size = buffer.get32();

	Buffer buffer2(buffer.get(size), size);

	uint8_t format = buffer2.get8();
	if (format != 2)
		throw std::invalid_argument("unknown chunk format");

	inflateChunk(buffer2);
	parseChunk();
	analyzeChunk();
}

void Chunk::inflateChunk(Buffer buffer) {
	size_t outlen = 0;
	uint8_t *output = nullptr;

	z_stream stream = {};
	int ret = inflateInit(&stream);
	if (ret != Z_OK)
		throw std::runtime_error("inflateInit() failed");

	stream.avail_in = buffer.getRemaining();
	stream.next_in = const_cast<uint8_t *>(buffer.get(stream.avail_in));

	while (stream.avail_in) {
		outlen += 65536;
		output = static_cast<uint8_t *>(std::realloc(output, outlen));

		stream.next_out = output + stream.total_out;
		stream.avail_out = outlen - stream.total_out;

		ret = inflate(&stream, Z_NO_FLUSH);
		switch (ret) {
		case Z_NEED_DICT:
		case Z_DATA_ERROR:
		case Z_MEM_ERROR:
			inflateEnd(&stream);
			throw std::runtime_error("inflate() failed");
		}
	}

	inflateEnd(&stream);

	len = stream.total_out;
	data = UniqueCPtr<uint8_t[]>(output);
}

void Chunk::parseChunk() {
	Buffer nbt(data.get(), len);
	std::pair<std::string, std::shared_ptr<const NBT::Tag>> tag = NBT::Tag::readNamedTag(&nbt);
	if (tag.first != "")
		throw std::invalid_argument("invalid root tag");

	root = assertValue(std::dynamic_pointer_cast<const NBT::CompoundTag>(tag.second));
	level = assertValue(root->get<NBT::CompoundTag>("Level"));
}

void Chunk::analyzeChunk() {
	std::shared_ptr<const NBT::ByteTag> lightPopulatedTag = level->get<NBT::ByteTag>("LightPopulated");
	if (!(lightPopulatedTag && lightPopulatedTag->getValue()))
		throw std::invalid_argument("light data missing");

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
		std::shared_ptr<const NBT::ByteArrayTag> blockLight = assertValue(section->get<NBT::ByteArrayTag>("BlockLight"));
		std::shared_ptr<const NBT::ByteArrayTag> skyLight = assertValue(section->get<NBT::ByteArrayTag>("SkyLight"));
		size_t Y = assertValue(section->get<NBT::ByteTag>("Y"))->getValue();

		if (blocks->getLength() != SIZE*SIZE*SIZE || data->getLength() != SIZE*SIZE*SIZE/2
		    || blockLight->getLength() != SIZE*SIZE*SIZE/2 || skyLight->getLength() != SIZE*SIZE*SIZE/2)
			throw std::invalid_argument("corrupt chunk data");

		std::memcpy(blockIDs.get() + Y*SIZE*SIZE*SIZE, blocks->getValue(), SIZE*SIZE*SIZE);
		std::memcpy(blockData.get() + Y*SIZE*SIZE*SIZE/2, data->getValue(), SIZE*SIZE*SIZE/2);
		std::memcpy(blockBlockLight.get() + Y*SIZE*SIZE*SIZE/2, blockLight->getValue(), SIZE*SIZE*SIZE/2);
		std::memcpy(blockSkyLight.get() + Y*SIZE*SIZE*SIZE/2, skyLight->getValue(), SIZE*SIZE*SIZE/2);
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
				if (!Resource::BLOCK_TYPES[id].opaque)
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
