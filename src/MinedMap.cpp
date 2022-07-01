// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015-2021, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
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
#include <unordered_map>
#include <stdexcept>
#include <sstream>
#include <system_error>

#include <sys/types.h>

#include <dirent.h>
#include <fcntl.h>
#include <sys/stat.h>


namespace MinedMap {

static const int BIOME_SMOOTH = 3;
static const uint32_t DIM = World::Region::SIZE*World::Chunk::SIZE;


static void addChunkBiome(uint8_t biomemap[DIM*DIM], chunk_idx_t X, chunk_idx_t Z, const World::ChunkData *data) {
	World::Chunk chunk(data);
	World::Chunk::Heightmap layer = chunk.getTopLayer(0);

	for (block_idx_t x = 0; x < World::Chunk::SIZE; x++) {
		for (block_idx_t z = 0; z < World::Chunk::SIZE; z++) {
			size_t i = (Z*World::Chunk::SIZE+z)*DIM + X*World::Chunk::SIZE+x;
			const World::Chunk::Height &height = layer.v[x][z];
			biomemap[i] = chunk.getBiome(x, height.y, z);
		}
	}
}

static uint8_t biomeAt(int16_t x, int16_t z, const std::unique_ptr<uint8_t[]> biomemaps[3][3]) {
	size_t a = 1, b = 1;

	if (x < 0) {
		a--;
		x += DIM;
	} else if (x >= (int32_t)DIM) {
		a++;
		x -= DIM;
	}
	if (z < 0) {
		b--;
		z += DIM;
	} else if (z >= (int32_t)DIM) {
		b++;
		z -= DIM;
	}

	return biomemaps[a][b].get()[z*DIM + x];
}

static Resource::Color collectColors(
	region_block_idx_t x, region_block_idx_t z,
	const World::Block &block, const std::unique_ptr<uint8_t[]> biomemaps[3][3]
) {
	std::unordered_map<uint8_t, unsigned> biomes;
	for (int16_t dx = -BIOME_SMOOTH; dx <= BIOME_SMOOTH; dx++) {
		for (int16_t dz = -BIOME_SMOOTH; dz <= BIOME_SMOOTH; dz++) {
			if (std::abs(dx) + std::abs(dz) > BIOME_SMOOTH)
				continue;

			uint8_t biome = biomeAt(x+dx, z+dz, biomemaps);
			if (biomes.count(biome))
				biomes[biome]++;
			else
				biomes[biome] = 1;
		}
	}

	Resource::FloatColor c = {};
	unsigned total = 0;

	for (const auto &e : biomes) {
		uint8_t biome = e.first;
		unsigned count = e.second;

		if (biome == 0xff)
			continue;

		c = c + count * block.getColor(biome);
		total += count;
	}

	if (!total)
		return block.getColor(0);

	return (1.0f / total) * c;
}

static void addChunk(Resource::Color image[DIM*DIM], uint8_t lightmap[2*DIM*DIM], chunk_idx_t X, chunk_idx_t Z,
	const World::ChunkData *data, const std::unique_ptr<uint8_t[]> biomemaps[3][3]
) {
	World::Chunk chunk(data);
	World::Chunk::Heightmap layer = chunk.getTopLayer(World::Chunk::WITH_DEPTH);

	for (block_idx_t x = 0; x < World::Chunk::SIZE; x++) {
		for (block_idx_t z = 0; z < World::Chunk::SIZE; z++) {
			size_t i = (Z*World::Chunk::SIZE+z)*DIM + X*World::Chunk::SIZE+x;
			const World::Chunk::Height &height = layer.v[x][z];
			World::Block block = chunk.getBlock(x, height, z);

			if (!block.isVisible())
				continue;

			image[i] = collectColors(X*World::Chunk::SIZE+x, Z*World::Chunk::SIZE+z, block, biomemaps);
			lightmap[2*i+1] = (1 - block.blockLight/15.f)*192;
		}
	}
}

static int64_t readStamp(const std::string &filename) {
	int64_t v = INT64_MIN;

	std::FILE *f = std::fopen((filename + ".stamp").c_str(), "r");
	if (f) {
		if (std::fscanf(f, "%" SCNd64, &v) != 1) {
			// Ignore errors
		}
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

static bool writeImage(const std::string &output, const uint8_t *data, PNG::Format format, int64_t t) {
	const std::string tmpfile = output + ".tmp";

	size_t len = PNG::formatBytes(format)*DIM*DIM;
	bool changed = true;

	try {
		std::unique_ptr<uint8_t[]> old(new uint8_t[len]);
		PNG::read(output.c_str(), old.get(), DIM, DIM, format);

		if (std::memcmp(data, old.get(), len) == 0)
			changed = false;
	} catch (const std::exception& ex) {
	}

	try {
		if (changed) {
			PNG::write(tmpfile.c_str(), data, DIM, DIM, format);

			if (std::rename(tmpfile.c_str(), output.c_str()) < 0) {
				std::fprintf(stderr, "Unable to save %s: %s\n", output.c_str(), std::strerror(errno));
				std::remove(tmpfile.c_str());
			}
		}

		writeStamp(output, t);
	} catch (const std::exception& ex) {
		std::remove(tmpfile.c_str());
		throw;
	}

	return changed;
}

static int64_t getModTime(const std::string &file) {
	struct stat s;
	if (stat(file.c_str(), &s) < 0) {
		if (errno != ENOENT)
			std::fprintf(stderr, "Unable to stat %s: %s\n", file.c_str(), std::strerror(errno));
		return INT64_MIN;
	}

#if defined(_WIN32)
	return (int64_t)s.st_mtime * 1000000;
#elif defined(__APPLE__)
	return (int64_t)s.st_mtimespec.tv_sec * 1000000 + s.st_mtimespec.tv_nsec / 1000;
#else
	return (int64_t)s.st_mtim.tv_sec * 1000000 + s.st_mtim.tv_nsec / 1000;
#endif
}

static bool checkRegion(int64_t changed, const std::string &file) {
	struct stat s;
	if (stat(file.c_str(), &s) < 0)
		return true;

	int64_t outtime = readStamp(file);
	if (changed <= outtime) {
		std::printf("%s is up-to-date.\n", file.c_str());
		return false;
	}

	return true;
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

static void makeBiome(const std::string &regiondir, const std::string &outputdir, int x, int z) {
	std::string inname = formatTileName(x, z, "mca");
	std::string outname = formatTileName(x, z, "png");
	std::string input = regiondir + "/" + inname, output = outputdir + "/biome/" + outname;

	int64_t intime = getModTime(input);
	if (intime == INT64_MIN)
		return;

	if (!checkRegion(intime, output))
		return;

	std::printf("Generating %s from %s... ", output.c_str(), input.c_str());
	std::fflush(stdout);

	try {
		std::unique_ptr<uint8_t[]> biomemap(new uint8_t[DIM*DIM]);
		std::memset(biomemap.get(), 0xff, DIM*DIM);

		World::Region::visitChunks(input.c_str(), [&] (chunk_idx_t X, chunk_idx_t Z, const World::ChunkData *chunk) {
			addChunkBiome(biomemap.get(), X, Z, chunk);
		});

		bool changed = writeImage(output, biomemap.get(), PNG::GRAY, intime);
		std::printf("%s.\n", changed ? "done" : "unchanged");
	} catch (const std::exception& ex) {
		std::fprintf(stderr, "Failed to generate %s: %s\n", output.c_str(), ex.what());
	}
}

static void makeBiomes(const std::string &regiondir, const std::string &outputdir, const Info *info) {
	info->visitRegions(0, [&] (int x, int z) {
		makeBiome(regiondir, outputdir, x, z);
	});
}

static void makeMap(const std::string &regiondir, const std::string &outputdir, int x, int z) {
	std::string inname = formatTileName(x, z, "mca");
	std::string outname = formatTileName(x, z, "png");
	std::string input = regiondir + "/" + inname;
	std::string output = outputdir + "/map/0/" + outname, output_light = outputdir + "/light/0/" + outname;
	std::string biomenames[3][3];

	int64_t intime = getModTime(input);
	if (intime == INT64_MIN)
		return;

	for (int i = 0; i < 3; i++) {
		for (int j = 0; j < 3; j++) {
			biomenames[i][j] = outputdir + "/biome/" + formatTileName(x + i - 1, z + j - 1, "png");
			intime = std::max(intime, getModTime(biomenames[i][j]));
		}
	}

	if (!checkRegion(intime, output))
		return;

	std::printf("Generating %s from %s... ", output.c_str(), input.c_str());
	std::fflush(stdout);

	try {
		std::unique_ptr<uint8_t[]> biomemaps[3][3];
		for (int i = 0; i < 3; i++) {
			for (int j = 0; j < 3; j++) {
				biomemaps[i][j].reset(new uint8_t[DIM*DIM]);
				std::memset(biomemaps[i][j].get(), 0, DIM*DIM);

				try {
					PNG::read(biomenames[i][j].c_str(), biomemaps[i][j].get(), DIM, DIM, PNG::GRAY);
				} catch (const std::exception& ex) {}
			}
		}

		std::unique_ptr<Resource::Color[]> image(new Resource::Color[DIM*DIM]);

		std::unique_ptr<uint8_t[]> lightmap(new uint8_t[2*DIM*DIM]);
		std::memset(lightmap.get(), 0, 2*DIM*DIM);

		World::Region::visitChunks(input.c_str(), [&] (chunk_idx_t X, chunk_idx_t Z, const World::ChunkData *chunk) {
			addChunk(image.get(), lightmap.get(), X, Z, chunk, biomemaps);
		});

		bool changed = writeImage(output, reinterpret_cast<const uint8_t*>(image.get()), PNG::RGB_ALPHA, intime);
		changed = writeImage(output_light, lightmap.get(), PNG::GRAY_ALPHA, intime) || changed;
		std::printf("%s.\n", changed ? "done" : "unchanged");
	} catch (const std::exception& ex) {
		std::fprintf(stderr, "Failed to generate %s: %s\n", output.c_str(), ex.what());
	}
}

static void makeMaps(const std::string &regiondir, const std::string &outputdir, const Info *info) {
	info->visitRegions(0, [&] (int x, int z) {
		makeMap(regiondir, outputdir, x, z);
	});
}

static bool makeMipmap(const std::string &dir, size_t level, int x, int z, PNG::Format imageFormat) {
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
	unsigned count = 0;
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
		PNG::mipmap(tmpfile.c_str(), DIM, DIM, imageFormat, nw, ne, sw, se);

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

static int floored_half(int a) {
	return (a - (a < 0)) / 2;
}

static void makeMipmaps(const std::string &dir, Info *info) {
	for (size_t level = 0; ; level++) {
		int minX, maxX, minZ, maxZ;
		std::tie(minX, maxX, minZ, maxZ) = info->getBounds(level);

		if (minX >= -1 && maxX <= 0 && minZ >= -1 && maxZ <= 0)
			break;

		info->addMipmapLevel();
		makeDir(dir + "/map/" + format(level + 1));
		makeDir(dir + "/light/" + format(level + 1));

		info->visitRegions(level, [&] (int x, int z) {
			info->addRegion(floored_half(x), floored_half(z), level + 1);
		});

		info->visitRegions(level + 1, [&] (int x, int z) {
			if (makeMipmap(dir + "/map", level + 1, x, z, PNG::RGB_ALPHA))
				info->addRegion(x, z, level + 1);

			makeMipmap(dir + "/light", level + 1, x, z, PNG::GRAY_ALPHA);
		});
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

	makeDir(outputdir + "/biome");
	makeDir(outputdir + "/map");
	makeDir(outputdir + "/map/0");
	makeDir(outputdir + "/light");
	makeDir(outputdir + "/light/0");

	Info info = collectInfo(regiondir);

	std::printf("Updating biome data...\n");
	makeBiomes(regiondir, outputdir, &info);

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
