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

#include <cstdint>
#include <memory>
#include <ostream>
#include <vector>

#include "../Buffer.hpp"


namespace MinedMap {
namespace NBT {

class Tag;

class TagType {
public:
	TagType() = default;
	TagType(const TagType&) = delete;
	TagType & operator=(const TagType&) = delete;

	virtual const char * getName() const = 0;
	virtual std::shared_ptr<const Tag> read(Buffer *buffer) const = 0;

	bool operator==(const TagType &type) const {
		return this == &type;
	}
};

class Tag {
private:
	static const std::vector<const TagType *> types;

protected:
	template<typename T>
	class MakeType : public TagType {
	private:
		const char *name;

	public:
		MakeType(const char *name0) : name(name0) {}

		virtual const char * getName() const {
			return name;
		}

		virtual std::shared_ptr<const Tag> read(Buffer *buffer) const {
			return std::make_shared<T>(buffer);
		}
	};


	static const TagType & getTypeById(uint8_t id) {
		return *types.at(id);
	}

public:
	static std::pair<std::string, std::shared_ptr<const Tag>> readNamedTag(Buffer *buffer);

	virtual const TagType & getType() const = 0;
	virtual void print(std::ostream& os, const std::string &indent) const = 0;

	virtual ~Tag() {}
};

static inline std::ostream& operator<<(std::ostream& os, const TagType &type) {
	return os << type.getName();
}

static inline std::ostream& operator<<(std::ostream& os, const Tag &tag) {
	os << tag.getType() << " ";
	tag.print(os, "");
	return os;
}

}
}
