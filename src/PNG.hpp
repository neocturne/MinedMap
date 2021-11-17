// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include <cstddef>
#include <cstdint>


namespace MinedMap {
namespace PNG {

enum Format {
	RGB_ALPHA,
	GRAY_ALPHA,
	GRAY,
};

static inline size_t formatBytes(Format format) {
	const size_t data[] = {
		[RGB_ALPHA] = 4,
		[GRAY_ALPHA] = 2,
		[GRAY] = 1,
	};

	return data[format];
}

void write(const char *filename, const uint8_t *data, size_t width, size_t height, Format format);
void read(const char *filename, uint8_t *data, size_t width, size_t height, Format format);
void mipmap(const char *output, size_t width, size_t height, Format format, const char *nw, const char *ne, const char *sw, const char *se);

}
}
