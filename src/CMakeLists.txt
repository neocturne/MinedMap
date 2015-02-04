include_directories(${ZLIB_INCLUDE_DIRS} ${PNG_INCLUDE_DIRS})
add_definitions(${PNG_DEFINITIONS})

add_executable(MinedMap
  MinedMap.cpp
  GZip.cpp
  Info.cpp
  PNG.cpp

  NBT/Tag.cpp
  Resource/Biome.cpp
  Resource/BlockType.cpp
  World/Block.cpp
  World/Chunk.cpp
  World/Level.cpp
  World/Region.cpp
)
set_target_properties(MinedMap PROPERTIES COMPILE_FLAGS "-std=c++11 -Wall")
target_link_libraries(MinedMap ${ZLIB_LIBRARIES} ${PNG_LIBRARIES})
