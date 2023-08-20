//! The minedmap generator renders map tile images from Minecraft save data

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

mod core;
mod io;
mod util;
mod world;

use minedmap_nbt as nbt;
use minedmap_resource as resource;
use minedmap_types as types;

use anyhow::Result;

fn main() -> Result<()> {
	core::cli()
}
