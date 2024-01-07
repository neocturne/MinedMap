<!-- next-header -->

## [Unreleased] - ReleaseDate

### Updated

- Added support for Minecraft 1.20.3+

  Minecraft 1.20.3 renamed the `grass` block type to `short_grass`.
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
[Unreleased]: https://github.com/neocturne/MinedMap/compare/v2.0.1...HEAD
[2.0.1]: https://github.com/neocturne/MinedMap/compare/v2.0.0...v2.0.1
[2.0.0]: https://github.com/neocturne/MinedMap/compare/v1.19.1...v2.0.0
