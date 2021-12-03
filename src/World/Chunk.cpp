// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015-2021, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#include "Chunk.hpp"
#include "../NBT/IntTag.hpp"
#include "../NBT/ListTag.hpp"
#include "../NBT/StringTag.hpp"

#include <cstring>
#include <stdexcept>


namespace MinedMap {
namespace World {

Chunk::Chunk(const ChunkData *data) {
	std::shared_ptr<const NBT::CompoundTag> level =
		assertValue(data->getRoot()->get<NBT::CompoundTag>("Level"));

	std::shared_ptr<const NBT::ListTag> sectionsTag = level->get<NBT::ListTag>("Sections");
	if (!sectionsTag || sectionsTag->empty())
		return;

	auto biomesIntArray = level->get<NBT::IntArrayTag>("Biomes");
	auto biomesByteArray = level->get<NBT::ByteArrayTag>("Biomes");

	if (biomesIntArray && biomesIntArray->getLength() == BSIZE*BSIZE*BMAXY)
		biomeInts = std::move(biomesIntArray);
	else if (biomesIntArray && biomesIntArray->getLength() == SIZE*SIZE)
		biomeIntsPre115 = std::move(biomesIntArray);
	else if (biomesByteArray && biomesByteArray->getLength() == SIZE*SIZE)
		biomeBytes = std::move(biomesByteArray);
	else
		throw std::invalid_argument("corrupt biome data");

	auto dataVersionTag = data->getRoot()->get<NBT::IntTag>("DataVersion");
	uint32_t dataVersion = dataVersionTag ? dataVersionTag->getValue() : 0;

	for (auto &sTag : *sectionsTag) {
		auto s = std::dynamic_pointer_cast<const NBT::CompoundTag>(sTag);
		std::unique_ptr<Section> section = Section::makeSection(s, dataVersion);
		section_idx_t Y = section->getY();
		sections.resize(Y);
		sections.push_back(std::move(section));
	}
}

uint8_t Chunk::getBiome(block_idx_t x, y_idx_t y, block_idx_t z) const {
	if (x > SIZE || y > MAXY || z > SIZE)
		throw std::invalid_argument("corrupt chunk data");

	if (biomeInts)
		return biomeInts->getValue((y>>BSHIFT)*BSIZE*BSIZE + (z>>BSHIFT)*BSIZE + (x>>BSHIFT));
	else if (biomeIntsPre115)
		return biomeIntsPre115->getValue(z*SIZE + x);
	else if (biomeBytes)
		return biomeBytes->getValue(z*SIZE + x);
	else
		return 0xff;
}

Block Chunk::getBlock(block_idx_t x, Chunk::Height height, block_idx_t z) const {
	Block block = {};

	block.depth = height.depth;

	section_idx_t Y = height.y >> HSHIFT;
	block_idx_t y = height.y & HMASK;

	if (Y < sections.size() && sections[Y])
		block.type = sections[Y]->getBlockStateAt(x, y, z);

	section_idx_t Yt = (height.y + 1) >> HSHIFT;
	block_idx_t yt = (height.y + 1) & HMASK;

	if (Yt < sections.size() && sections[Yt])
		block.blockLight = sections[Yt]->getBlockLightAt(x, yt, z);

	return block;
}

bool Chunk::getHeight(
	Chunk::Height *height, const Section *section,
	block_idx_t x, block_idx_t y, block_idx_t z, int flags
) const {
	if (height->depth > 0)
		return false;

	if (!(flags & WITH_DEPTH) && height->y > 0)
		return false;

	const Resource::BlockType *type = section->getBlockStateAt(x, y, z);
	if (!type || !(type->flags & BLOCK_OPAQUE))
		return false;

	if (height->y == 0)
		height->y = (section->getY() << HSHIFT) + y;

	if (!(flags & WITH_DEPTH))
		return true;

	if (type->flags & BLOCK_WATER)
		return false;

	height->depth = (section->getY() << HSHIFT) + y;

	return true;
}

Chunk::Heightmap Chunk::getTopLayer(int flags) const {
	uint32_t done = 0;
	Heightmap ret = {};

	for (int16_t Y = sections.size() - 1; Y >= 0; Y--) {
		if (done == SIZE*SIZE)
			break;

		if (!sections[Y])
			continue;

		const Section *section = sections[Y].get();

		for (int8_t y = SIZE-1; y >= 0; y--) {
			if (done == SIZE*SIZE)
				break;

			for (block_idx_t z = 0; z < SIZE; z++) {
				for (block_idx_t x = 0; x < SIZE; x++) {
					if (getHeight(&ret.v[x][z], section, x, y, z, flags))
						done++;
				}
			}
		}
	}

	return ret;
}

}
}
