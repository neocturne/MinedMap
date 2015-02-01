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

#include <cstdint>
#include <cstring>
#include <stdexcept>
#include <string>


namespace MinedMap {

class Buffer {
private:
	const uint8_t *data;
	size_t len;

public:
	static uint16_t parse16(const uint8_t *buf) {
		return (buf[0] << 8) | buf[1];
	}

	static uint32_t parse32(const uint8_t *buf) {
		return (uint32_t(buf[0]) << 24) | (uint32_t(buf[1]) << 16) | (uint32_t(buf[2]) << 8) | uint32_t(buf[3]);
	}

	static uint64_t parse64(const uint8_t *buf) {
		return (uint64_t(buf[0]) << 56) | (uint64_t(buf[1]) << 48) | (uint64_t(buf[2]) << 40) | (uint64_t(buf[3]) << 32)
			| (uint64_t(buf[4]) << 24) | (uint64_t(buf[5]) << 16) | (uint64_t(buf[6]) << 8) | uint64_t(buf[7]);
	}


	Buffer(const uint8_t *data0, size_t len0) : data(data0), len(len0) {}

	size_t getRemaining() const {
		return len;
	}

	const uint8_t * get(size_t n) {
		if (n > len)
			throw std::runtime_error("Buffer::get(): buffer underrun");

		data += n;
		len -= n;
		return (data - n);
	}

	uint8_t get8() {
		return *get(1);
	}

	uint16_t get16() {
		return parse16(get(2));
	}

	uint32_t get32() {
		return parse32(get(4));
	}

	uint64_t get64() {
		return parse64(get(8));
	}

};

}
