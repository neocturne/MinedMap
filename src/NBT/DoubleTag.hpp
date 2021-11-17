// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include "Tag.hpp"


namespace MinedMap {
namespace NBT {

class DoubleTag : public Tag {
private:
	const uint8_t *ptr;

public:
	static const MakeType<DoubleTag> Type;


	DoubleTag(Buffer *buffer) {
		ptr = buffer->get(8);
	}

	virtual const TagType & getType() const {
		return Type;
	}

	virtual void print(std::ostream& os, const std::string &) const {
		union {
			uint64_t i;
			double d;
		};

		i = Buffer::parse64(ptr);
		os << d;
	}
};

}
}
