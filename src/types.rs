use std::fmt::Debug;

pub const CHUNKS_PER_REGION: u8 = 32;

/// A chunk X coordinate relative to a region
#[derive(Debug, Clone, Copy)]
pub struct ChunkX(pub u8);

/// A chunk Z coordinate relative to a region
#[derive(Debug, Clone, Copy)]
pub struct ChunkZ(pub u8);

/// A pair of chunk coordinates relative to a region
#[derive(Clone, Copy)]
pub struct ChunkCoords {
	pub x: ChunkX,
	pub z: ChunkZ,
}

impl Debug for ChunkCoords {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x.0, self.z.0)
	}
}
