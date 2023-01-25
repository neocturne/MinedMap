pub const CHUNKS_PER_REGION: u8 = 32;

/// A chunk X coordinate relative to a region
#[derive(Debug, Clone, Copy)]
pub struct ChunkX(pub u8);

/// A chunk Z coordinate relative to a region
#[derive(Debug, Clone, Copy)]
pub struct ChunkZ(pub u8);
