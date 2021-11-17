// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#include "Region.hpp"

#include <cstring>
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
	file.exceptions(std::ios::badbit);
	file.open(filename, std::ios::in | std::ios::binary);

	ChunkMap chunkMap;

	{
		uint8_t header[4096];
		file.read((char *)header, sizeof(header));

		chunkMap = processHeader(header);
	}

	bool seen[SIZE][SIZE] = {};

	size_t i = 1, c = 0;
	while (!file.eof() && !file.fail()) {
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
		std::memset(buffer, 0, sizeof(buffer));
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
