cmake_minimum_required(VERSION 3.7)
project(MINEDMAP CXX)

find_package(PkgConfig REQUIRED)

pkg_check_modules(ZLIB REQUIRED IMPORTED_TARGET zlib)
pkg_check_modules(PNG REQUIRED IMPORTED_TARGET libpng16)

add_subdirectory(src)
