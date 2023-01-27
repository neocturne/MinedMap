use std::{
	fmt::Debug,
	ops::{Index, IndexMut},
};

use itertools::iproduct;

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

#[derive(Debug, Clone, Copy, Default)]
pub struct ChunkArray<T>(pub [[T; CHUNKS_PER_REGION as usize]; CHUNKS_PER_REGION as usize]);

impl<T> ChunkArray<T> {
	pub fn keys() -> impl Iterator<Item = ChunkCoords> {
		iproduct!(0..CHUNKS_PER_REGION, 0..CHUNKS_PER_REGION).map(|(z, x)| ChunkCoords {
			x: ChunkX(x),
			z: ChunkZ(z),
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
