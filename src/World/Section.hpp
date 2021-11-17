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

private:
	section_idx_t Y;
	std::shared_ptr<const NBT::ByteArrayTag> blockLight;

protected:
	static size_t getIndex(block_idx_t x, block_idx_t y, block_idx_t z) {
		if (x >= SIZE || y >= SIZE || z >= SIZE)
			throw std::range_error("Chunk::getIndex(): bad coordinates");

		return SIZE*SIZE*y + SIZE*z + x;
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
	uint32_t dataVersion;
	unsigned bits;


	static const Resource::BlockType * lookup(const std::string &name, uint32_t dataVersion);

	static size_t mangleByteIndex(size_t index) {
		return (index & ~(size_t)7) + 7 - (index & 7);
	}

public:
	PaletteSection(
		const std::shared_ptr<const NBT::CompoundTag> &section,
		std::shared_ptr<const NBT::LongArrayTag> &&blockStates0,
		const std::shared_ptr<const NBT::ListTag> &paletteData,
		uint32_t dataVersion0
	);

	virtual const Resource::BlockType * getBlockStateAt(block_idx_t x, block_idx_t y, block_idx_t z) const;
};

}
}
