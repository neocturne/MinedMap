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


#include "Block.hpp"
#include "../Resource/Biome.hpp"
#include "../Resource/BlockType.hpp"


namespace MinedMap {
namespace World {

uint32_t Block::getColor() const {
	const Resource::BlockType *type = Resource::LEGACY_BLOCK_TYPES.types[id][data];
	if (!type || !type->opaque)
		return 0;

	float r = type->color.r;
	float g = type->color.g;
	float b = type->color.b;

	float heightCoef = height/255.0f + 0.5f;

	r *= heightCoef;
	g *= heightCoef;
	b *= heightCoef;

	if (type->green) {
		const Resource::Biome &biomeDef = Resource::BIOMES[biome];

		r *= biomeDef.r;
		g *= biomeDef.g;
		b *= biomeDef.b;
	}

	if (type->blue) {
		r *= 0.265;
		g *= 0.382;
		b *= 1.379;
	}

	if (r > 255) r = 255;
	if (g > 255) g = 255;
	if (b > 255) b = 255;

	return ((unsigned)r << 24) | ((unsigned)g << 16) | ((unsigned)b << 8) | 0xff;
}

}
}
