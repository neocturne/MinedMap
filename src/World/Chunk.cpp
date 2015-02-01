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

#include <iostream>
#include <stdexcept>
#include <cstdlib>

#include <zlib.h>


namespace MinedMap {
namespace World {

std::pair<UniqueCPtr<uint8_t[]>, size_t> Chunk::inflateChunk(uint8_t *data, size_t len) {
	size_t outlen = 0;
	uint8_t *output = nullptr;

	z_stream stream = {};
	int ret = inflateInit(&stream);
	if (ret != Z_OK)
		throw std::runtime_error("inflateInit() failed");

	stream.next_in = data;
	stream.avail_in = len;

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

	return std::make_pair(UniqueCPtr<uint8_t[]>(output), stream.total_out);
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

	size_t len;
	std::tie(data, len) = inflateChunk(buffer+5, size-1);

	std::cerr << "Chunk has size " << size << " (" << len << " inflated)" << std::endl;

	Buffer nbt(data.get(), len);
	std::pair<std::string, std::shared_ptr<const NBT::Tag>> tag = NBT::Tag::readNamedTag(&nbt);

	std::shared_ptr<const NBT::CompoundTag>::operator=(std::dynamic_pointer_cast<const NBT::CompoundTag>(tag.second));

	if (!(*this) || tag.first != "")
		throw std::invalid_argument("invalid root tag");

	sections = (*this)->get<const NBT::ListTag<NBT::CompoundTag>>("Level", "Sections");
	if (!sections)
		throw std::invalid_argument("no sections found");
}

}
}
