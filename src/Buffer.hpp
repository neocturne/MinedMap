// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
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
