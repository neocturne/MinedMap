cmake_minimum_required(VERSION 2.8.4)
project(MINEDMAP CXX)


find_package(PkgConfig REQUIRED)
pkg_check_modules(LIBPNG REQUIRED libpng>=1.2)
pkg_check_modules(ZLIB REQUIRED zlib)


add_subdirectory(src)
