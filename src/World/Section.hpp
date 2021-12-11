// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015-2021, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once


#include "../NBT/ByteArrayTag.hpp"
#include "../NBT/CompoundTag.hpp"
#include "../NBT/ListTag.hpp"
#include "../NBT/LongArrayTag.hpp"
#include "../Resource/BlockType.hpp"
#include "../Util.hpp"

#include <cstdint>
#include <stdexcept>


namespace MinedMap {
namespace World {

class Section {
public:
	// Number of blocks in a section in each dimension
	static const uint32_t SIZE = 16;

	// Since Minecraft 1.15, biome information is stored for
	// 4x4x4 block groups
	static const unsigned BSHIFT = 2;
	// Number of biome values in a chunk in x/z dimensions
	static const uint32_t BSIZE = SIZE >> BSHIFT;


private:
	section_idx_t Y;
	std::shared_ptr<const NBT::ByteArrayTag> blockLight;

protected:
	static size_t getIndex(block_idx_t x, block_idx_t y, block_idx_t z) {
		if (x >= SIZE || y >= SIZE || z >= SIZE)
			throw std::range_error("Section::getIndex(): bad coordinates");

		return SIZE*SIZE*y + SIZE*z + x;
	}

	static size_t getBiomeIndex(block_idx_t x, block_idx_t y, block_idx_t z) {
		if (x >= SIZE || y >= SIZE || z >= SIZE)
			throw std::range_error("Section::getBiomeIndex(): bad coordinates");

		return BSIZE*BSIZE*(y>>BSHIFT) + BSIZE*(z>>BSHIFT) + (x>>BSHIFT);
	}

	static uint8_t getHalf(const uint8_t *v, block_idx_t x, block_idx_t y, block_idx_t z) {
		size_t i = getIndex(x, y, z);

		if (i % 2)
			return (v[i/2] >> 4);
		else
			return (v[i/2] & 0xf);
	}

	Section(const std::shared_ptr<const NBT::CompoundTag> &section);

public:
	virtual ~Section() {}

	section_idx_t getY() const { return Y; };

	virtual const Resource::BlockType * getBlockStateAt(block_idx_t x, block_idx_t y, block_idx_t z) const;
	virtual uint8_t getBiomeAt(block_idx_t x, block_idx_t y, block_idx_t z) const;

	uint8_t getBlockLightAt(block_idx_t x, block_idx_t y, block_idx_t z) const {
		if (!blockLight)
			return 0;

		return getHalf(blockLight->getPointer(), x, y, z);
	}

	static std::unique_ptr<Section> makeSection(const std::shared_ptr<const NBT::CompoundTag> &section, uint32_t dataVersion);
};

class LegacySection : public Section {
private:
	std::shared_ptr<const NBT::ByteArrayTag> blocks;
	std::shared_ptr<const NBT::ByteArrayTag> data;


	uint8_t getBlockAt(block_idx_t x, block_idx_t y, block_idx_t z) const {
		return blocks->getValue(getIndex(x, y, z));
	}

	uint8_t getDataAt(block_idx_t x, block_idx_t y, block_idx_t z) const {
		return getHalf(data->getPointer(), x, y, z);
	}

public:
	LegacySection(
		const std::shared_ptr<const NBT::CompoundTag> &section,
		std::shared_ptr<const NBT::ByteArrayTag> &&blocks0,
		std::shared_ptr<const NBT::ByteArrayTag> &&data0
	) : Section(section), blocks(blocks0), data(data0) {}

	virtual const Resource::BlockType * getBlockStateAt(block_idx_t x, block_idx_t y, block_idx_t z) const;
};

class PaletteSection : public Section {
private:
	std::shared_ptr<const NBT::LongArrayTag> blockStates;
	std::vector<const Resource::BlockType *> palette;
	unsigned bits;

	std::shared_ptr<const NBT::LongArrayTag> biomes;
	std::vector<uint8_t> biomePalette;
	unsigned biomeBits;

	uint32_t dataVersion;

	static const Resource::BlockType * lookup(const std::string &name, uint32_t dataVersion);

	static size_t mangleByteIndex(size_t index) {
		return (index & ~(size_t)7) + 7 - (index & 7);
	}

public:
	PaletteSection(
		const std::shared_ptr<const NBT::CompoundTag> &section,
		std::shared_ptr<const NBT::LongArrayTag> &&blockStates0,
		const std::shared_ptr<const NBT::ListTag> &paletteData,
		std::shared_ptr<const NBT::LongArrayTag> &&biomes0,
		const std::shared_ptr<const NBT::ListTag> &biomePaletteData,
		uint32_t dataVersion0
	);

	virtual const Resource::BlockType * getBlockStateAt(block_idx_t x, block_idx_t y, block_idx_t z) const;
	virtual uint8_t getBiomeAt(block_idx_t x, block_idx_t y, block_idx_t z) const;
};

}
}
