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
#include "../Util.hpp"
#include "../NBT/ByteTag.hpp"
#include "../NBT/ByteArrayTag.hpp"

#include <iostream>
#include <stdexcept>
#include <cstdlib>

#include <zlib.h>


namespace MinedMap {
namespace World {

void Chunk::inflateChunk(uint8_t *buffer, size_t buflen) {
	size_t outlen = 0;
	uint8_t *output = nullptr;

	z_stream stream = {};
	int ret = inflateInit(&stream);
	if (ret != Z_OK)
		throw std::runtime_error("inflateInit() failed");

	stream.next_in = buffer;
	stream.avail_in = buflen;

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

	std::shared_ptr<const NBT::CompoundTag>::operator=(std::dynamic_pointer_cast<const NBT::CompoundTag>(tag.second));

	if (!(*this) || tag.first != "")
		throw std::invalid_argument("invalid root tag");

	level = assertValue((*this)->get<NBT::CompoundTag>("Level"));
	sections = assertValue(level->get<NBT::ListTag<NBT::CompoundTag>>("Sections"));
}

void Chunk::analyzeChunk() {
	maxY = (assertValue(sections->back()->get<NBT::ByteTag>("Y"))->getValue() + 1) * SIZE;

	for (auto &section : *sections) {
		if (assertValue(section->get<NBT::ByteArrayTag>("Blocks"))->getLength() != SIZE*SIZE*SIZE
		    || assertValue(section->get<NBT::ByteArrayTag>("Data"))->getLength() != SIZE*SIZE*SIZE/2)
			throw std::invalid_argument("corrupt chunk data");
	}
}

uint8_t Chunk::getBlockAt(const std::shared_ptr<const NBT::CompoundTag> &section, size_t x, size_t y, size_t z) {
	return (*section->get<NBT::ByteArrayTag>("Blocks"))[getIndex(x, y, z)];
}

uint8_t Chunk::getDataAt(const std::shared_ptr<const NBT::CompoundTag> &section, size_t x, size_t y, size_t z) {
	size_t i = getIndex(x, y, z);
	uint8_t v = (*section->get<NBT::ByteArrayTag>("Data"))[i / 2];

	if (i % 2)
		return (v >> 4);
	else
		return (v & 0xf);
}

Chunk::Chunk(uint8_t *buffer, size_t buflen) {
	if (buflen < 5)
		throw std::invalid_argument("short chunk");

	size_t size = (buffer[0] << 24) | (buffer[1] << 16) | (buffer[2] << 8) | buffer[3];
	if (size < 1 || size > (buflen - 4))
		throw std::invalid_argument("invalid chunk size");

	uint8_t format = buffer[4];
	if (format != 2)
		throw std::invalid_argument("unknown chunk format");

	inflateChunk(buffer+5, size-1);
	parseChunk();
	analyzeChunk();
}

}
}
