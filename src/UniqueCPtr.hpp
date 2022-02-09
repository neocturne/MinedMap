// SPDX-License-Identifier: BSD-2-Clause
/*
  Copyright (c) 2015, Matthias Schiffer <mschiffer@universe-factory.net>
  All rights reserved.
*/


#pragma once

#include <cstdlib>
#include <memory>

// UniqueCPtr is a wrapper around std::unique_ptr using std::free as its deallocator
//
// This allows to use it to manage memory allocated by C APIs (malloc, realloc, ...)
template<typename T> class UniqueCPtr : public std::unique_ptr<T, void (*)(void *)> {
public:
	UniqueCPtr() : std::unique_ptr<T, void (*)(void *)>(nullptr, std::free) {}
	template<typename T2> UniqueCPtr(T2 ptr) : std::unique_ptr<T, void (*)(void *)>(ptr, std::free) {}
};
