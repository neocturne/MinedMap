// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include "Tag.hpp"


namespace MinedMap {
namespace NBT {

class EndTag : public Tag {
public:
	static const MakeType<EndTag> Type;


	EndTag(Buffer *) {}

	virtual const TagType & getType() const {
		return Type;
	}

	virtual void print(std::ostream&, const std::string &) const {
	}
};

}
}
