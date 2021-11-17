// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
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
	static const size_t SIZE = 32;

	typedef std::function<void (size_t, size_t, const ChunkData *)> ChunkVisitor;

	Region() = delete;

private:
	typedef std::tuple<size_t, size_t, size_t> ChunkDesc;
	typedef std::unordered_map<size_t, ChunkDesc> ChunkMap;

	static ChunkMap processHeader(const uint8_t header[4096]);

public:
	static void visitChunks(const char *filename, const ChunkVisitor &visitor);
};

}
}
