include_directories(${ZLIB_INCLUDE_DIRS} ${LIBPNG_INCLUDE_DIRS})
link_directories(${ZLIB_LIBRARY_DIRS} ${LIBPNG_LIBRARY_DIRS})

add_executable(MinedMap
  MinedMap.cpp
  NBT/Tag.cpp
  World/BlockType.cpp
  World/Chunk.cpp
  World/Region.cpp
)
set_target_properties(MinedMap PROPERTIES COMPILE_FLAGS "-std=c++11 -Wall ${ZLIB_CFLAGS_OTHER} ${LIBPNG_CFLAGS_OTHER}")
set_target_properties(MinedMap PROPERTIES LINK_FLAGS "${ZLIB_LDFLAGS_OTHER} ${LIBPNG_LDFLAGS_OTHER}")
target_link_libraries(MinedMap ${ZLIB_LIBRARIES} ${LIBPNG_LIBRARIES})
