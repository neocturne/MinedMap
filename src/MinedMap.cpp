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
#include "PNG.hpp"
#include "World/Level.hpp"
#include "World/Region.hpp"

#include <cerrno>
#include <climits>
#include <cstdio>
#include <cstring>
#include <cstdlib>
#include <iostream>
#include <system_error>

#include <sys/types.h>

#include <arpa/inet.h>
#include <dirent.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/time.h>
#include <unistd.h>


namespace MinedMap {

static const size_t DIM = World::Region::SIZE*World::Chunk::SIZE;


static inline bool operator<(const struct timespec &t1, const struct timespec &t2) {
	return (t1.tv_sec < t2.tv_sec || (t1.tv_sec == t2.tv_sec && t1.tv_nsec < t2.tv_nsec));
}

static inline bool operator<=(const struct timespec &t1, const struct timespec &t2) {
	return (t1.tv_sec < t2.tv_sec || (t1.tv_sec == t2.tv_sec && t1.tv_nsec <= t2.tv_nsec));
}

static inline bool operator>(const struct timespec &t1, const struct timespec &t2) {
	return (t1.tv_sec > t2.tv_sec || (t1.tv_sec == t2.tv_sec && t1.tv_nsec > t2.tv_nsec));
}

static inline bool operator>=(const struct timespec &t1, const struct timespec &t2) {
	return (t1.tv_sec >= t2.tv_sec || (t1.tv_sec == t2.tv_sec && t1.tv_nsec >= t2.tv_nsec));
}

static inline bool operator==(const struct timespec &t1, const struct timespec &t2) {
	return (t1.tv_sec == t2.tv_sec && t1.tv_nsec == t2.tv_nsec);
}


static void addChunk(uint32_t image[DIM*DIM], uint8_t lightmap[2*DIM*DIM], size_t X, size_t Z, const World::Chunk *chunk) {
	World::Chunk::Blocks layer = chunk->getTopLayer();

	for (size_t x = 0; x < World::Chunk::SIZE; x++) {
		for (size_t z = 0; z < World::Chunk::SIZE; z++) {
			size_t i = (Z*World::Chunk::SIZE+z)*DIM + X*World::Chunk::SIZE+x;
			const World::Block &block = layer.blocks[x][z];

			image[i] = htonl(block.getColor());
			lightmap[2*i+1] = (1 - block.getBlockLight()/15.f)*128;
		}
	}
}

static void writeImage(const std::string &output, const uint8_t *data, bool colored, const struct timespec *t) {
	const std::string tmpfile = output + ".tmp";

	try {
		PNG::write(tmpfile.c_str(), data, DIM, DIM, colored);

		struct timespec times[2] = {*t, *t};
		if (utimensat(AT_FDCWD, tmpfile.c_str(), times, 0) < 0)
			std::fprintf(stderr, "Warning: failed to set utime on %s: %s\n", tmpfile.c_str(), std::strerror(errno));

		if (std::rename(tmpfile.c_str(), output.c_str()) < 0) {
			std::fprintf(stderr, "Unable to save %s: %s\n", output.c_str(), std::strerror(errno));
			unlink(tmpfile.c_str());
		}
	}
	catch (const std::exception& ex) {
		unlink(tmpfile.c_str());
		throw;
	}
}

static void doRegion(const std::string &input, const std::string &output, const std::string &output_light) {
	struct stat instat, outstat;

	if (stat(input.c_str(), &instat) < 0) {
		std::fprintf(stderr, "Unable to stat %s: %s\n", input.c_str(), std::strerror(errno));
		return;
	}

	if (stat(output.c_str(), &outstat) == 0) {
		if (instat.st_mtim <= outstat.st_mtim) {
			std::printf("%s is up-to-date.\n", output.c_str());
			return;
		}
	}

	std::printf("Generating %s from %s...\n", output.c_str(), input.c_str());

	try {
		uint32_t image[DIM*DIM] = {};
		uint8_t lightmap[2*DIM*DIM] = {};
		World::Region::visitChunks(input.c_str(), [&] (size_t X, size_t Z, const World::Chunk *chunk) { addChunk(image, lightmap, X, Z, chunk); });

		writeImage(output, reinterpret_cast<const uint8_t*>(image), true, &instat.st_mtim);
		writeImage(output_light, lightmap, false, &instat.st_mtim);
	}
	catch (const std::exception& ex) {
		std::fprintf(stderr, "Failed to generate %s: %s\n", output.c_str(), ex.what());
	}
}

static bool checkFilename(const char *name, int *x, int *z) {
	if (std::sscanf(name, "r.%i.%i.mca", x, z) != 2)
		return false;

	size_t l = strlen(name) + 1;
	char buf[l];
	std::snprintf(buf, l, "r.%i.%i.mca", *x, *z);
	if (std::memcmp(name, buf, l))
		return false;

	return true;

}

static void makeDir(const std::string &name) {
	if (mkdir(name.c_str(), 0777) < 0 && errno != EEXIST)
		throw std::system_error(errno, std::generic_category(), "unable to create directory " + name);
}

static void makeMipmaps(const std::string &outputdir, const Info &info) {
}

}


int main(int argc, char *argv[]) {
	using namespace MinedMap;


	if (argc < 3) {
		std::fprintf(stderr, "Usage: %s <data directory> <output directory>\n", argv[0]);
		return 1;
	}

	std::string inputdir(argv[1]);
	std::string regiondir = inputdir + "/region";

	std::string outputdir(argv[2]);

	makeDir(outputdir + "/map");
	makeDir(outputdir + "/map/0");
	makeDir(outputdir + "/light");
	makeDir(outputdir + "/light/0");

	DIR *dir = opendir(regiondir.c_str());
	if (!dir) {
		std::fprintf(stderr, "Unable to read input directory: %s\n", std::strerror(errno));
		return 1;
	}

	Info info;

	struct dirent *entry;
	while ((entry = readdir(dir)) != nullptr) {
		int x, z;
		if (!checkFilename(entry->d_name, &x, &z))
			continue;

		info.addRegion(x, z);

		std::string name(entry->d_name), outname = name.substr(0, name.length()-3) + "png";
		doRegion(regiondir + "/" + name, outputdir + "/map/0/" + outname, outputdir + "/light/0/" + outname);
	}

	closedir(dir);

	World::Level level((inputdir + "/level.dat").c_str());
	info.setSpawn(level.getSpawn());

	makeMipmaps(outputdir, info);

	info.writeJSON((outputdir + "/info.json").c_str());

	return 0;
}
