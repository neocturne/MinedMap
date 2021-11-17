// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2018, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#include "Buffer.hpp"
#include "GZip.hpp"
#include "Util.hpp"
#include "NBT/Tag.hpp"

#include <cstdio>
#include <iostream>
#include <stdexcept>


int main(int argc, char *argv[]) {
	using namespace MinedMap;

	if (argc != 2) {
		std::fprintf(stderr, "Usage: %s <nbtfile>\n", argv[0]);
		return 1;
	}

	std::vector<uint8_t> buffer = readGZip(argv[1]);

	Buffer nbt(buffer.data(), buffer.size());
	std::pair<std::string, std::shared_ptr<const NBT::Tag>> tag = NBT::Tag::readNamedTag(&nbt);
	if (tag.first != "")
		throw std::invalid_argument("invalid root tag");

	std::cout << *tag.second << std::endl;

	return 0;
}
