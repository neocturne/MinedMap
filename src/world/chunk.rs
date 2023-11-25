//! Higher-level interfaces to chunk data
//!
//! The data types in this module attempt to provide interfaces abstracting
//! over different data versions as much as possible.

use std::{
	collections::{btree_map, BTreeMap},
	iter::{self, FusedIterator},
};

use anyhow::{bail, Context, Result};

use super::{de, section::*};
use crate::{
	resource::{BiomeTypes, BlockTypes},
	types::*,
};

/// Chunk data structure wrapping a [de::Chunk] for convenient access to
/// block and biome data
#[derive(Debug)]
pub enum Chunk<'a> {
	/// Minecraft v1.18+ chunk with biome data moved into sections
	V1_18 {
		/// Section data
		section_map: BTreeMap<SectionY, (SectionV1_13<'a>, BiomesV1_18<'a>, BlockLight<'a>)>,
	},
	/// Minecraft v1.13+ chunk
	///
	/// Block data is stored in an indexed format with variable bit width
	/// (depending on the total numer of distinct block types used in a
	/// section), and a palette mapping these indices to namespaced
	/// block IDs
	V1_13 {
		/// Section data
		section_map: BTreeMap<SectionY, (SectionV1_13<'a>, BlockLight<'a>)>,
		/// Biome data
		biomes: BiomesV0<'a>,
	},
	/// Original pre-1.13 chunk
	///
	/// The original chunk format with fixed 8-bit numeric block IDs
	V0 {
		/// Section data
		section_map: BTreeMap<SectionY, (SectionV0<'a>, BlockLight<'a>)>,
		/// Biome data
		biomes: BiomesV0<'a>,
	},
	/// Unpopulated chunk without any block data
	Empty,
}

/// Inner data structure of [SectionIter]
#[derive(Debug, Clone)]
enum SectionIterInner<'a> {
	/// Iterator over sections of [Chunk::V1_18]
	V1_18 {
		/// Inner iterator into section map
		iter: btree_map::Iter<'a, SectionY, (SectionV1_13<'a>, BiomesV1_18<'a>, BlockLight<'a>)>,
	},
	/// Iterator over sections of [Chunk::V1_13]
	V1_13 {
		/// Inner iterator into section map
		iter: btree_map::Iter<'a, SectionY, (SectionV1_13<'a>, BlockLight<'a>)>,
		/// Chunk biome data
		biomes: &'a BiomesV0<'a>,
	},
	/// Iterator over sections of [Chunk::V0]
	V0 {
		/// Inner iterator into section map
		iter: btree_map::Iter<'a, SectionY, (SectionV0<'a>, BlockLight<'a>)>,
		/// Chunk biome data
		biomes: &'a BiomesV0<'a>,
	},
	/// Empty iterator over an unpopulated chunk ([Chunk::Empty])
	Empty,
}

/// Iterator over the sections of a [Chunk]
#[derive(Debug, Clone)]
pub struct SectionIter<'a> {
	/// Inner iterator enum
	inner: SectionIterInner<'a>,
}

impl<'a> Chunk<'a> {
	/// Creates a new [Chunk] from a deserialized [de::Chunk]
	pub fn new(
		data: &'a de::Chunk,
		block_types: &'a BlockTypes,
		biome_types: &'a BiomeTypes,
	) -> Result<Self> {
		let data_version = data.data_version.unwrap_or_default();

		match &data.chunk {
			de::ChunkVariant::V1_18 { sections } => {
				Self::new_v1_18(data_version, sections, block_types, biome_types)
			}
			de::ChunkVariant::V0 { level } => {
				Self::new_v0(data_version, level, block_types, biome_types)
			}
		}
	}

	/// [Chunk::new] implementation for Minecraft v1.18+ chunks
	fn new_v1_18(
		data_version: u32,
		sections: &'a Vec<de::SectionV1_18>,
		block_types: &'a BlockTypes,
		biome_types: &'a BiomeTypes,
	) -> Result<Self> {
		let mut section_map = BTreeMap::new();

		for section in sections {
			match &section.section {
				de::SectionV1_18Variant::V1_18 {
					block_states,
					biomes,
					block_light,
				} => {
					section_map.insert(
						SectionY(section.y),
						(
							SectionV1_13::new(
								data_version,
								block_states.data.as_deref(),
								&block_states.palette,
								block_types,
							)
							.with_context(|| {
								format!("Failed to load section at Y={}", section.y)
							})?,
							BiomesV1_18::new(biomes.data.as_deref(), &biomes.palette, biome_types)
								.with_context(|| {
									format!("Failed to load section biomes at Y={}", section.y)
								})?,
							BlockLight::new(block_light.as_deref()).with_context(|| {
								format!("Failed to load section block light at Y={}", section.y)
							})?,
						),
					);
				}
				de::SectionV1_18Variant::Empty {} => {}
			};
		}

		Ok(Chunk::V1_18 { section_map })
	}

	/// [Chunk::new] implementation for all pre-1.18 chunk variants
	fn new_v0(
		data_version: u32,
		level: &'a de::LevelV0,
		block_types: &'a BlockTypes,
		biome_types: &'a BiomeTypes,
	) -> Result<Self> {
		let mut section_map_v1_13 = BTreeMap::new();
		let mut section_map_v0 = BTreeMap::new();

		for section in &level.sections {
			let block_light =
				BlockLight::new(section.block_light.as_deref()).with_context(|| {
					format!("Failed to load section block light at Y={}", section.y)
				})?;
			match &section.section {
				de::SectionV0Variant::V1_13 {
					block_states,
					palette,
				} => {
					section_map_v1_13.insert(
						SectionY(section.y.into()),
						(
							SectionV1_13::new(
								data_version,
								Some(block_states),
								palette,
								block_types,
							)
							.with_context(|| {
								format!("Failed to load section at Y={}", section.y)
							})?,
							block_light,
						),
					);
				}
				de::SectionV0Variant::V0 { blocks, data } => {
					section_map_v0.insert(
						SectionY(section.y.into()),
						(
							SectionV0::new(blocks, data, block_types).with_context(|| {
								format!("Failed to load section at Y={}", section.y)
							})?,
							block_light,
						),
					);
				}
				de::SectionV0Variant::Empty {} => {}
			}
		}

		let biomes = BiomesV0::new(level.biomes.as_ref(), biome_types);

		Ok(
			match (section_map_v1_13.is_empty(), section_map_v0.is_empty()) {
				(true, true) => Chunk::Empty,
				(false, true) => Chunk::V1_13 {
					section_map: section_map_v1_13,
					biomes: biomes?,
				},
				(true, false) => Chunk::V0 {
					section_map: section_map_v0,
					biomes: biomes?,
				},
				(false, false) => {
					bail!("Mixed section versions");
				}
			},
		)
	}

	/// Returns true if the chunk does not contain any sections
	pub fn is_empty(&self) -> bool {
		match self {
			Chunk::V1_18 { section_map } => section_map.is_empty(),
			Chunk::V1_13 { section_map, .. } => section_map.is_empty(),
			Chunk::V0 { section_map, .. } => section_map.is_empty(),
			Chunk::Empty => true,
		}
	}

	/// Returns an interator over the chunk's sections and their Y coordinates
	pub fn sections(&self) -> SectionIter {
		use SectionIterInner::*;
		SectionIter {
			inner: match self {
				Chunk::V1_18 { section_map } => V1_18 {
					iter: section_map.iter(),
				},
				Chunk::V1_13 {
					section_map,
					biomes,
				} => V1_13 {
					iter: section_map.iter(),
					biomes,
				},
				Chunk::V0 {
					section_map,
					biomes,
				} => V0 {
					iter: section_map.iter(),
					biomes,
				},
				Chunk::Empty => Empty,
			},
		}
	}
}

/// Reference to block, biome and block light data of a section
#[derive(Debug, Clone, Copy)]
pub struct SectionIterItem<'a> {
	/// The Y coordinate of the section
	pub y: SectionY,
	/// Section block data
	pub section: &'a dyn Section,
	/// Section biome data
	pub biomes: &'a dyn Biomes,
	/// Section block light data
	pub block_light: BlockLight<'a>,
}

/// Helper trait to specify section iterator trait bounds
trait SectionIterTrait<'a>:
	Iterator<Item = SectionIterItem<'a>> + DoubleEndedIterator + ExactSizeIterator + FusedIterator
{
}

impl<'a, T> SectionIterTrait<'a> for T where
	T: Iterator<Item = SectionIterItem<'a>>
		+ DoubleEndedIterator
		+ ExactSizeIterator
		+ FusedIterator
{
}

impl<'a> SectionIter<'a> {
	/// Helper to run a closure on the inner section iterator
	fn with_iter<F, T>(&mut self, f: F) -> T
	where
		F: FnOnce(&mut dyn SectionIterTrait<'a>) -> T,
	{
		match &mut self.inner {
			SectionIterInner::V1_18 { iter } => f(&mut iter.map(
				|(&y, (section, biomes, block_light))| SectionIterItem {
					y,
					section,
					biomes,
					block_light: *block_light,
				},
			)),
			SectionIterInner::V1_13 { iter, biomes } => f(&mut iter.map(
				|(&y, (section, block_light))| SectionIterItem {
					y,
					section,
					biomes: *biomes,
					block_light: *block_light,
				},
			)),
			SectionIterInner::V0 { iter, biomes } => f(&mut iter.map(
				|(&y, (section, block_light))| SectionIterItem {
					y,
					section,
					biomes: *biomes,
					block_light: *block_light,
				},
			)),
			SectionIterInner::Empty => f(&mut iter::empty()),
		}
	}
}

impl<'a> Iterator for SectionIter<'a> {
	type Item = SectionIterItem<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.with_iter(|iter| iter.next())
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		match &self.inner {
			SectionIterInner::V1_18 { iter } => iter.size_hint(),
			SectionIterInner::V1_13 { iter, .. } => iter.size_hint(),
			SectionIterInner::V0 { iter, .. } => iter.size_hint(),
			SectionIterInner::Empty => (0, Some(0)),
		}
	}

	fn last(mut self) -> Option<Self::Item> {
		self.with_iter(|iter| iter.last())
	}
}

impl<'a> DoubleEndedIterator for SectionIter<'a> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.with_iter(|iter| iter.next_back())
	}
}

impl<'a> ExactSizeIterator for SectionIter<'a> {
	fn len(&self) -> usize {
		match &self.inner {
			SectionIterInner::V1_18 { iter } => iter.len(),
			SectionIterInner::V1_13 { iter, .. } => iter.len(),
			SectionIterInner::V0 { iter, .. } => iter.len(),
			SectionIterInner::Empty => 0,
		}
	}
}

impl<'a> FusedIterator for SectionIter<'a> {}
