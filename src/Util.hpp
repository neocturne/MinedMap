// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015-2021, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include <algorithm>
#include <cstdint>
#include <limits>
#include <stdexcept>
#include <utility>


namespace MinedMap {

template <typename T> static inline T assertValue(T&& val) {
	if (!val)
		throw std::invalid_argument("assertValue failed");

	return std::forward<T>(val);
}

static inline float clamp(float v, float min, float max) {
	return std::max(std::min(v, max), min);
}

// A block coordinate relative to a chunk/section (X/Y/Z)
typedef uint8_t block_idx_t;
// A section index in a chunk (Y)
typedef int8_t section_idx_t;
// A chunk coordinate relative to a region (X/Z)
typedef uint8_t chunk_idx_t;

// A block coordinate relative to a region (X/Z)
typedef uint16_t region_block_idx_t;
// A block coordinate (Y)
typedef int16_t y_idx_t;

const y_idx_t y_idx_min = std::numeric_limits<y_idx_t>::min();

}
