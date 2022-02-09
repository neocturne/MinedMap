// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015-2018, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#include "ChunkData.hpp"

#include <cstdlib>
#include <iostream>
#include <memory>
#include <stdexcept>

#include <zlib.h>


namespace MinedMap {
namespace World {

ChunkData::ChunkData(Buffer buffer) {
	size_t size = buffer.get32();

	Buffer buffer2(buffer.get(size), size);

	uint8_t format = buffer2.get8();
	if (format != 2)
		throw std::invalid_argument("unknown chunk format");

	inflateChunk(buffer2);
	parseChunk();
}

void ChunkData::inflateChunk(Buffer buffer) {
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
	data = std::unique_ptr<uint8_t[]>(output);
}

void ChunkData::parseChunk() {
	Buffer nbt(data.get(), len);
	std::pair<std::string, std::shared_ptr<const NBT::Tag>> tag = NBT::Tag::readNamedTag(&nbt);
	if (!tag.first.empty())
		throw std::invalid_argument("invalid root tag");

	root = assertValue(std::dynamic_pointer_cast<const NBT::CompoundTag>(tag.second));
}

}
}
