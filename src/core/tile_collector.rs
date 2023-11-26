//! A trait for recursively processing tiles
//!
//! Used for mipmap generation and collecting entity data

use std::sync::mpsc;

use anyhow::Result;
use rayon::prelude::*;

use super::common::*;

/// Helper to determine if no further mipmap levels are needed
///
/// If all tile coordinates are -1 or 0, further mipmap levels will not
/// decrease the number of tiles and mipmap generated is considered finished.
fn done(tiles: &TileCoordMap) -> bool {
	tiles
		.0
		.iter()
		.all(|(z, xs)| (-1..=0).contains(z) && xs.iter().all(|x| (-1..=0).contains(x)))
}

/// Derives the map of populated tile coordinates for the next mipmap level
fn map_coords(tiles: &TileCoordMap) -> TileCoordMap {
	let mut ret = TileCoordMap::default();

	for (&z, xs) in &tiles.0 {
		for &x in xs {
			let xt = x >> 1;
			let zt = z >> 1;

			ret.0.entry(zt).or_default().insert(xt);
		}
	}

	ret
}

/// Trait to implement for collecting tiles recursively
pub trait TileCollector: Sync {
	/// Return value of [TileCollector::collect_one]
	type CollectOutput: Send;

	/// List of level 0 tiles
	fn tiles(&self) -> &[TileCoords];

	/// Called at the beginning of each level of processing
	fn prepare(&self, level: usize) -> Result<()>;

	/// Called at the end of each level of processing
	fn finish(
		&self,
		level: usize,
		outputs: impl Iterator<Item = Self::CollectOutput>,
	) -> Result<()>;

	/// Called for each tile coordinate of the level that is currently being generated
	fn collect_one(
		&self,
		level: usize,
		coords: TileCoords,
		prev: &TileCoordMap,
	) -> Result<Self::CollectOutput>;

	/// Collects tiles recursively
	fn collect_tiles(&self) -> Result<Vec<TileCoordMap>> {
		let mut tile_stack = {
			let mut tile_map = TileCoordMap::default();

			for &TileCoords { x, z } in self.tiles() {
				tile_map.0.entry(z).or_default().insert(x);
			}

			vec![tile_map]
		};

		loop {
			let level = tile_stack.len();
			let prev = &tile_stack[level - 1];
			if done(prev) {
				break;
			}

			self.prepare(level)?;

			let next = map_coords(prev);

			let (send, recv) = mpsc::channel();

			next.0
				.par_iter()
				.flat_map(|(&z, xs)| xs.par_iter().map(move |&x| TileCoords { x, z }))
				.try_for_each(|coords| {
					let output = self.collect_one(level, coords, prev)?;
					send.send(output).unwrap();
					anyhow::Ok(())
				})?;

			drop(send);
			self.finish(level, recv.into_iter())?;

			tile_stack.push(next);
		}

		Ok(tile_stack)
	}
}
