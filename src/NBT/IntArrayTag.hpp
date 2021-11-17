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

class IntArrayTag : public Tag {
private:
	uint32_t len;
	const uint8_t *ptr;

public:
	static const MakeType<IntArrayTag> Type;


	IntArrayTag(Buffer *buffer) {
		len = buffer->get32();
		ptr = buffer->get(4*len);
	}

	virtual const TagType & getType() const {
		return Type;
	}

	virtual void print(std::ostream& os, const std::string &indent) const {
		os << "(" << len << ") [" << std::endl;

		std::string inner = indent + "  ";

		for (size_t i = 0; i < len; i++) {
			uint32_t v = getValue(i);

			os << inner
			   << v << " / "
			   << (int32_t)v << " / "
			   << std::hex << "0x" << v << std::dec
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

	uint32_t getValue(size_t i) const {
		return Buffer::parse32(&ptr[4*i]);
	}
};

}
}
