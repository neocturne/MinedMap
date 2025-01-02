# Resource management

## Scripts

The following scripts can be found in the `resource` directory of this Git
repository. Python 3.8 should be sufficient, older versions may or may not
work.

- `blocklist.py`: Lists all supported block IDs of an unpacked Minecraft JAR, or diffs the ID lists
  of two different versions
- `extract.py`: Takes the block type information from `blocks.json` and texture data
  from an unpacked Minecraft JAR, storing the result in `colors.json`
- `generate.py`: Generates `block_types.rs` from `colors.json`
- `biomes.py`: Generates `biomes.rs` from biome JSON files of an unpacked
  Minecraft JAR
- `sign_textures.py`: Generates all needed sign graphics from Minecraft assets

In addition to these scripts, the JSON processor *jq* is a useful tool to work
with MinedMap's resource metadata.


## How to add support for block IDs and biomes of a new Minecraft version

1. Download the Minecraft version you want to support as well as the previous
   version currently supported by MinedMap. You can use the Minecraft launcher
   to do so. On Linux the downloaded JAR archive can be found at
   `~/.minecraft/versions/`.
2. Unpack both versions to different directories. The next part assumes that
   the unpacked data is stored in `resource/data/old` and `resource/data/new`;
   using the respective Minecraft version numbers instead of `old`
   and `new` is advisable.
3. Check the added and removed block types using `blocklist.py`:

     ```sh
     ./blocklist.py diff data/old data/new
     ```

4. Append all new block types to `blocks.json`. The following command can be
   used to generate the basic JSON structure:

     ```sh
     ./blocklist.py added data/old data/new | jq -R -n -S --tab '[inputs] | map({key: ., value: {}}) | from_entries'
     ```

5. Edit `blocks.json` until the following command passes without errors:

     ```sh
     ./extract.py blocks.json data/new colors.json
     ```

   If possible, the top texture of blocks should be used where different sides
   exist. Block types that should not be visible on the map are just set to
   `null` in the JSON (or have a `null` `texture` field when other flags need
   to be set, like for sign blocks).

   The `water`, `grass` and `foliage` flags control biome-dependent texture color modifiers.

6. When `colors.json` builds successfully, use the following command to sort
   `blocks.json` by block ID:

     ```sh
     jq --tab -S < blocks.json > blocks.json.new && mv blocks.json.new blocks.json
     ```

   Then regenerate `colors.json` one last time, so it is sorted as well.

7. Update the source code with the new block colors:

     ```sh
     ./generate.py colors.json ../crates/resource/src/block_types.rs
     cargo fmt --all
     ```

8. Update the source code for new biome data:

     ```sh
     ./biomes.py data/new ../crates/resource/src/biomes.rs
     cargo fmt --all
     ```

    After regenerating, check if only new biomes were added. If entries
    got removed, biomes may have been renamed or merged, requiring updates
    to the alias list in `crates/resource/src/legacy_biomes.rs`.

After the update, the new version should be tested with old savegames (both
before and after migration by the new version) as well as newly generated
worlds. Use creative mode to add the new block types to your test world.
