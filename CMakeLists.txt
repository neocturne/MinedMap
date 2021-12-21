cmake_minimum_required(VERSION 3.13)
project(MINEDMAP CXX)

# May not work with all toolchains, added for the Windows CI build
option(BUILD_STATIC "Create a statically linked MinedMap executable")

if(BUILD_STATIC)
  list(REMOVE_ITEM CMAKE_FIND_LIBRARY_SUFFIXES ".so" ".dll.a")
  add_link_options("-static")
endif()

find_package(PkgConfig REQUIRED)

pkg_check_modules(ZLIB REQUIRED IMPORTED_TARGET zlib)
pkg_check_modules(PNG REQUIRED IMPORTED_TARGET libpng16)

add_subdirectory(src)
