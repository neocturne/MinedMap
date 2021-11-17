// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include "EndTag.hpp"
#include "Tag.hpp"

#include <string>
#include <unordered_map>


namespace MinedMap {
namespace NBT {

class CompoundTag : public Tag, public std::unordered_map<std::string, std::shared_ptr<const Tag>> {
public:
	static const MakeType<CompoundTag> Type;


	CompoundTag(Buffer *buffer) {
		while (true) {
			std::pair<std::string, std::shared_ptr<const Tag>> v = Tag::readNamedTag(buffer);
			if (v.second->getType() == EndTag::Type)
				break;

			insert(std::move(v));
		}
	}

	virtual const TagType & getType() const {
		return Type;
	}

	virtual void print(std::ostream& os, const std::string &indent) const {
		os << "{" << std::endl;

		std::string inner = indent + "  ";

		for (const auto &item : *this) {
			os << inner << item.first << ": " << item.second->getType() << " ";
			item.second->print(os, inner);
			os << std::endl;
		}

		os << indent << "}";
	}

	template<typename T> std::shared_ptr<const T> get(const std::string &key) const {
		auto it = find(key);
		if (it == end())
			return std::shared_ptr<const T>();

		return std::dynamic_pointer_cast<const T>(it->second);
	}
};

}
}
