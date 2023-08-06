use std::{
	fmt::Debug,
	iter::FusedIterator,
	ops::{Index, IndexMut},
};

use itertools::iproduct;
use serde::{Deserialize, Serialize};

pub mod axis {
	pub const X: u8 = 0;
	pub const Y: u8 = 1;
	pub const Z: u8 = 2;
}

macro_rules! coord_type {
	($t:ident, $max:expr) => {
		#[derive(Debug, Clone, Copy, PartialEq, Eq)]
		pub struct $t<const AXIS: u8>(pub u8);

		impl<const AXIS: u8> $t<AXIS> {
			const MAX: usize = $max;

			/// Constructs a new value
			///
			/// Will panic if the value is not in the valid range
			pub fn new<T: TryInto<u8>>(value: T) -> Self {
				Self(
					value
						.try_into()
						.ok()
						.filter(|&v| (v as usize) < Self::MAX)
						.expect("coordinate should be in the valid range"),
				)
			}

			/// Returns an iterator over all possible values of the type
			pub fn iter() -> impl Iterator<Item = $t<AXIS>>
			       + DoubleEndedIterator
			       + ExactSizeIterator
			       + FusedIterator
			       + Clone
			       + Debug {
				(0..Self::MAX as u8).map($t)
			}
		}
	};
}

pub const BLOCK_BITS: u8 = 4;
pub const BLOCKS_PER_CHUNK: usize = 1 << BLOCK_BITS;
coord_type!(BlockCoord, BLOCKS_PER_CHUNK);

/// A block X coordinate relative to a chunk
pub type BlockX = BlockCoord<{ axis::X }>;

/// A block Y coordinate relative to a chunk section
pub type BlockY = BlockCoord<{ axis::Y }>;

/// A block Z coordinate relative to a chunk
pub type BlockZ = BlockCoord<{ axis::Z }>;

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

impl LayerBlockCoords {
	/// Computes a block's offset in various data structures
	///
	/// Many chunk data structures store block and biome data in the same
	/// order. This method computes the offset at which the data for the
	/// block at a given coordinate is stored.
	pub fn offset(&self) -> usize {
		use BLOCKS_PER_CHUNK as N;
		let x = self.x.0 as usize;
		let z = self.z.0 as usize;
		N * z + x
	}
}

/// Generic array for data stored per block of a chunk layer
///
/// Includes various convenient iteration functions.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct LayerBlockArray<T>(pub [[T; BLOCKS_PER_CHUNK]; BLOCKS_PER_CHUNK]);

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
		let y = self.y.0 as usize;
		N * N * y + self.xz.offset()
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

pub const CHUNK_BITS: u8 = 5;
pub const CHUNKS_PER_REGION: usize = 1 << CHUNK_BITS;
coord_type!(ChunkCoord, CHUNKS_PER_REGION);

/// A chunk X coordinate relative to a region
pub type ChunkX = ChunkCoord<{ axis::X }>;

/// A chunk Z coordinate relative to a region
pub type ChunkZ = ChunkCoord<{ axis::Z }>;

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
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
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
