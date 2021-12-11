// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015-2021, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#include "Section.hpp"
#include "../Resource/Biome.hpp"
#include "../NBT/ByteTag.hpp"
#include "../NBT/StringTag.hpp"

#include <cstdio>


namespace MinedMap {
namespace World {

Section::Section(const std::shared_ptr<const NBT::CompoundTag> &section) {
	Y = int8_t(assertValue(section->get<NBT::ByteTag>("Y"))->getValue());
	blockLight = section->get<NBT::ByteArrayTag>("BlockLight");
}

const Resource::BlockType * Section::getBlockStateAt(block_idx_t, block_idx_t, block_idx_t) const {
	return nullptr;
}

uint8_t Section::getBiomeAt(block_idx_t x, block_idx_t y, block_idx_t z) const {
	return 0xff;
}

std::unique_ptr<Section> Section::makeSection(const std::shared_ptr<const NBT::CompoundTag> &section, uint32_t dataVersion) {
	{
		const std::shared_ptr<const NBT::CompoundTag> blockStates = section->get<NBT::CompoundTag>("block_states");
		if (blockStates) {
			const std::shared_ptr<const NBT::ListTag> palette = assertValue(blockStates->get<NBT::ListTag>("palette"));
			std::shared_ptr<const NBT::LongArrayTag> data = blockStates->get<NBT::LongArrayTag>("data");

			const std::shared_ptr<const NBT::CompoundTag> biomes = assertValue(section->get<NBT::CompoundTag>("biomes"));
			const std::shared_ptr<const NBT::ListTag> biomePalette = assertValue(biomes->get<NBT::ListTag>("palette"));
			std::shared_ptr<const NBT::LongArrayTag> biomeData = biomes->get<NBT::LongArrayTag>("data");

			return std::unique_ptr<Section>(new PaletteSection(
				section, std::move(data), palette, std::move(biomeData), biomePalette, dataVersion
			));
		}
	}

	{
		std::shared_ptr<const NBT::LongArrayTag> blockStates = section->get<NBT::LongArrayTag>("BlockStates");
		if (blockStates) {
			const std::shared_ptr<const NBT::ListTag> palette = assertValue(section->get<NBT::ListTag>("Palette"));

			return std::unique_ptr<Section>(new PaletteSection(
				section, std::move(blockStates), palette,
				std::shared_ptr<const NBT::LongArrayTag>(), std::shared_ptr<const NBT::ListTag>(), dataVersion
			));
		}
	}

	{
		std::shared_ptr<const NBT::ByteArrayTag> blocks = section->get<NBT::ByteArrayTag>("Blocks");
		if (blocks) {
			std::shared_ptr<const NBT::ByteArrayTag> data = assertValue(section->get<NBT::ByteArrayTag>("Data"));

			return std::unique_ptr<Section>(new LegacySection(section, std::move(blocks), std::move(data)));
		}
	}

	return std::unique_ptr<Section>(new Section(section));
}


const Resource::BlockType * LegacySection::getBlockStateAt(block_idx_t x, block_idx_t y, block_idx_t z) const {
	uint8_t type = getBlockAt(x, y, z);
	uint8_t data = getDataAt(x, y, z);

	return Resource::LEGACY_BLOCK_TYPES.types[type][data];
}


const Resource::BlockType * PaletteSection::lookup(const std::string &name, uint32_t dataVersion) {
	if (dataVersion < 1900 && name == "minecraft:stone_slab")
		return Resource::BlockType::lookup("minecraft:smooth_stone_slab");

	return Resource::BlockType::lookup(name);
}

PaletteSection::PaletteSection(
	const std::shared_ptr<const NBT::CompoundTag> &section,
	std::shared_ptr<const NBT::LongArrayTag> &&blockStates0,
	const std::shared_ptr<const NBT::ListTag> &paletteData,
	std::shared_ptr<const NBT::LongArrayTag> &&biomes0,
	const std::shared_ptr<const NBT::ListTag> &biomePaletteData,
	uint32_t dataVersion0
) : Section(section), blockStates(blockStates0), biomes(biomes0), dataVersion(dataVersion0) {
	bits = 4;
	while ((1u << bits) < paletteData->size()) {
		bits++;

		if (bits > 12)
			throw std::invalid_argument("unsupported block palette size");
	}

	biomeBits = 1;
	if (biomePaletteData) {
		while ((1u << biomeBits) < biomePaletteData->size()) {
			biomeBits++;

			if (bits > 6)
				throw std::invalid_argument("unsupported biome palette size");
		}
	}

	unsigned expectedLength;

	if (dataVersion < 2529) {
		expectedLength = 64 * bits;
	} else {
		unsigned blocksPerWord = (64 / bits);
		expectedLength = (4096 + blocksPerWord - 1) / blocksPerWord;
	}

	if (blockStates && blockStates->getLength() != expectedLength)
		throw std::invalid_argument("corrupt section block data");

	unsigned biomesPerWord = (64 / biomeBits);
	unsigned expectedBiomeLength = (64 + biomesPerWord - 1) / biomesPerWord;

	if (biomes && biomes->getLength() != expectedBiomeLength)
		throw std::invalid_argument("corrupt section biome data");

	for (const auto &entry : *paletteData) {
		const NBT::CompoundTag &paletteEntry = *assertValue(dynamic_cast<const NBT::CompoundTag *>(entry.get()));
		std::string name = assertValue(paletteEntry.get<NBT::StringTag>("Name"))->getValue();

		const Resource::BlockType *type = lookup(name, dataVersion);
		if (!type)
			std::fprintf(stderr, "Warning: unknown block type: %s\n", name.c_str());

		palette.push_back(type);
	}

	if (biomePaletteData) {
		for (const auto &entry : *biomePaletteData) {
			const NBT::StringTag &paletteEntry =
				*assertValue(dynamic_cast<const NBT::StringTag *>(entry.get()));
			std::string name = paletteEntry.getValue();

			auto it = Resource::Biome::Names.find(name);
			uint8_t biome;
			if (it != Resource::Biome::Names.end()) {
				biome = it->second;
			} else {
				std::fprintf(stderr, "Warning: unknown biome: %s\n", name.c_str());
				biome = 0xff;
			}

			biomePalette.push_back(biome);
		}
	}
}

const Resource::BlockType * PaletteSection::getBlockStateAt(block_idx_t x, block_idx_t y, block_idx_t z) const {
	if (!blockStates)
		return palette.at(0);

	size_t index = getIndex(x, y, z);
	size_t bitIndex;

	if (dataVersion < 2529) {
		bitIndex = bits * index;
	} else {
		unsigned blocksPerWord = (64 / bits);
		size_t word = index / blocksPerWord;
		bitIndex = 64 * word + bits * (index % blocksPerWord);
	}

	size_t pos = bitIndex >> 3;
	unsigned shift = bitIndex & 7;
	uint16_t mask = (1u << bits) - 1;

	uint32_t v = blockStates->getPointer()[mangleByteIndex(pos)];

	if (shift + bits > 8)
		v |= blockStates->getPointer()[mangleByteIndex(pos + 1)] << 8;
	if (shift + bits > 16)
		v |= blockStates->getPointer()[mangleByteIndex(pos + 2)] << 16;
	/* We do not need to check for shift+bits > 24: bits should never
	be greater than 12, so our value will never span more than 3 bytes */

	return palette.at((v >> shift) & mask);
}

uint8_t PaletteSection::getBiomeAt(block_idx_t x, block_idx_t y, block_idx_t z) const {
	if (!biomes)
		return biomePalette.at(0);

	size_t index = getBiomeIndex(x, y, z);

	unsigned biomesPerWord = (64 / biomeBits);
	size_t word = index / biomesPerWord;
	size_t bitIndex = 64 * word + biomeBits * (index % biomesPerWord);

	size_t pos = bitIndex >> 3;
	unsigned shift = bitIndex & 7;
	uint16_t mask = (1u << biomeBits) - 1;

	uint32_t v = biomes->getPointer()[mangleByteIndex(pos)];

	if (shift + biomeBits > 8)
		v |= biomes->getPointer()[mangleByteIndex(pos + 1)] << 8;
	/* We do not need to check for shift+bits > 16: biomeBits should never
	be greater than 6, so our value will never span more than 2 bytes */

	return biomePalette.at((v >> shift) & mask);
}

}
}
