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
#include <cinttypes>
#include <climits>
#include <cstdio>
#include <cstring>
#include <cstdlib>
#include <iostream>
#include <stdexcept>
#include <sstream>
#include <system_error>

#include <sys/types.h>

#include <dirent.h>
#include <fcntl.h>
#include <sys/stat.h>


namespace MinedMap {

static const size_t DIM = World::Region::SIZE*World::Chunk::SIZE;


static void addChunk(Resource::Color image[DIM*DIM], uint8_t lightmap[2*DIM*DIM], size_t X, size_t Z, const World::ChunkData *data) {
	World::Chunk chunk(data);
	World::Chunk::Blocks layer = chunk.getTopLayer();

	for (size_t x = 0; x < World::Chunk::SIZE; x++) {
		for (size_t z = 0; z < World::Chunk::SIZE; z++) {
			size_t i = (Z*World::Chunk::SIZE+z)*DIM + X*World::Chunk::SIZE+x;
			const World::Block &block = layer.blocks[x][z];

			image[i] = block.getColor();
			lightmap[2*i+1] = (1 - block.blockLight/15.f)*192;
		}
	}
}

static int64_t readStamp(const std::string &filename) {
	int64_t v = INT64_MIN;

	std::FILE *f = std::fopen((filename + ".stamp").c_str(), "r");
	if (f) {
		std::fscanf(f, "%" SCNd64, &v);
		std::fclose(f);
	}

	return v;
}

static void writeStamp(const std::string &filename, int64_t v) {
	std::FILE *f = std::fopen((filename + ".stamp").c_str(), "w");
	if (f) {
		std::fprintf(f, "%" PRId64, v);
		std::fclose(f);
	}
}

static void writeImage(const std::string &output, const uint8_t *data, bool colored, int64_t t) {
	const std::string tmpfile = output + ".tmp";

	try {
		PNG::write(tmpfile.c_str(), data, DIM, DIM, colored);

		if (std::rename(tmpfile.c_str(), output.c_str()) < 0) {
			std::fprintf(stderr, "Unable to save %s: %s\n", output.c_str(), std::strerror(errno));
			std::remove(tmpfile.c_str());
		}

		writeStamp(output, t);
	} catch (const std::exception& ex) {
		std::remove(tmpfile.c_str());
		throw;
	}
}

static void doRegion(const std::string &input, const std::string &output, const std::string &output_light) {
	int64_t intime;

	{
		struct stat instat;

		if (stat(input.c_str(), &instat) < 0) {
			if (errno != ENOENT)
				std::fprintf(stderr, "Unable to stat %s: %s\n", input.c_str(), std::strerror(errno));
			return;
		}

#ifdef _WIN32
		intime = (int64_t)instat.st_mtime * 1000000;
#else
		intime = (int64_t)instat.st_mtim.tv_sec * 1000000 + instat.st_mtim.tv_nsec / 1000;
#endif
	}

	{
		struct stat s;
		if (stat(output.c_str(), &s) == 0) {
			int64_t outtime = readStamp(output);
			if (intime <= outtime) {
				std::printf("%s is up-to-date.\n", output.c_str());
				return;
			}
		}
	}

	std::printf("Generating %s from %s...\n", output.c_str(), input.c_str());

	try {
		std::unique_ptr<Resource::Color[]> image(new Resource::Color[DIM*DIM]);
		std::memset(image.get(), 0, 4*DIM*DIM);

		std::unique_ptr<uint8_t[]> lightmap(new uint8_t[2*DIM*DIM]);
		std::memset(lightmap.get(), 0, 2*DIM*DIM);

		World::Region::visitChunks(input.c_str(), [&] (size_t X, size_t Z, const World::ChunkData *chunk) { addChunk(image.get(), lightmap.get(), X, Z, chunk); });

		writeImage(output, reinterpret_cast<const uint8_t*>(image.get()), true, intime);
		writeImage(output_light, lightmap.get(), false, intime);
	} catch (const std::exception& ex) {
		std::fprintf(stderr, "Failed to generate %s: %s\n", output.c_str(), ex.what());
	}
}

template<typename T>
static std::string format(const T &v) {
	std::ostringstream s;

	s << v;
	return s.str();
}

static std::string formatTileName(int x, int z, const std::string &ext) {
	std::ostringstream s;

	s << "r." << x << "." << z << "." << ext;
	return s.str();
}

static bool checkFilename(const char *name, int *x, int *z) {
	if (std::sscanf(name, "r.%i.%i.mca", x, z) != 2)
		return false;

	return (std::string(name) == formatTileName(*x, *z, "mca"));
}

static void makeDir(const std::string &name) {
	if (
		mkdir(
			name.c_str()
#ifndef _WIN32
			, 0777
#endif
		) < 0 && errno != EEXIST
	)
		throw std::system_error(errno, std::generic_category(), "unable to create directory " + name);
}

static void makeMap(const std::string &regiondir, const std::string &outputdir, size_t x, size_t z) {
	std::string inname = formatTileName(x, z, "mca");
	std::string outname = formatTileName(x, z, "png");
	doRegion(regiondir + "/" + inname, outputdir + "/map/0/" + outname, outputdir + "/light/0/" + outname);
}

static void makeMaps(const std::string &regiondir, const std::string &outputdir, const Info *info) {
	int minX, maxX, minZ, maxZ;
	std::tie(minX, maxX, minZ, maxZ) = info->getBounds(0);

	for (int x = minX; x <= maxX; x++) {
		for (int z = minZ; z <= maxZ; z++)
			makeMap(regiondir, outputdir, x, z);
	}
}

static bool makeMipmap(const std::string &dir, size_t level, size_t x, size_t z, bool colored) {
	bool ret = false;

	std::string indir = dir + "/" + format(level-1) + "/";
	std::string outdir = dir + "/" + format(level) + "/";

	const std::string nw_str = indir + formatTileName(x*2, z*2, "png");
	const std::string ne_str = indir + formatTileName(x*2+1, z*2, "png");
	const std::string sw_str = indir + formatTileName(x*2, z*2+1, "png");
	const std::string se_str = indir + formatTileName(x*2+1, z*2+1, "png");

	const char *nw = nw_str.c_str();
	const char *ne = ne_str.c_str();
	const char *sw = sw_str.c_str();
	const char *se = se_str.c_str();

	int64_t t = INT64_MIN;
	size_t count = 0;
	for (auto name : {&nw, &ne, &sw, &se}) {
		struct stat s;
		if (stat(*name, &s) < 0) {
			*name = nullptr;
			continue;
		}

		int64_t t_part = readStamp(*name);
		if (t_part > t)
			t = t_part;

		count++;
	}

	std::string output = outdir + formatTileName(x, z, "png");

	{
		struct stat s;
		if (stat(output.c_str(), &s) == 0) {
			ret = true;

			int64_t outtime = readStamp(output);
			if (t <= outtime)
				return ret;
		}
	}

	if (!count)
		return ret;

	const std::string tmpfile = output + ".tmp";

	try {
		PNG::mipmap(tmpfile.c_str(), DIM, DIM, colored, nw, ne, sw, se);

		if (std::rename(tmpfile.c_str(), output.c_str()) < 0) {
			std::fprintf(stderr, "Unable to save %s: %s\n", output.c_str(), std::strerror(errno));
			std::remove(tmpfile.c_str());
		}

		writeStamp(output, t);
	} catch (const std::exception& ex) {
		std::remove(tmpfile.c_str());
		throw;
	}

	return true;
}

static void makeMipmaps(const std::string &dir, Info *info) {
	int minX, maxX, minZ, maxZ;
	std::tie(minX, maxX, minZ, maxZ) = info->getBounds(0);

	while (minX < -1 || maxX > 0 || minZ < -1 || maxZ > 0) {
		info->addMipmapLevel();
		size_t level = info->getMipmapLevel();
		makeDir(dir + "/map/" + format(level));
		makeDir(dir + "/light/" + format(level));

		minX = (minX-1)/2;
		maxX = maxX/2;
		minZ = (minZ-1)/2;
		maxZ = maxZ/2;

		for (int x = minX; x <= maxX; x++) {
			for (int z = minZ; z <= maxZ; z++) {
				if (makeMipmap(dir + "/map", level, x, z, true))
					info->addRegion(x, z, level);

				makeMipmap(dir + "/light", level, x, z, false);
			}
		}
	}
}

static Info collectInfo(const std::string &regiondir) {
	DIR *dir = opendir(regiondir.c_str());
	if (!dir)
		throw std::system_error(errno, std::generic_category(), "Unable to read input directory");

	Info info;

	struct dirent *entry;
	while ((entry = readdir(dir)) != nullptr) {
		int x, z;
		if (!checkFilename(entry->d_name, &x, &z))
			continue;

		info.addRegion(x, z, 0);

	}

	closedir(dir);

	return info;
}

static void doLevel(const std::string &inputdir, const std::string &outputdir) {
	const std::string regiondir = inputdir + "/region";

	makeDir(outputdir + "/map");
	makeDir(outputdir + "/map/0");
	makeDir(outputdir + "/light");
	makeDir(outputdir + "/light/0");

	Info info = collectInfo(regiondir);

	std::printf("Updating map data...\n");
	makeMaps(regiondir, outputdir, &info);

	World::Level level((inputdir + "/level.dat").c_str());
	info.setSpawn(level.getSpawn());

	std::printf("Updating mipmaps...\n");
	makeMipmaps(outputdir, &info);

	info.writeJSON((outputdir + "/info.json").c_str());
}

}


int main(int argc, char *argv[]) {
	if (argc < 3) {
		std::fprintf(stderr, "Usage: %s <data directory> <output directory>\n", argv[0]);
		return 1;
	}

	try {
		MinedMap::doLevel(argv[1], argv[2]);
	} catch (const std::runtime_error& ex) {
		std::fprintf(stderr, "Error: %s\n", ex.what());
		return 1;
	}

	return 0;
}
