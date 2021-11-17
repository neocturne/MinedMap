// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include "Tag.hpp"


namespace MinedMap {
namespace NBT {

class FloatTag : public Tag {
	const uint8_t *ptr;

public:
	static const MakeType<FloatTag> Type;


	FloatTag(Buffer *buffer) {
		ptr = buffer->get(4);
	}

	virtual const TagType & getType() const {
		return Type;
	}

	virtual void print(std::ostream& os, const std::string &) const {
		union {
			uint32_t i;
			float f;
		};

		i = Buffer::parse32(ptr);
		os << f;
	}
};

}
}
