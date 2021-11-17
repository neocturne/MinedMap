// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2018, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#include "Buffer.hpp"
#include "GZip.hpp"
#include "Util.hpp"
#include "NBT/Tag.hpp"
#include "World/Region.hpp"

#include <cstdio>
#include <iostream>


int main(int argc, char *argv[]) {
	using namespace MinedMap;

	if (argc != 2) {
		std::fprintf(stderr, "Usage: %s <regionfile>\n", argv[0]);
		return 1;
	}

	World::Region::visitChunks(argv[1], [&] (size_t X, size_t Z, const World::ChunkData *chunk) {
		std::cout << "Chunk(" << X << ", " << Z << "): "
			<< chunk->getRoot() << std::endl;
	});

	return 0;
}
