// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#include "Level.hpp"
#include "../GZip.hpp"
#include "../Util.hpp"
#include "../NBT/IntTag.hpp"


namespace MinedMap {
namespace World {

Level::Level(const char *filename) {
	buffer = readGZip(filename);

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
