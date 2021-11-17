// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
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
