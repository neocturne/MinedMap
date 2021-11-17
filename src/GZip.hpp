// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include <cstdint>
#include <vector>


namespace MinedMap {

std::vector<uint8_t> readGZip(const char *filename);

}
