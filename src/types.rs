use std::{
	fmt::Debug,
	iter::FusedIterator,
	ops::{Index, IndexMut},
};

use itertools::iproduct;

macro_rules! coord_impl {
	($t:ident, $max:expr) => {
		impl $t {
			/// Returns an iterator over all possible values of the type
			pub fn iter() -> impl Iterator<Item = $t>
			       + DoubleEndedIterator
			       + ExactSizeIterator
			       + FusedIterator
			       + Clone
			       + Debug {
				(0..$max as u8).map($t)
			}
		}
	};
}

pub const BLOCKS_PER_CHUNK: usize = 16;

/// A block X coordinate relative to a chunk
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockX(pub u8);
coord_impl!(BlockX, BLOCKS_PER_CHUNK);

/// A block Y coordinate relative to a chunk section
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockY(pub u8);
coord_impl!(BlockY, BLOCKS_PER_CHUNK);

/// A block Z coordinate relative to a chunk
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockZ(pub u8);
coord_impl!(BlockZ, BLOCKS_PER_CHUNK);

/// X and Z coordinates of a block in a chunk
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LayerBlockCoords {
	pub x: BlockX,
	pub z: BlockZ,
}

impl Debug for LayerBlockCoords {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x.0, self.z.0)
	}
}

/// Generic array for data stored per block of a chunk layer
///
/// Includes various convenient iteration functions.
#[derive(Debug, Clone, Copy, Default)]
pub struct LayerBlockArray<T>(pub [[T; BLOCKS_PER_CHUNK]; BLOCKS_PER_CHUNK]);

impl<T> LayerBlockArray<T> {
	pub fn keys() -> impl Iterator<Item = LayerBlockCoords> + Clone + Debug {
		iproduct!(BlockZ::iter(), BlockX::iter()).map(|(z, x)| LayerBlockCoords { x, z })
	}

	pub fn values(&self) -> impl Iterator<Item = &T> + Clone + Debug {
		Self::keys().map(|k| &self[k])
	}

	pub fn iter(&self) -> impl Iterator<Item = (LayerBlockCoords, &T)> + Clone + Debug {
		Self::keys().map(|k| (k, &self[k]))
	}
}

impl<T> Index<LayerBlockCoords> for LayerBlockArray<T> {
	type Output = T;

	fn index(&self, index: LayerBlockCoords) -> &Self::Output {
		&self.0[index.z.0 as usize][index.x.0 as usize]
	}
}

impl<T> IndexMut<LayerBlockCoords> for LayerBlockArray<T> {
	fn index_mut(&mut self, index: LayerBlockCoords) -> &mut Self::Output {
		&mut self.0[index.z.0 as usize][index.x.0 as usize]
	}
}

/// X, Y and Z coordinates of a block in a chunk section
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SectionBlockCoords {
	pub xz: LayerBlockCoords,
	pub y: BlockY,
}

impl SectionBlockCoords {
	/// Computes a block's offset in various data structures
	///
	/// Many chunk data structures store block and biome data in the same
	/// order. This method computes the offset at which the data for the
	/// block at a given coordinate is stored.
	pub fn offset(&self) -> usize {
		use BLOCKS_PER_CHUNK as N;
		let x = self.xz.x.0 as usize;
		let y = self.y.0 as usize;
		let z = self.xz.z.0 as usize;
		((y * N) + z) * N + x
	}
}

impl Debug for SectionBlockCoords {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {}, {})", self.xz.x.0, self.y.0, self.xz.z.0)
	}
}

/// A section Y coordinate
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SectionY(pub i32);

pub const CHUNKS_PER_REGION: usize = 32;

/// A chunk X coordinate relative to a region
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkX(pub u8);
coord_impl!(ChunkX, CHUNKS_PER_REGION);

/// A chunk Z coordinate relative to a region
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkZ(pub u8);
coord_impl!(ChunkZ, CHUNKS_PER_REGION);

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
	pub fn keys() -> impl Iterator<Item = ChunkCoords> + Clone + Debug {
		iproduct!(ChunkZ::iter(), ChunkX::iter()).map(|(z, x)| ChunkCoords { x, z })
	}

	pub fn values(&self) -> impl Iterator<Item = &T> + Clone + Debug {
		Self::keys().map(|k| &self[k])
	}

	pub fn iter(&self) -> impl Iterator<Item = (ChunkCoords, &T)> + Clone + Debug {
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
