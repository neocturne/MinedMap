use std::{
	fmt::Debug,
	ops::{Index, IndexMut},
};

use itertools::iproduct;

pub const BLOCKS_PER_CHUNK: usize = 16;

/// A block X coordinate relative to a chunk
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockX(pub u8);

/// A block Y coordinate relative to a chunk section
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockY(pub u8);

/// A block Z coordinate relative to a chunk
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockZ(pub u8);

/// X, Y and Z coordinates of a block in a chunk section
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BlockCoords {
	pub x: BlockX,
	pub y: BlockY,
	pub z: BlockZ,
}

impl BlockCoords {
	/// Computes a block's offset in various data structures
	///
	/// Many chunk data structures store block and biome data in the same
	/// order. [BlockCoords::offset] computes the offset at which the data
	/// for the block at a given coordinate is stored.
	pub fn offset(&self) -> usize {
		use BLOCKS_PER_CHUNK as N;
		let x = self.x.0 as usize;
		let y = self.y.0 as usize;
		let z = self.z.0 as usize;
		((y * N) + z) * N + x
	}
}

impl Debug for BlockCoords {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {}, {})", self.x.0, self.y.0, self.z.0)
	}
}

/// A section Y coordinate
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SectionY(pub i32);

pub const CHUNKS_PER_REGION: usize = 32;

/// A chunk X coordinate relative to a region
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkX(pub u8);

/// A chunk Z coordinate relative to a region
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkZ(pub u8);

/// A pair of chunk coordinates relative to a region
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChunkCoords {
	pub x: ChunkX,
	pub z: ChunkZ,
}

impl Debug for ChunkCoords {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x.0, self.z.0)
	}
}

/// Generic array for data stored per chunk of a region
///
/// Includes various convenient iteration functions.
#[derive(Debug, Clone, Copy, Default)]
pub struct ChunkArray<T>(pub [[T; CHUNKS_PER_REGION]; CHUNKS_PER_REGION]);

impl<T> ChunkArray<T> {
	pub fn keys() -> impl Iterator<Item = ChunkCoords> {
		iproduct!(0..(CHUNKS_PER_REGION as u8), 0..(CHUNKS_PER_REGION as u8)).map(|(z, x)| {
			ChunkCoords {
				x: ChunkX(x),
				z: ChunkZ(z),
			}
		})
	}

	pub fn values(&self) -> impl Iterator<Item = &T> {
		Self::keys().map(|k| &self[k])
	}

	pub fn iter(&self) -> impl Iterator<Item = (ChunkCoords, &T)> {
		Self::keys().map(|k| (k, &self[k]))
	}
}

impl<T> Index<ChunkCoords> for ChunkArray<T> {
	type Output = T;

	fn index(&self, index: ChunkCoords) -> &Self::Output {
		&self.0[index.z.0 as usize][index.x.0 as usize]
	}
}

impl<T> IndexMut<ChunkCoords> for ChunkArray<T> {
	fn index_mut(&mut self, index: ChunkCoords) -> &mut Self::Output {
		&mut self.0[index.z.0 as usize][index.x.0 as usize]
	}
}
