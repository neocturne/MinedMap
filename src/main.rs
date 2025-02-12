#![doc = env!("CARGO_PKG_DESCRIPTION")]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

#[cfg(feature = "jemalloc-auto")]
extern crate minedmap_default_alloc;

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
