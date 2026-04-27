# MCUtilz

This tool allows you to easily save and load regions of a local Minecraft world using a simple text format. It is a fork of [`dxrcy/mcutils`](https://github.com/dxrcy/mcutils) specifically adapted to make generating Minecraft regions for autograders as straightforward as possible.

The primary changes from the original include:
- **Plaintext Format**: Configurations are saved into a simple `x,y,z: block_id:modifier` text file instead of an unreadable binary format.
- **Player-Centric Operations**: All tasks are centered directly over your character—saving and loading structures without worrying about exact manual coordinates.
- **Configurable Radii**: Areas span out dynamically. By default, a command sweeps `20` blocks horizontally (E/W and N/S) and `10` blocks vertically (up/down). Since this includes the block you stand on plus the radius in the positive and negative directions, the default footprint is a **41x21x41** block area.

## Installation

```sh
# Download
git clone https://github.com/axelahmer-teaching/mcutilz
cd mcutilz
# Build and install locally
cargo install --path .
```

## Usage

Both `save` and `load` commands operate dynamically around your current standing location. Walk to where you want the region's origin to be, and run:

```sh
# Save the default 41x21x41 area (20 blocks horizontal and 10 blocks vertical in all 6 directions)
mcutilz save blocks.txt

# Save with a custom uniform radius (e.g., 15 blocks in all directions -> 31x31x31 area)
mcutilz save blocks.txt --radius 15

# Save a wide and flat rectangular prism (15 blocks E/W, 5 blocks up/down, 15 blocks N/S)
mcutilz save blocks.txt --dx 15 --dy 5 --dz 15

# Load the previously saved snapshot seamlessly relative to your new player position
mcutilz load blocks.txt

# Clear the default 41x21x41 area around you to AIR
mcutilz clear

# Clear a custom uniform dimension 
mcutilz clear --radius 15

# Clear only a vertical column directly above and below you
mcutilz clear --dx 2 --dy 20 --dz 2
```
