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


#include "Region.hpp"

#include <fstream>
#include <iostream>


namespace MinedMap {
namespace World {

Region::ChunkMap Region::processHeader(const uint8_t header[4096]) {
	ChunkMap map;

	for (size_t z = 0; z < 32; z++) {
		for (size_t x = 0; x < 32; x++) {
			const uint8_t *p = &header[128*z + x*4];

			size_t offset = (p[0] << 16) | (p[1] << 8) | p[2];

			if (!offset)
				continue;

			size_t len = p[3];

			map.emplace(offset, ChunkDesc(x, z, len));
		}
	}

	return map;
}

void Region::visitChunks(const char *filename, const ChunkVisitor &visitor) {
	std::ifstream file;
	file.exceptions(std::ios::failbit | std::ios::badbit);
	file.open(filename, std::ios::in | std::ios::binary);

	ChunkMap chunkMap;

	{
		uint8_t header[4096];
		file.read((char *)header, sizeof(header));

		chunkMap = processHeader(header);
	}

	bool seen[SIZE][SIZE] = {};

	size_t i = 1, c = 0;
	while (!file.eof()) {
		auto it = chunkMap.find(i);
		if (it == chunkMap.end()) {
			file.ignore(4096);
			i++;
			continue;
		}

		size_t x, z, len;
		std::tie(x, z, len) = it->second;

		if (seen[x][z])
			throw std::invalid_argument("duplicate chunk");

		seen[x][z] = true;
		c++;

		uint8_t buffer[len * 4096];
		file.read((char *)buffer, len * 4096);

		ChunkData chunk(Buffer(buffer, len * 4096));
		visitor(x, z, &chunk);

		i += len;
	}

	if (c != chunkMap.size())
		throw std::invalid_argument("region incomplete");
}

}
}
