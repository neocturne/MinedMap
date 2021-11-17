// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015-2018, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include "Tag.hpp"

#include <vector>


namespace MinedMap {
namespace NBT {

class LongArrayTag : public Tag {
private:
	uint32_t len;
	const uint8_t *ptr;

public:
	static const MakeType<LongArrayTag> Type;


	LongArrayTag(Buffer *buffer) {
		len = buffer->get32();
		ptr = buffer->get(8*len);
	}

	virtual const TagType & getType() const {
		return Type;
	}

	virtual void print(std::ostream& os, const std::string &indent) const {
		os << "(" << len << ") [" << std::endl;

		std::string inner = indent + "  ";

		for (size_t i = 0; i < len; i++) {
			uint64_t v = Buffer::parse64(&ptr[8*i]);

			os << inner
			   << v << " / "
			   << (int64_t)v << " / "
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

	uint64_t getValue(size_t i) const {
		return Buffer::parse64(&ptr[8*i]);
	}
};

}
}
