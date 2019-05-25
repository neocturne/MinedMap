/*
  Copyright (c) 2015-2019, Matthias Schiffer <mschiffer@universe-factory.net>
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


#pragma once


#include "../NBT/ByteArrayTag.hpp"
#include "../NBT/CompoundTag.hpp"
#include "../NBT/ListTag.hpp"
#include "../NBT/LongArrayTag.hpp"
#include "../Resource/BlockType.hpp"

#include <cstdint>
#include <stdexcept>


namespace MinedMap {
namespace World {

class Section {
public:
	static const size_t SIZE = 16;

private:
	size_t Y;
	std::shared_ptr<const NBT::ByteArrayTag> blockLight;

protected:
	static size_t getIndex(size_t x, size_t y, size_t z) {
		if (x >= SIZE || y >= SIZE || z >= SIZE)
			throw std::range_error("Chunk::getIndex(): bad coordinates");

		return SIZE*SIZE*y + SIZE*z + x;
	}

	static uint8_t getHalf(const uint8_t *v, size_t x, size_t y, size_t z) {
		size_t i = getIndex(x, y, z);

		if (i % 2)
			return (v[i/2] >> 4);
		else
			return (v[i/2] & 0xf);
	}

	Section(const std::shared_ptr<const NBT::CompoundTag> &section);

public:
	virtual ~Section() {}

	size_t getY() const { return Y; };

	virtual const Resource::BlockType * getBlockStateAt(size_t x, size_t y, size_t z) const;

	uint8_t getBlockLightAt(size_t x, size_t y, size_t z) const {
		if (!blockLight)
			return 0;

		return getHalf(blockLight->getValue(), x, y, z);
	}

	static std::unique_ptr<Section> makeSection(const std::shared_ptr<const NBT::CompoundTag> &section, uint32_t dataVersion);
};

class LegacySection : public Section {
private:
	std::shared_ptr<const NBT::ByteArrayTag> blocks;
	std::shared_ptr<const NBT::ByteArrayTag> data;


	uint8_t getBlockAt(size_t x, size_t y, size_t z) const {
		return blocks->getValue()[getIndex(x, y, z)];
	}

	uint8_t getDataAt(size_t x, size_t y, size_t z) const {
		return getHalf(data->getValue(), x, y, z);
	}

public:
	LegacySection(
		const std::shared_ptr<const NBT::CompoundTag> &section,
		std::shared_ptr<const NBT::ByteArrayTag> &&blocks0,
		std::shared_ptr<const NBT::ByteArrayTag> &&data0
	) : Section(section), blocks(blocks0), data(data0) {}

	virtual const Resource::BlockType * getBlockStateAt(size_t x, size_t y, size_t z) const;
};

class PaletteSection : public Section {
private:
	std::shared_ptr<const NBT::LongArrayTag> blockStates;
	std::vector<const Resource::BlockType *> palette;
	size_t bits;
	uint16_t mask;


	static const Resource::BlockType * lookup(const std::string &name, uint32_t dataVersion);

	static size_t mangleByteIndex(size_t index) {
		return (index & ~(size_t)7) + 7 - (index & 7);
	}

public:
	PaletteSection(
		const std::shared_ptr<const NBT::CompoundTag> &section,
		std::shared_ptr<const NBT::LongArrayTag> &&blockStates0,
		const std::shared_ptr<const NBT::ListTag> &paletteData,
		uint32_t dataVersion
	);

	virtual const Resource::BlockType * getBlockStateAt(size_t x, size_t y, size_t z) const;
};

}
}
