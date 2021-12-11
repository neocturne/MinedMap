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
	std::shared_ptr<const NBT::ListTag> sectionsTag;

	auto level = data->getRoot()->get<NBT::CompoundTag>("Level");
	if (level) {
		sectionsTag = level->get<NBT::ListTag>("Sections");
		if (!sectionsTag || sectionsTag->empty())
			return;

		auto biomesIntArray = level->get<NBT::IntArrayTag>("Biomes");
		auto biomesByteArray = level->get<NBT::ByteArrayTag>("Biomes");

		if (biomesIntArray && biomesIntArray->getLength() == BSIZE*BSIZE*BMAXY) {
			biomeInts = std::move(biomesIntArray);
			biomeFormat = INT_ARRAY;
		} else if (biomesIntArray && biomesIntArray->getLength() == SIZE*SIZE) {
			biomeInts = std::move(biomesIntArray);
			biomeFormat = INT_ARRAY_PRE1_15;
		} else if (biomesByteArray && biomesByteArray->getLength() == SIZE*SIZE) {
			biomeBytes = std::move(biomesByteArray);
			biomeFormat = BYTE_ARRAY;
		} else {
			throw std::invalid_argument("corrupt biome data");
		}
	} else {
		sectionsTag = data->getRoot()->get<NBT::ListTag>("sections");
		if (!sectionsTag || sectionsTag->empty())
			return;

		biomeFormat = SECTION;
	}

	auto dataVersionTag = data->getRoot()->get<NBT::IntTag>("DataVersion");
	uint32_t dataVersion = dataVersionTag ? dataVersionTag->getValue() : 0;

	for (auto &sTag : *sectionsTag) {
		auto s = std::dynamic_pointer_cast<const NBT::CompoundTag>(sTag);
		std::unique_ptr<Section> section = Section::makeSection(s, dataVersion);
		section_idx_t Y = section->getY();
		if (sections.empty())
			sectionOffset = Y;

		Y -= sectionOffset;
		assertValue(Y >= 0 && size_t(Y) >= sections.size());
		sections.resize(Y);
		sections.push_back(std::move(section));
	}
}

uint8_t Chunk::getBiome(block_idx_t x, y_idx_t y, block_idx_t z) const {
	if (x >= SIZE || z >= SIZE)
		throw std::invalid_argument("getBiome: invalid block coordinate");

	switch (biomeFormat) {
	case INT_ARRAY:
		if (y < 0 || y >= MAXY)
			break;
		return biomeInts->getValue((y>>BSHIFT)*BSIZE*BSIZE + (z>>BSHIFT)*BSIZE + (x>>BSHIFT));
	case INT_ARRAY_PRE1_15:
		return biomeInts->getValue(z*SIZE + x);
	case BYTE_ARRAY:
		return biomeBytes->getValue(z*SIZE + x);
	case SECTION: {
		section_idx_t Y = (y >> HSHIFT) - sectionOffset;

		if (Y < 0 || size_t(Y) >= sections.size() || !sections[Y])
			break;

		return sections[Y]->getBiomeAt(x, y & HMASK, z);
	}
	default:
		break;
	}

	return 0xff;
}

Block Chunk::getBlock(block_idx_t x, Chunk::Height height, block_idx_t z) const {
	Block block = {};

	block.depth = height.depth;

	if (height.y == y_idx_min)
		return block;

	section_idx_t Y = (height.y >> HSHIFT) - sectionOffset;
	block_idx_t y = height.y & HMASK;

	if (Y >= 0 && size_t(Y) < sections.size() && sections[Y])
		block.type = sections[Y]->getBlockStateAt(x, y, z);

	section_idx_t Yt = ((height.y + 1) >> HSHIFT) - sectionOffset;
	block_idx_t yt = (height.y + 1) & HMASK;

	if (Yt >= 0 && size_t(Yt) < sections.size() && sections[Yt])
		block.blockLight = sections[Yt]->getBlockLightAt(x, yt, z);

	return block;
}

bool Chunk::getHeight(
	Chunk::Height *height, const Section *section,
	block_idx_t x, block_idx_t y, block_idx_t z, int flags
) const {
	if (height->depth > y_idx_min)
		return false;

	if (!(flags & WITH_DEPTH) && height->y > y_idx_min)
		return false;

	const Resource::BlockType *type = section->getBlockStateAt(x, y, z);
	if (!type || !(type->flags & BLOCK_OPAQUE))
		return false;

	if (height->y == y_idx_min)
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
	Heightmap ret;

	for (block_idx_t z = 0; z < SIZE; z++) {
		for (block_idx_t x = 0; x < SIZE; x++)
			ret.v[x][z] = Height { .y = y_idx_min, .depth = y_idx_min };
	}

	for (section_idx_t Y = sections.size() - 1; Y >= 0; Y--) {
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
