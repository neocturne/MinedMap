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

#include "Tag.hpp"

#include <vector>


namespace MinedMap {
namespace NBT {

class ByteArrayTag : public Tag {
private:
	uint32_t len;
	const uint8_t *value;

public:
	ByteArrayTag(Buffer *buffer) {
		len = buffer->get32();
		value = buffer->get(len);
	}
	virtual Type getType() const {
		return Type::ByteArray;
	}

	virtual void print(std::ostream& os, const std::string &indent) const {
		os << "(" << len << ") [" << std::endl;

		std::string inner = indent + "  ";

		for (size_t i = 0; i < len; i++) {
			uint8_t v = value[i];

			os << inner
			   << (unsigned)v << " / "
			   << (int)(int8_t)v << " / "
			   << std::hex << "0x" << (unsigned)v << std::dec
			   << std::endl;
		}

		os << indent << "]";
	}

	uint32_t getLength() const {
		return len;
	}

	const uint8_t * getValue() const {
		return value;
	}
};

}
}
