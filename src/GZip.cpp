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
		throw std::system_error(errno, std::generic_category(), "unable to open GZip file");

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
