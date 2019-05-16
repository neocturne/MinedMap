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


#include "ChunkData.hpp"

#include <iostream>
#include <stdexcept>
#include <cstdlib>

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
	data = UniqueCPtr<uint8_t[]>(output);
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
