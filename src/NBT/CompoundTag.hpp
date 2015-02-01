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

#include <string>
#include <unordered_map>


namespace MinedMap {
namespace NBT {

class CompoundTag : public Tag, public std::unordered_map<std::string, std::shared_ptr<const Tag>> {
private:
	friend class Tag;

	CompoundTag(Buffer *buffer) {
		while (true) {
			std::pair<std::string, std::shared_ptr<const Tag>> v = Tag::readNamedTag(buffer);
			if (v.second->getType() == Type::End)
				break;

			insert(std::move(v));
		}
	}

public:
	virtual Type getType() const {
		return Type::Compound;
	}

	template<typename T>
	std::shared_ptr<const T> get(const std::string &key) const {
		return std::dynamic_pointer_cast<const T>(at(key));
	}

	template<typename T, typename... Args>
		std::shared_ptr<const T> get(const std::string &key, const Args &...args) const {
		std::shared_ptr<const CompoundTag> tag = get<CompoundTag>(key);
		if (!tag)
			return std::shared_ptr<const T>();

		return tag->get<T>(args...);
	}
};

}
}
