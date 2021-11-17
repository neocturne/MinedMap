// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include "../NBT/CompoundTag.hpp"

#include <cstdint>
#include <vector>


namespace MinedMap {
namespace World {

class Level {
private:
	std::vector<uint8_t> buffer;

	std::shared_ptr<const NBT::CompoundTag> root;
	std::shared_ptr<const NBT::CompoundTag> data;

public:
	Level(const char *filename);

	std::pair<int32_t, int32_t> getSpawn() const;
};

}
}
