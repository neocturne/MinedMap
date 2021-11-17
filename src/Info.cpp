// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#include "Info.hpp"

#include <cerrno>
#include <cstdio>
#include <cstring>
#include <string>


namespace MinedMap {

void Info::writeJSON(const char *filename) const {
	const std::string tmpfile = std::string(filename) + ".tmp";

	FILE *f = std::fopen(tmpfile.c_str(), "w");
	if (!f) {
		std::fprintf(stderr, "Unable to open %s: %s\n", tmpfile.c_str(), std::strerror(errno));
		return;
	}

	std::fprintf(f, "{");
	std::fprintf(f, "\"mipmaps\":[");

	bool first_level = true;
	for (const auto &level : levels) {
		if (!first_level)
			std::fprintf(f, ",");
		first_level = false;

		int minX, maxX, minZ, maxZ;
		std::tie(minX, maxX, minZ, maxZ) = level.bounds;

		std::fprintf(f, "{");
		std::fprintf(f, "\"bounds\":{");
		std::fprintf(f, "\"minX\":%i,", minX);
		std::fprintf(f, "\"maxX\":%i,", maxX);
		std::fprintf(f, "\"minZ\":%i,", minZ);
		std::fprintf(f, "\"maxZ\":%i", maxZ);
		std::fprintf(f, "},");
		std::fprintf(f, "\"regions\":{");

		bool first_z = true;
		for (const auto &item : level.regions) {
			if (!first_z)
				std::fprintf(f, ",");
			first_z = false;

			int z = item.first;
			const std::set<int> &z_regions = item.second;

			std::fprintf(f, "\"%d\":[", z);

			bool first_x = true;
			for (int x : z_regions) {
				if (!first_x)
					std::fprintf(f, ",");
				first_x = false;


				std::fprintf(f, "%d", x);
			}

			std::fprintf(f, "]");
		}

		std::fprintf(f, "}}");
	}

	std::fprintf(f, "],");
	std::fprintf(f, "\"spawn\":{");
	std::fprintf(f, "\"x\":%li,", (long)spawnX);
	std::fprintf(f, "\"z\":%li", (long)spawnZ);
	std::fprintf(f, "}");
	std::fprintf(f, "}");

	std::fclose(f);

	if (std::rename(tmpfile.c_str(), filename) < 0) {
		std::fprintf(stderr, "Unable to save %s: %s\n", filename, std::strerror(errno));
		std::remove(tmpfile.c_str());
	}
}

}
