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


#include "Tag.hpp"

#include "EndTag.hpp"
#include "ByteTag.hpp"
#include "ShortTag.hpp"
#include "IntTag.hpp"
#include "LongTag.hpp"
#include "FloatTag.hpp"
#include "DoubleTag.hpp"
#include "ByteArrayTag.hpp"
#include "StringTag.hpp"
#include "ListTag.hpp"
#include "CompoundTag.hpp"
#include "IntArrayTag.hpp"
#include "LongArrayTag.hpp"


#include <stdexcept>


namespace MinedMap {
namespace NBT {

std::shared_ptr<const Tag> Tag::readTag(Type type, Buffer *buffer) {
	switch (type) {
	case Type::End:
		return std::make_shared<EndTag>();

	case Type::Byte:
		return std::make_shared<ByteTag>(buffer);

	case Type::Short:
		return std::make_shared<ShortTag>(buffer);

	case Type::Int:
		return std::make_shared<IntTag>(buffer);

	case Type::Long:
		return std::make_shared<LongTag>(buffer);

	case Type::Float:
		return std::make_shared<FloatTag>(buffer);

	case Type::Double:
		return std::make_shared<DoubleTag>(buffer);

	case Type::ByteArray:
		return std::make_shared<ByteArrayTag>(buffer);

	case Type::String:
		return std::make_shared<StringTag>(buffer);

	case Type::List:
		return readList(buffer);

	case Type::Compound:
		return std::make_shared<CompoundTag>(buffer);

	case Type::IntArray:
		return std::make_shared<IntArrayTag>(buffer);

	case Type::LongArray:
		return std::make_shared<LongArrayTag>(buffer);

	default:
		throw std::runtime_error("Tag::readTag: unknown tag type");
	}
}

std::shared_ptr<const Tag> Tag::readList(Buffer *buffer) {
	Type type = static_cast<Type>(buffer->get8());

	switch (type) {
	case Type::End:
		return std::make_shared<ListTag<EndTag>>(type, buffer);

	case Type::Byte:
		return std::make_shared<ListTag<ByteTag>>(type, buffer);

	case Type::Short:
		return std::make_shared<ListTag<ShortTag>>(type, buffer);

	case Type::Int:
		return std::make_shared<ListTag<IntTag>>(type, buffer);

	case Type::Long:
		return std::make_shared<ListTag<LongTag>>(type, buffer);

	case Type::Float:
		return std::make_shared<ListTag<FloatTag>>(type, buffer);

	case Type::Double:
		return std::make_shared<ListTag<DoubleTag>>(type, buffer);

	case Type::ByteArray:
		return std::make_shared<ListTag<ByteArrayTag>>(type, buffer);

	case Type::String:
		return std::make_shared<ListTag<StringTag>>(type, buffer);

	case Type::List:
		return std::make_shared<ListTag<ListTagBase>>(type, buffer);

	case Type::Compound:
		return std::make_shared<ListTag<CompoundTag>>(type, buffer);

	case Type::IntArray:
		return std::make_shared<ListTag<IntArrayTag>>(type, buffer);

	case Type::LongArray:
		return std::make_shared<ListTag<LongArrayTag>>(type, buffer);

	default:
		throw std::runtime_error("Tag::readList: unknown tag type");
	}
}

std::pair<std::string, std::shared_ptr<const Tag>> Tag::readNamedTag(Buffer *buffer) {
	Type type = static_cast<Type>(buffer->get8());
	if (type == Type::End)
		return std::make_pair("", std::shared_ptr<EndTag>(new EndTag()));

	uint16_t len = buffer->get16();
	std::string name(reinterpret_cast<const char*>(buffer->get(len)), len);

	return std::make_pair(name, readTag(type, buffer));
}

std::ostream& operator<<(std::ostream& os, Tag::Type type) {
	switch (type) {
	case Tag::Type::End:
		os << "End";
		break;

	case Tag::Type::Byte:
		os << "Byte";
		break;

	case Tag::Type::Short:
		os << "Short";
		break;

	case Tag::Type::Int:
		os << "Int";
		break;

	case Tag::Type::Long:
		os << "Long";
		break;

	case Tag::Type::Float:
		os << "Float";
		break;

	case Tag::Type::Double:
		os << "Double";
		break;

	case Tag::Type::ByteArray:
		os << "ByteArray";
		break;

	case Tag::Type::String:
		os << "String";
		break;

	case Tag::Type::List:
		os << "List";
		break;

	case Tag::Type::Compound:
		os << "Compound";
		break;

	case Tag::Type::IntArray:
		os << "IntArray";
		break;

	case Tag::Type::LongArray:
		os << "LongArray";
		break;

	default:
		os.setstate(std::ios_base::failbit);
	}

	return os;
}

}
}
