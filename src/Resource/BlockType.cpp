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
