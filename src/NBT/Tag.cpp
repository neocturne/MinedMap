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


namespace MinedMap {
namespace NBT {

const Tag::MakeType<EndTag> EndTag::Type("End");
const Tag::MakeType<ByteTag> ByteTag::Type("Byte");
const Tag::MakeType<ShortTag> ShortTag::Type("Short");
const Tag::MakeType<IntTag> IntTag::Type("Int");
const Tag::MakeType<LongTag> LongTag::Type("Long");
const Tag::MakeType<FloatTag> FloatTag::Type("Float");
const Tag::MakeType<DoubleTag> DoubleTag::Type("Double");
const Tag::MakeType<ByteArrayTag> ByteArrayTag::Type("ByteArray");
const Tag::MakeType<StringTag> StringTag::Type("String");
const Tag::MakeType<ListTag> ListTag::Type("List");
const Tag::MakeType<CompoundTag> CompoundTag::Type("Compound");
const Tag::MakeType<IntArrayTag> IntArrayTag::Type("IntArray");
const Tag::MakeType<LongArrayTag> LongArrayTag::Type("LongArray");


const std::vector<const TagType *> Tag::types = {
	&EndTag::Type,
	&ByteTag::Type,
	&ShortTag::Type,
	&IntTag::Type,
	&LongTag::Type,
	&FloatTag::Type,
	&DoubleTag::Type,
	&ByteArrayTag::Type,
	&StringTag::Type,
	&ListTag::Type,
	&CompoundTag::Type,
	&IntArrayTag::Type,
	&LongArrayTag::Type,
};


std::pair<std::string, std::shared_ptr<const Tag>> Tag::readNamedTag(Buffer *buffer) {
	const TagType &type = getTypeById(buffer->get8());
	if (type == EndTag::Type)
		return std::make_pair("", std::make_shared<EndTag>(buffer));

	uint16_t len = buffer->get16();
	std::string name(reinterpret_cast<const char*>(buffer->get(len)), len);

	return std::make_pair(name, type.read(buffer));
}

}
}
