// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include "Tag.hpp"


namespace MinedMap {
namespace NBT {

class ShortTag : public Tag {
private:
	const uint8_t *ptr;

public:
	static const MakeType<ShortTag> Type;


	ShortTag(Buffer *buffer) {
		ptr = buffer->get(2);
	}

	virtual const TagType & getType() const {
		return Type;
	}

	virtual void print(std::ostream& os, const std::string &) const {
		os << getValue() << " / "
		   << (int16_t)getValue() << " / "
		   << std::hex << "0x" << getValue() << std::dec;
	}

	uint16_t getValue() const {
		return Buffer::parse16(ptr);
	}
};

}
}
