/*
  Copyright (c) 2018, Matthias Schiffer <mschiffer@universe-factory.net>
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
