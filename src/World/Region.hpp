// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015-2021, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include "Chunk.hpp"

#include <functional>
#include <memory>
#include <stdexcept>
#include <tuple>
#include <unordered_map>


namespace MinedMap {
namespace World {

class Region {
public:
	// Number of chunks in a region in each dimension
	static const uint32_t SIZE = 32;

	typedef std::function<void (chunk_idx_t, chunk_idx_t, const ChunkData *)> ChunkVisitor;

	Region() = delete;

private:
	typedef std::tuple<chunk_idx_t, chunk_idx_t, uint8_t> ChunkDesc;
	typedef std::unordered_map<uint32_t, ChunkDesc> ChunkMap;

	static ChunkMap processHeader(const uint8_t header[4096]);

public:
	static void visitChunks(const char *filename, const ChunkVisitor &visitor);
};

}
}
