/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.

  Redistribution and use in source and binary forms, with or without
  modification, are permitted provided that the following conditions are met:

    1. Redistributions of source code must retain the above copyright notice,
       this list of conditions and the following disclaimer.
    2. Redistributions in binary form must reproduce the above copyright notice,
       this list of conditions and the following disclaimer in the documentation
       and/or other materials provided with the distribution.

  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
  AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
  IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
  FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
  DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
  SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
  CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
  OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
  OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
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
