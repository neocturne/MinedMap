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


#include "World/Region.hpp"
#include "NBT/ListTag.hpp"

#include <cerrno>
#include <csetjmp>
#include <cstdio>
#include <iostream>
#include <system_error>

#include <arpa/inet.h>
#include <png.h>


using namespace MinedMap;


static const size_t DIM = World::Region::SIZE*World::Chunk::SIZE;


static void addChunk(uint32_t image[DIM][DIM], size_t X, size_t Z, const World::Chunk *chunk) {
	World::Chunk::Blocks layer = chunk->getTopLayer();

	for (size_t x = 0; x < World::Chunk::SIZE; x++) {
		for (size_t z = 0; z < World::Chunk::SIZE; z++)
			image[Z*World::Chunk::SIZE+z][X*World::Chunk::SIZE+x] = htonl(layer.blocks[x][z].getColor());
	}
}

static void writePNG(const char *filename, const uint32_t data[DIM][DIM]) {
	std::FILE *f = std::fopen(filename, "wb");
	if (!f)
		throw std::system_error(errno, std::generic_category(), "unable to open output file");

	png_structp png_ptr = png_create_write_struct (PNG_LIBPNG_VER_STRING, nullptr, nullptr, nullptr);
	if (!png_ptr)
		throw std::runtime_error("unable to create PNG write struct");

	png_infop info_ptr = png_create_info_struct(png_ptr);
	if (!info_ptr) {
		png_destroy_write_struct(&png_ptr, nullptr);
		throw std::runtime_error("unable to create PNG info struct");
	}

	if (setjmp(png_jmpbuf(png_ptr))) {
		png_destroy_write_struct(&png_ptr, &info_ptr);
		fclose(f);
		throw std::runtime_error("unable to write PNG file");
	}

	png_init_io(png_ptr, f);

	png_set_IHDR(png_ptr, info_ptr, DIM, DIM, 8, PNG_COLOR_TYPE_RGB_ALPHA,
		     PNG_INTERLACE_NONE, PNG_COMPRESSION_TYPE_DEFAULT, PNG_FILTER_TYPE_DEFAULT);

	uint8_t *row_pointers[World::Region::SIZE*World::Chunk::SIZE];
	for (size_t i = 0; i < World::Region::SIZE*World::Chunk::SIZE; i++)
		row_pointers[i] = const_cast<uint8_t*>(reinterpret_cast<const uint8_t*>(data[i]));

	png_set_rows(png_ptr, info_ptr, row_pointers);
	png_write_png(png_ptr, info_ptr, PNG_TRANSFORM_IDENTITY, NULL);

	std::fclose(f);
}

int main(int argc, char *argv[]) {
	if (argc < 3) {
		std::cerr << "Usage: " << argv[0] << " region output" << std::endl;
		return 1;
	}

	uint32_t image[DIM][DIM] = {};

	World::Region::visitChunks(argv[1], [&image] (size_t X, size_t Z, const World::Chunk *chunk) { addChunk(image, X, Z, chunk); });

	writePNG(argv[2], image);

	return 0;
}
