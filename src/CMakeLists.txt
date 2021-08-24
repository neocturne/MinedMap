add_compile_options(-std=c++11 -Wall)

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
target_link_libraries(MinedMap PkgConfig::ZLIB PkgConfig::PNG)

add_executable(nbtdump
  nbtdump.cpp
  GZip.cpp
  NBT/Tag.cpp
)
target_link_libraries(nbtdump PkgConfig::ZLIB)

add_executable(regiondump
  regiondump.cpp
  GZip.cpp
  NBT/Tag.cpp
  World/ChunkData.cpp
  World/Region.cpp
)
target_link_libraries(regiondump PkgConfig::ZLIB)

install(TARGETS MinedMap RUNTIME DESTINATION bin)
