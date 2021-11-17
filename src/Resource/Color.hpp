// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2018, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include <cstdint>


namespace MinedMap {
namespace Resource {

struct FloatColor {
	float r, g, b;
};

static inline FloatColor operator+(const FloatColor &a, const FloatColor &b){
	return FloatColor {
		a.r+b.r,
		a.g+b.g,
		a.b+b.b,
	};
}

static inline FloatColor & operator*=(FloatColor &a, const FloatColor &b) {
	a.r *= b.r;
	a.g *= b.g;
	a.b *= b.b;

	return a;
}

static inline FloatColor operator*(float s, const FloatColor &c) {
	return FloatColor {
		s*c.r,
		s*c.g,
		s*c.b,
	};
}

struct Color {
	uint8_t r, g, b, a;

	Color() : r(0), g(0), b(0), a(0) {}
	Color(FloatColor c) : r(c.r), g(c.g), b(c.b), a(0xff) {}
};

}
}
