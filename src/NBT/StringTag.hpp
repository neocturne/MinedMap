// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include "Tag.hpp"


namespace MinedMap {
namespace NBT {

class StringTag : public Tag {
private:
	uint16_t len;
	const uint8_t *ptr;

public:
	static const MakeType<StringTag> Type;


	StringTag(Buffer *buffer) {
		len = buffer->get16();
		ptr = buffer->get(len);
	}

	virtual const TagType & getType() const {
		return Type;
	}

	std::string getValue() const {
		return std::string(reinterpret_cast<const char *>(ptr), len);
	}

	virtual void print(std::ostream& os, const std::string &) const {
		os << "\"" << getValue() << "\"";
	}
};

}
}
