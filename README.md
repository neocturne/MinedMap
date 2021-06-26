# MinedMap

* Render beautiful maps of your [Minecraft](https://minecraft.net/) worlds!
* Put them on a webserver and view them in your browser!
* Compatible with unmodified Minecraft Java Edition 1.8 up to 1.17 (no mod installation necessary!)
* Illumination layer: the world at night
* Fast: create a full map for a huge 3GB savegame in less than 5 minutes
* Incremental updates: only recreate map tiles for regions that have changed
* Very low memory usage: typically uses less than 5MB of RAM

![Screenshot](docs/images/MinedMap.png)


## How to use

MinedMap consists of two components: a map renderer generating map tiles from
Minecraft save games, and a viewer for displaying and navigating maps in a browser
based on [Leaflet](https://leafletjs.com/). The map renderer is heavily inspired by
[MapRend](https://github.com/YSelfTool/MapRend), but it has been implemented in C++
from scratch for highest performance.

The viewer expects the the map data in a directory named `data`. To generate a new
map, create this empty directory inside the viewer directory. Next, to generate the
map files run MinedMap passing the source and the destination paths on the command
line:
```shell
./MinedMap /path/to/save/game /path/to/viewer/data
```
The save game is stored in `saves` inside your Minecraft main directory
(`~/.minecraft` on Linux, `C:\Users\<username>\AppData\Roaming\.minecraft` on Windows)
in a subdirectory with the name of your world.

The first map generation might take a while for big worlds, but subsequent calls will
only rebuild tiles for region files that have changed, rarely taking more than a second
or two. This makes it feasible to update the map very frequently, e.g. by running
MinedMap as a Cron job every minute.

Note that it is not possible to open the viewer *index.html* without a webserver, as
it cannot load the generated map information from `file://` URIs. For testing purposes,
you can use a minimal HTTP server, e.g. (if you have Python installed):
```shell
python3 -m http.server
```
This test server is very slow and cannot handle multiple requests concurrently, so use
a proper webserver like [nginx](https://nginx.org/) or upload the viewer together with
the generated map files to public webspace to make the map available to others.


## Building MinedMap

Precompiled MinedMap binaries for Windows (32bit and 64bit versions) are available under
"Releases" on the Github page. On other platforms, MinedMap must be built from source.

MinedMap has been tested to work on Windows and Linux. I assume it can also be
built for MacOS and pretty much any POSIX-like system, but I didn't check. ¯\\\_(ツ)\_/¯

To build from source, you need Git, CMake, the GCC toolchain and the development
files for the libraries *zlib* and *libpng* (packages *git*, *cmake*, *build-essential*,
*zlib1g-dev* and *libpng-dev* on Debian/Ubuntu).

Use the following commands to build:
```shell
git clone https://github.com/NeoRaider/MinedMap.git # Or download a release ZIP and unpack it
mkdir MinedMap-build
cd MinedMap-build
cmake ../MinedMap -DCMAKE_BUILD_TYPE=RELEASE
make
```
After a successful build, the MinedMap renderer binary can be found in the *src*
subdirectory of the build dir `MinedMap-build`. The viewer is located in the cloned
Git repository `MinedMap`.
