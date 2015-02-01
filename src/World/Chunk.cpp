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
#include "BlockType.hpp"

#include <iostream>
#include <stdexcept>
#include <cstdlib>

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
	if (!lightPopulatedTag && lightPopulatedTag->getValue())
		throw std::invalid_argument("light data missing");

	for (auto &section : *assertValue(level->get<NBT::ListTag<NBT::CompoundTag>>("Sections")))
		sections.emplace_back(*section);

	maxY = sections.back().getY() + SIZE;
	blocks.reset(new Block[maxY * SIZE * SIZE]);

	for (auto &section : sections) {
		unsigned Y = section.getY();

		for (size_t y = 0; y < SIZE; y++) {
			for (size_t z = 0; z < SIZE; z++) {
				for (size_t x = 0; x < SIZE; x++) {
					Block &block = getBlock(x, Y+y, z);

					block.id = section.getBlockAt(x, y, z);
					block.data = section.getDataAt(x, y, z);
					block.blockLight = section.getBlockLightAt(x, y, z);
					block.skyLight = section.getSkyLightAt(x, y, z);
				}
			}
		}
	}
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

				const Block &block = getBlock(x, y, z);
				if (!BLOCK_TYPES[block.id].opaque)
					continue;

				Block &b = ret.blocks[x][z];

				b.id = block.id;
				b.data = block.data;

				const Block *lightBlock;
				if (y < maxY-1)
					lightBlock = &getBlock(x, y+1, z);
				else
					lightBlock =  &block;

				b.blockLight = lightBlock->blockLight;
				b.skyLight = lightBlock->skyLight;


				unsigned h;
				for (h = y; h > 0; h--) {
					const Block &block2 = getBlock(x, h, z);
					if (block2.id != 8 && block2.id != 9)
						break;
				}

				b.height = h;

				done++;
			}
		}
	}

	return ret;
}

}
}
