// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include "Tag.hpp"

#include <vector>


namespace MinedMap {
namespace NBT {

class ByteArrayTag : public Tag {
private:
	uint32_t len;
	const uint8_t *ptr;

public:
	static const MakeType<ByteArrayTag> Type;


	ByteArrayTag(Buffer *buffer) {
		len = buffer->get32();
		ptr = buffer->get(len);
	}

	virtual const TagType & getType() const {
		return Type;
	}

	virtual void print(std::ostream& os, const std::string &indent) const {
		os << "(" << len << ") [" << std::endl;

		std::string inner = indent + "  ";

		for (size_t i = 0; i < len; i++) {
			uint8_t v = ptr[i];

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

	const uint8_t * getPointer() const {
		return ptr;
	}

	uint8_t getValue(size_t i) const {
		return ptr[i];
	}
};

}
}
