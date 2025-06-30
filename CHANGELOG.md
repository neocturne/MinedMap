<!-- next-header -->

## [Unreleased] - ReleaseDate

## [2.6.0] - 2025-06-30

### Added

- Added support for Minecraft 1.21.5 to 1.21.7

  Added new block types and handling for changed sign text storage format.

## [2.5.0] - 2025-03-16

### Added

- Added experimental watch mode

  Passing `--watch` will cause MinedMap to run continuously instead of exiting
  after map generation, regenerating tiles whenever they change.

  `--watch-delay` can be used to configure the delay between detecting a change
  and runing the map generation, also limiting how often the regeneration
  happens. This defaults to `30s`; significantly smaller values probably don't
  make sense because Minecraft writes out changes in batches anyways.

  Finally, `--jobs-initial` can be used to configure the number of parallel
  generation threads for the initial cycle separately from the value used for
  subsequent cycles after a change is detected (`-j`/`--jobs`). Subsequent
  cycles usually need to regenerate only a small number of tiles, so setting
  `--jobs` to a smaller value than `--jobs-initial` may be advantageous.

- Added jemalloc support to fix performace on musl targets

  The global allocator can be switched to jemalloc by enabling the `jemalloc`
  cargo feature now. This is not the default because it is not always faster
  than the default system allocator; in particular, the glibc allocator has
  slightly better performance in multithreaded mode. In addition, jemalloc
  uses a bit more memory.

  In addition, the `jemalloc-auto` feature has been introduced, which is enabled
  by default and sets the global allocator to jemalloc on platforms where it is
  clearly advantageous. For now, this is only done on musl-based targets, as
  musl's default allocator is very slow in multithreaded operation (which was
  making higher thread counts like `-j8` basically useless due to 7-8x
  slowdowns). With the new default, performance on musl is basically identical
  to glibc.

  Note that some platforms like `msvc` are unsupported by jemalloc, and trying
  to enable the `jemalloc` feature on these platforms may break the MinedMap
  build or cause issues at runtime.
- Docker images can be downloaded from the GitHub Container registry

  Two images are provided, one for the tile renderer and one with the viewer
  and a web server. A `docker-compose.yml` example can be found in the
  repository as a starting point.

### Changed

- Unknown biome types (from not yet supported or modded versions of Minecraft)
  will now use plains biome colors as a fallback instead of resulting in water,
  grass and foliage blocks to be rendered as transparent pixels
- Switched from zlib-ng to zlib-rs

  This should have no noticable effect on the usage of MinedMap, but avoids
  an external build dependency on CMake.
- Small (1-block) seagrass is now visible on the map

  1-block seagrass in 1-block deep water would previously result in the ground
  to be shown instead of water, as MinedMap currently doesn't handle the
  "waterlogged" block status. As 1-block seagrass is relatively big compared to
  other "small" plants, just considering it opaque seems like a good enough
  solution that avoids having to implement advanced block status flags.
- Use Bincode 2 for storage of intermediate data

  The update from Bincode 1 to 2 slightly reduces the size of the `processed`
  directory used for intermediate data. At least Rust 1.85 is now required to
  build MinedMap.

## [2.4.0] - 2025-01-11

### Added

- Added support for rendering tiles in WebP format using the `--image-format` option

## [2.3.1] - 2025-01-06

### Fixed

- Fix text colors for signs modified using dye
- Fix text colors specified using `#rrggbb` CSS syntax in JSON text

Only named colors specified via JSON text were working as intended.

The mapping of color names to values is now handled by the generator. Both the generator and the
viewer must be updated for sign text colors to work.

## [2.3.0] - 2025-01-02

### Added

- Added support for Minecraft 1.21.4 block types
- Added support for Minecraft 1.21.4 Pale Garden biome
- viewer: added images for pale oak signs

## [2.2.0] - 2024-06-23

### Added

- Added support for Minecraft 1.21 block types

## [2.1.1] - 2024-06-14

### Fixed

- Fix crash due to incorrect counting in info message

  The calculation of the number of skipped regions could underflow when more invalid than valid
  regions were encountered.
- Ignore empty region files instead of treating them as invalid

  Minecraft generates empty region files in some cases. Just ignore them instead of printing an
  error message every time.

## [2.1.0] - 2024-01-27

### Added

- Added sign layer

  This feature is disabled by default. Use the `--sign-prefix` and `--sign-filter` options to
  configure which signs to show on the map. `--sign-transform` allows to modify the displayed
  sign text.

### Changed

- Without `--verbose`, only a single warning is printed at the end of
  processing for unknown block/biome types, rather than once for every
  section where such a block/biome is encountered.

## [2.0.2] - 2024-01-07

### Added

- Added support for Minecraft 1.20.3+

  Minecraft 1.20.3 renamed the `grass` block type to `short_grass`.

### Changed

- Updated [Leaflet](https://leafletjs.com/) to 1.9.4
- Updated attribution URL to https://github.com/neocturne/MinedMap

## [2.0.1] - 2023-11-18

### Fixed

- Proceed with missing tiles rather can failing completely when an invalid
  region file is encountered and no processed data from a previous run exists

## [2.0.0] - 2023-09-30

This is a complete rewrite of the map renderer in Rust, as the previous C++
implementation was getting more and more difficult to maintain and keep current
versions of Minecraft supported.

The new implementation is generally faster than the old one (by using better
data structures), but it also uses a bit more RAM and storage space for
intermediate data.

### Added

- Added support for Minecraft 1.20 biomes and block types
- Multithreading: Pass `-j N` to minedmap to use *N* CPU cores in parallel. Note
  that this also multiplies the RAM requirements of MinedMap.
- Extended OS support: MinedMap should now run on every system supported by Rust
  as a target. As I don't have a way to test these builds, binary releases are
  still limited to Windows and Linux for now; on other targets, MinedMap must
  be built from source.

### Changed

- Biome smoothing uses a different filter kernel now, which might result in
  nicer gradients?
- Log messages have been reduced. Pass `-v` to get a message for each
  processed file again.
- The intermediate data directory `biome` in the output directory has been
  replaced with a new `processed` directory. The `biome` directory can be
  deleted when reusing the output directory of an older MinedMap version.

### Fixed

- Warnings about unknown biomes or block types have been reduced to once per
  chunk/section, so rending is not slowed down by these message so much anymore.

  Full support for custom biomes datapacks might be added in a future release.

<!-- next-url -->
[Unreleased]: https://github.com/neocturne/MinedMap/compare/v2.6.0...HEAD
[2.6.0]: https://github.com/neocturne/MinedMap/compare/v2.5.0...v2.6.0
[2.5.0]: https://github.com/neocturne/MinedMap/compare/v2.4.0...v2.5.0
[2.4.0]: https://github.com/neocturne/MinedMap/compare/v2.3.1...v2.4.0
[2.3.1]: https://github.com/neocturne/MinedMap/compare/v2.3.0...v2.3.1
[2.3.0]: https://github.com/neocturne/MinedMap/compare/v2.2.0...v2.3.0
[2.2.0]: https://github.com/neocturne/MinedMap/compare/v2.1.1...v2.2.0
[2.1.1]: https://github.com/neocturne/MinedMap/compare/v2.1.0...v2.1.1
[2.1.0]: https://github.com/neocturne/MinedMap/compare/v2.0.2...v2.1.0
[2.0.2]: https://github.com/neocturne/MinedMap/compare/v2.0.1...v2.0.2
[2.0.1]: https://github.com/neocturne/MinedMap/compare/v2.0.0...v2.0.1
[2.0.0]: https://github.com/neocturne/MinedMap/compare/v1.19.1...v2.0.0
