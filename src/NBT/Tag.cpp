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


#include <stdexcept>


namespace MinedMap {
namespace NBT {

std::shared_ptr<Tag> Tag::readTag(Type type, Buffer *buffer) {
	switch (type) {
	case Type::End:
		return std::shared_ptr<EndTag>(new EndTag());

	case Type::Byte:
		return std::shared_ptr<ByteTag>(new ByteTag(buffer));

	case Type::Short:
		return std::shared_ptr<ShortTag>(new ShortTag(buffer));

	case Type::Int:
		return std::shared_ptr<IntTag>(new IntTag(buffer));

	case Type::Long:
		return std::shared_ptr<LongTag>(new LongTag(buffer));

	case Type::Float:
		return std::shared_ptr<FloatTag>(new FloatTag(buffer));

	case Type::Double:
		return std::shared_ptr<DoubleTag>(new DoubleTag(buffer));

	case Type::ByteArray:
		return std::shared_ptr<ByteArrayTag>(new ByteArrayTag(buffer));

	case Type::String:
		return std::shared_ptr<StringTag>(new StringTag(buffer));

	case Type::List:
		return std::shared_ptr<ListTag>(new ListTag(buffer));

	case Type::Compound:
		return std::shared_ptr<CompoundTag>(new CompoundTag(buffer));

	case Type::IntArray:
		return std::shared_ptr<IntArrayTag>(new IntArrayTag(buffer));

	default:
		throw std::runtime_error("Tag::read: unknown tag type");
	}
}

std::pair<std::string, std::shared_ptr<Tag>> Tag::readNamedTag(Buffer *buffer) {
	Type type = static_cast<Type>(buffer->get());
	if (type == Type::End)
		return std::make_pair("", std::shared_ptr<EndTag>(new EndTag()));

	uint16_t len = buffer->get() << 8;
	len |= buffer->get();
	std::string name = buffer->getString(len);

	return std::make_pair(name, readTag(type, buffer));
}

}
}
