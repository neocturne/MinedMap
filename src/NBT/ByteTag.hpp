// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include "Tag.hpp"


namespace MinedMap {
namespace NBT {

class ByteTag : public Tag {
private:
	uint8_t value;

public:
	static const MakeType<ByteTag> Type;


	ByteTag(Buffer *buffer) {
		value = buffer->get8();
	}

	virtual const TagType & getType() const {
		return Type;
	}

	virtual void print(std::ostream& os, const std::string &) const {
		os << (unsigned)getValue() << " / "
		   << (int)(int8_t)getValue() << " / "
		   << std::hex << "0x" << (unsigned)getValue() << std::dec;
	}

	uint8_t getValue() const {
		return value;
	}
};

}
}
