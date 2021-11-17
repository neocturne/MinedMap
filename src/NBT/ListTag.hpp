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

class ListTag : public Tag, public std::vector<std::shared_ptr<const Tag>> {
private:
	const TagType *subtype;

public:
	static const MakeType<ListTag> Type;


	ListTag(Buffer *buffer) {
		subtype = &getTypeById(buffer->get8());

		uint32_t len = buffer->get32();

		for (uint32_t i = 0; i < len; i++)
			push_back(subtype->read(buffer));
	}

	virtual const TagType & getType() const {
		return Type;
	}

	virtual const TagType & getSubtype() const {
		return *subtype;
	}

	virtual void print(std::ostream& os, const std::string &indent) const {
		os << getSubtype() << " [" << std::endl;

		std::string inner = indent + "  ";

		for (const auto &item : *this) {
			os << inner;
			item->print(os, inner);
			os << std::endl;
		}

		os << indent << "]";
	}
};

}
}
