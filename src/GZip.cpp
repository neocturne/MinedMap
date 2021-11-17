// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#include "GZip.hpp"

#include <system_error>
#include <stdexcept>
#include <zlib.h>


namespace MinedMap {

std::vector<uint8_t> readGZip(const char *filename) {
	std::vector<uint8_t> buffer;
	size_t len = 0;

	gzFile f = gzopen(filename, "rb");
	if (!f)
		throw std::system_error(
			errno, std::generic_category(),
			(std::string("unable to open file ") + filename).c_str()
		);

	while (true) {
		if ((buffer.size() - len) < 4096)
			buffer.resize(buffer.size() + 4096);

		int r = gzread(f, buffer.data()+len, buffer.size()-len);
		if (r < 0)
			throw std::system_error(errno, std::generic_category(), "error reading GZip file");

		if (!r)
			break;

		len += r;
	}

	gzclose_r(f);

	buffer.resize(len);

	return buffer;
}

}
