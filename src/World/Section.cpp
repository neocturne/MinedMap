/*
  Copyright (c) 2015-2018, Matthias Schiffer <mschiffer@universe-factory.net>
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


#include "Section.hpp"
#include "../NBT/ByteTag.hpp"
#include "../NBT/StringTag.hpp"
#include "../Util.hpp"

#include <cstdio>


namespace MinedMap {
namespace World {

Section::Section(const std::shared_ptr<const NBT::CompoundTag> &section) {
	Y = assertValue(section->get<NBT::ByteTag>("Y"))->getValue();
	blockLight = section->get<NBT::ByteArrayTag>("BlockLight");
}


std::unique_ptr<Section> Section::makeSection(const std::shared_ptr<const NBT::CompoundTag> &section) {
	std::shared_ptr<const NBT::LongArrayTag> blockStates = section->get<NBT::LongArrayTag>("BlockStates");
	if (blockStates) {
		const std::shared_ptr<const NBT::ListTag> palette = assertValue(section->get<NBT::ListTag>("Palette"));

		return std::unique_ptr<Section>(new PaletteSection(section, std::move(blockStates), palette));
	}

	std::shared_ptr<const NBT::ByteArrayTag> blocks = assertValue(section->get<NBT::ByteArrayTag>("Blocks"));
	std::shared_ptr<const NBT::ByteArrayTag> data = assertValue(section->get<NBT::ByteArrayTag>("Data"));

	return std::unique_ptr<Section>(new LegacySection(section, std::move(blocks), std::move(data)));
}


const Resource::BlockType * LegacySection::getBlockStateAt(size_t x, size_t y, size_t z) const {
	uint8_t type = getBlockAt(x, y, z);
	uint8_t data = getDataAt(x, y, z);

	return Resource::LEGACY_BLOCK_TYPES.types[type][data];
}


PaletteSection::PaletteSection(
	const std::shared_ptr<const NBT::CompoundTag> &section,
	std::shared_ptr<const NBT::LongArrayTag> &&blockStates0,
	const std::shared_ptr<const NBT::ListTag> &paletteData
) : Section(section), blockStates(blockStates0) {
	if (blockStates->getLength() % 64)
		throw std::invalid_argument("corrupt section data");

	bits = blockStates->getLength() / 64;
	if (bits > 16)
		throw std::invalid_argument("unsupported block state bits");

	mask = (1u << bits) - 1;


	for (const auto &entry : *paletteData) {
		const NBT::CompoundTag &paletteEntry = *assertValue(dynamic_cast<const NBT::CompoundTag *>(entry.get()));
		std::string name = assertValue(paletteEntry.get<NBT::StringTag>("Name"))->getValue();

		const Resource::BlockType *type = Resource::BlockType::lookup(name);
		if (!type)
			std::fprintf(stderr, "Warning: unknown block type: %s\n", name.c_str());

		palette.push_back(type);
	}
}

const Resource::BlockType * PaletteSection::getBlockStateAt(size_t x, size_t y, size_t z) const {
	size_t index = bits * getIndex(x, y, z);
	size_t pos = index >> 3;
	size_t shift = index & 7;

	uint32_t v = blockStates->getPointer()[mangleByteIndex(pos)];

	if (shift + bits > 8)
		v |= blockStates->getPointer()[mangleByteIndex(pos + 1)] << 8;
	if (shift + bits > 16)
		v |= blockStates->getPointer()[mangleByteIndex(pos + 2)] << 16;
	/* We do not need to check for shift+bits > 24: bits should never
	be greater than 12, so our value will never span more than 3 bytes */

	return palette.at((v >> shift) & mask);
}

}
}
