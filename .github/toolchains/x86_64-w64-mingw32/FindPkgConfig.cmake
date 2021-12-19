# Hack: The toolchain file is evaluated too early to set the library suffixes, so we use a wrapper
# around FindPkgConfig set it right before we search for libraries.
set(CMAKE_FIND_LIBRARY_SUFFIXES ".a")

include(${CMAKE_ROOT}/Modules/FindPkgConfig.cmake)
