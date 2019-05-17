cmake_minimum_required(VERSION 2.8.4)
project(MINEDMAP CXX)


find_package(PNG REQUIRED)
find_package(ZLIB REQUIRED)
find_package(Threads REQUIRED)

add_subdirectory(src)
