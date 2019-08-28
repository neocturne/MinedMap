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
  World/Chunk.cpp
  World/ChunkData.cpp
  World/Level.cpp
  World/Region.cpp
  World/Section.cpp
)
set_target_properties(MinedMap PROPERTIES COMPILE_FLAGS "-std=c++11 -Wall")
target_link_libraries(MinedMap ${ZLIB_LIBRARIES} ${PNG_LIBRARIES})

add_executable(nbtdump
  nbtdump.cpp
  GZip.cpp
  NBT/Tag.cpp
)
set_target_properties(nbtdump PROPERTIES COMPILE_FLAGS "-std=c++11 -Wall")
target_link_libraries(nbtdump ${ZLIB_LIBRARIES})

add_executable(regiondump
  regiondump.cpp
  GZip.cpp
  NBT/Tag.cpp
  World/ChunkData.cpp
  World/Region.cpp
)
set_target_properties(regiondump PROPERTIES COMPILE_FLAGS "-std=c++11 -Wall")
target_link_libraries(regiondump ${ZLIB_LIBRARIES})

install(TARGETS MinedMap RUNTIME DESTINATION bin)
