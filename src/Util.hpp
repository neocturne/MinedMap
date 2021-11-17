// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include <algorithm>
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

}
