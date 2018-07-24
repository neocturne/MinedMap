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
#include "../Util.hpp"


namespace MinedMap {
namespace World {

Section::Section(const std::shared_ptr<const NBT::CompoundTag> &section) {
	Y = assertValue(section->get<NBT::ByteTag>("Y"))->getValue();
	blockLight = section->get<NBT::ByteArrayTag>("BlockLight");
}


std::unique_ptr<Section> Section::makeSection(const std::shared_ptr<const NBT::CompoundTag> &section) {
	std::shared_ptr<const NBT::ByteArrayTag> blocks = assertValue(section->get<NBT::ByteArrayTag>("Blocks"));
	std::shared_ptr<const NBT::ByteArrayTag> data = assertValue(section->get<NBT::ByteArrayTag>("Data"));

	return std::unique_ptr<Section>(new LegacySection(section, std::move(blocks), std::move(data)));
}

const Resource::BlockType * LegacySection::getBlockStateAt(size_t x, size_t y, size_t z) const {
	uint8_t type = getBlockAt(x, y, z);
	uint8_t data = getDataAt(x, y, z);

	return Resource::LEGACY_BLOCK_TYPES.types[type][data];
}

}
}
