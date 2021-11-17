// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include "Tag.hpp"


namespace MinedMap {
namespace NBT {

class IntTag : public Tag {
private:
	const uint8_t *ptr;

public:
	static const MakeType<IntTag> Type;


	IntTag(Buffer *buffer) {
		ptr = buffer->get(4);
	}

	virtual const TagType & getType() const {
		return Type;
	}

	virtual void print(std::ostream& os, const std::string &) const {
		os << getValue() << " / "
		   << (int32_t)getValue() << " / "
		   << std::hex << "0x" << getValue() << std::dec;
	}

	uint32_t getValue() const {
		return Buffer::parse32(ptr);
	}
};

}
}
