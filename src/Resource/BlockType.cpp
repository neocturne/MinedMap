// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015-2018, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#include "BlockType.hpp"


namespace MinedMap {
namespace Resource {

const std::unordered_map<std::string, BlockType> BlockType::Types = {

#include "BlockType.inc.cpp"

};

struct LegacyBlockType {
       const char *data[16];
};

static constexpr LegacyBlockType simple(const char *t) {
	return {
		t, t, t, t,
		t, t, t, t,
		t, t, t, t,
		t, t, t, t,
	};
}

static const LegacyBlockType LEGACY_BLOCK_TYPE_DATA[256] = {

#include "LegacyBlockType.inc.cpp"

};


const BlockType * BlockType::lookup(const std::string &name) {
	auto it = Types.find(name);
	if (it == Types.end())
		return nullptr;

	return &it->second;
}

static LegacyPalette makeLegacyPalette() {
	const std::string name_prefix("minecraft:");

	LegacyPalette palette = {};
	for (size_t type = 0; type < 256; type++) {
		for (size_t data = 0; data < 16; data++) {
			const char *name = LEGACY_BLOCK_TYPE_DATA[type].data[data];
			if (!name)
				continue;

			palette.types[type][data] = BlockType::lookup(name_prefix + name);
		}
	}

	return palette;
}

const LegacyPalette LEGACY_BLOCK_TYPES = makeLegacyPalette();

}
}
