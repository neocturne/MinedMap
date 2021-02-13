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
