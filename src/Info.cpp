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


#include "Info.hpp"

#include <cerrno>
#include <cstdio>
#include <cstring>
#include <string>
#include <unistd.h>


namespace MinedMap {

void Info::writeJSON(const char *filename) const {
	const std::string tmpfile = std::string(filename) + ".tmp";

	FILE *f = fopen(tmpfile.c_str(), "w");
	if (!f) {
		std::fprintf(stderr, "Unable to open %s: %s\n", tmpfile.c_str(), std::strerror(errno));
		return;
	}

	fprintf(f, "{\n");
	fprintf(f, "  \"mipmaps\" : [\n");

	for (size_t level = 0; level < regions.size(); level++) {
		int minX, maxX, minZ, maxZ;
		std::tie(minX, maxX, minZ, maxZ) = getBounds(level);

		fprintf(f, "    {\n");
		fprintf(f, "      \"info\" : {\n");
		fprintf(f, "        \"minX\" : %i,\n", minX);
		fprintf(f, "        \"maxX\" : %i,\n", maxX);
		fprintf(f, "        \"minZ\" : %i,\n", minZ);
		fprintf(f, "        \"maxZ\" : %i\n", maxZ);
		fprintf(f, "      },\n");
		fprintf(f, "      \"regions\" : [\n");

		for (int z = minZ; z <= maxZ; z++) {
			fprintf(f, "        [");

			for (int x = minX; x <= maxX; x++) {
				fprintf(f, "%s", regions[level].count(std::make_pair(x, z)) ? "true" : "false");

				if (x < maxX)
					fprintf(f, ", ");
			}

			if (z < maxZ)
				fprintf(f, "],\n");
			else
				fprintf(f, "]\n");
		}

		fprintf(f, "      ]\n");

		if (level < regions.size() - 1)
			fprintf(f, "    },\n");
		else
			fprintf(f, "    }\n");
	}

	fprintf(f, "  ],\n");
	fprintf(f, "  \"spawn\" : {\n");
	fprintf(f, "    \"x\" : %li,\n", (long)spawnX);
	fprintf(f, "    \"z\" : %li\n", (long)spawnZ);
	fprintf(f, "  }\n");
	fprintf(f, "}\n");

	fclose(f);

	if (std::rename(tmpfile.c_str(), filename) < 0) {
		std::fprintf(stderr, "Unable to save %s: %s\n", filename, std::strerror(errno));
		unlink(tmpfile.c_str());
	}
}

}
