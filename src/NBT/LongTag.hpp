// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include "Tag.hpp"


namespace MinedMap {
namespace NBT {

class LongTag : public Tag {
private:
	const uint8_t *ptr;

public:
	static const MakeType<LongTag> Type;


	LongTag(Buffer *buffer) {
		ptr = buffer->get(8);
	}

	virtual const TagType & getType() const {
		return Type;
	}

	virtual void print(std::ostream& os, const std::string &) const {
		os << getValue() << " / "
		   << (int64_t)getValue() << " / "
		   << std::hex << "0x" << getValue() << std::dec;
	}

	uint64_t getValue() const {
		return Buffer::parse64(ptr);
	}
};

}
}
