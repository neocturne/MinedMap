/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
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


#include "Level.hpp"
#include "../GZip.hpp"
#include "../Util.hpp"
#include "../NBT/IntTag.hpp"

#include <iostream>


namespace MinedMap {
namespace World {

Level::Level(const char *filename) {
	std::vector<uint8_t> buffer = readGZip(filename);

	Buffer nbt(buffer.data(), buffer.size());
	std::pair<std::string, std::shared_ptr<const NBT::Tag>> tag = NBT::Tag::readNamedTag(&nbt);
	if (tag.first != "")
		throw std::invalid_argument("invalid root tag");

	root = assertValue(std::dynamic_pointer_cast<const NBT::CompoundTag>(tag.second));
	data = assertValue(root->get<NBT::CompoundTag>("Data"));
}

std::pair<int32_t, int32_t> Level::getSpawn() const {
	int32_t x = assertValue(data->get<NBT::IntTag>("SpawnX"))->getValue();
	int32_t z = assertValue(data->get<NBT::IntTag>("SpawnZ"))->getValue();

	return std::make_pair(x, z);
}

}
}
