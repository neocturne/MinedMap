// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include <climits>
#include <cstddef>
#include <cstdint>
#include <functional>
#include <map>
#include <set>
#include <tuple>
#include <utility>
#include <vector>


namespace MinedMap {

class Info {
public:
	typedef std::function<void (int, int)> RegionVisitor;

	struct Level {
		std::map<int, std::set<int>> regions;
		std::tuple<int, int, int, int> bounds;
	};

private:
	std::vector<Level> levels;

	int32_t spawnX, spawnZ;

public:
	Info() : spawnX(0), spawnZ(0) {
		addMipmapLevel();
	}

	std::tuple<int, int, int, int> getBounds(size_t level) const {
		return levels[level].bounds;
	}

	void addRegion(int x, int z, size_t level) {
		auto &the_level = levels[level];
		auto z_regions = the_level.regions.emplace(
			std::piecewise_construct,
			std::make_tuple(z),
			std::make_tuple()).first;
		z_regions->second.insert(x);

		std::tuple<int, int, int, int> &b = the_level.bounds;

		if (x < std::get<0>(b)) std::get<0>(b) = x;
		if (x > std::get<1>(b)) std::get<1>(b) = x;
		if (z < std::get<2>(b)) std::get<2>(b) = z;
		if (z > std::get<3>(b)) std::get<3>(b) = z;
	}

	void addMipmapLevel() {
		levels.emplace_back(Level {
			.regions = {},
			.bounds = {INT_MAX, INT_MIN, INT_MAX, INT_MIN},
		});
	}

	void visitRegions(size_t level, const RegionVisitor &visitor) const {
		for (const auto &item : levels[level].regions) {
			int z = item.first;
			for (int x : item.second)
				visitor(x, z);
		}
	}

	void setSpawn(const std::pair<int32_t, int32_t> &v) {
		std::tie(spawnX, spawnZ) = v;
	}

	void writeJSON(const char *filename) const;
};

}
