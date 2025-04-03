//! Higher-level interfaces to chunk data
//!
//! The data types in this module attempt to provide interfaces abstracting
//! over different data versions as much as possible.

use std::{
	collections::{BTreeMap, btree_map},
	iter::{self, FusedIterator},
};

use anyhow::{Context, Result, bail};

use super::{block_entity::BlockEntity, de, section::*};
use crate::{
	resource::{BiomeTypes, BlockType, BlockTypes},
	types::*,
	util::{self, ShiftMask},
};

/// Version-specific part of [Chunk]
#[derive(Debug)]
pub enum ChunkInner<'a> {
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

/// Chunk data structure wrapping a [de::Chunk] for convenient access to
/// block and biome data
#[derive(Debug)]
pub struct Chunk<'a> {
	/// Version-specific data
	inner: ChunkInner<'a>,
	/// Unprocessed block entities
	block_entities: &'a Vec<de::BlockEntity>,
	/// Chunk data version
	data_version: u32,
}

impl<'a> Chunk<'a> {
	/// Creates a new [Chunk] from a deserialized [de::Chunk]
	pub fn new(
		data: &'a de::Chunk,
		block_types: &'a BlockTypes,
		biome_types: &'a BiomeTypes,
	) -> Result<(Self, bool)> {
		let data_version = data.data_version.unwrap_or_default();

		let ((inner, has_unknown), block_entities) = match &data.chunk {
			de::ChunkVariant::V1_18 {
				sections,
				block_entities,
			} => (
				Self::new_v1_18(data_version, sections, block_types, biome_types)?,
				block_entities,
			),
			de::ChunkVariant::V0 { level } => (
				Self::new_v0(data_version, level, block_types, biome_types)?,
				&level.tile_entities,
			),
		};

		Ok((
			Chunk {
				inner,
				block_entities,
				data_version,
			},
			has_unknown,
		))
	}

	/// [Chunk::new] implementation for Minecraft v1.18+ chunks
	fn new_v1_18(
		data_version: u32,
		sections: &'a Vec<de::SectionV1_18>,
		block_types: &'a BlockTypes,
		biome_types: &'a BiomeTypes,
	) -> Result<(ChunkInner<'a>, bool)> {
		let mut section_map = BTreeMap::new();
		let mut has_unknown = false;

		for section in sections {
			match &section.section {
				de::SectionV1_18Variant::V1_18 {
					block_states,
					biomes,
					block_light,
				} => {
					let (loaded_section, unknown_blocks) = SectionV1_13::new(
						data_version,
						block_states.data.as_deref(),
						&block_states.palette,
						block_types,
					)
					.with_context(|| format!("Failed to load section at Y={}", section.y))?;
					has_unknown |= unknown_blocks;

					let (loaded_biomes, unknown_biomes) =
						BiomesV1_18::new(biomes.data.as_deref(), &biomes.palette, biome_types)
							.with_context(|| {
								format!("Failed to load section biomes at Y={}", section.y)
							})?;
					has_unknown |= unknown_biomes;

					section_map.insert(
						SectionY(section.y),
						(
							loaded_section,
							loaded_biomes,
							BlockLight::new(block_light.as_deref()).with_context(|| {
								format!("Failed to load section block light at Y={}", section.y)
							})?,
						),
					);
				}
				de::SectionV1_18Variant::Empty {} => {}
			};
		}

		let chunk = ChunkInner::V1_18 { section_map };
		Ok((chunk, has_unknown))
	}

	/// [Chunk::new] implementation for all pre-1.18 chunk variants
	fn new_v0(
		data_version: u32,
		level: &'a de::LevelV0,
		block_types: &'a BlockTypes,
		biome_types: &'a BiomeTypes,
	) -> Result<(ChunkInner<'a>, bool)> {
		let mut section_map_v1_13 = BTreeMap::new();
		let mut section_map_v0 = BTreeMap::new();
		let mut has_unknown = false;

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
					let (loaded_section, unknown_blocks) =
						SectionV1_13::new(data_version, Some(block_states), palette, block_types)
							.with_context(|| format!("Failed to load section at Y={}", section.y))?;
					has_unknown |= unknown_blocks;

					section_map_v1_13
						.insert(SectionY(section.y.into()), (loaded_section, block_light));
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
		let chunk = match (section_map_v1_13.is_empty(), section_map_v0.is_empty()) {
			(true, true) => ChunkInner::Empty,
			(false, true) => ChunkInner::V1_13 {
				section_map: section_map_v1_13,
				biomes: biomes?,
			},
			(true, false) => ChunkInner::V0 {
				section_map: section_map_v0,
				biomes: biomes?,
			},
			(false, false) => {
				bail!("Mixed section versions");
			}
		};

		Ok((chunk, has_unknown))
	}

	/// Returns true if the chunk does not contain any sections
	pub fn is_empty(&self) -> bool {
		match &self.inner {
			ChunkInner::V1_18 { section_map } => section_map.is_empty(),
			ChunkInner::V1_13 { section_map, .. } => section_map.is_empty(),
			ChunkInner::V0 { section_map, .. } => section_map.is_empty(),
			ChunkInner::Empty => true,
		}
	}

	/// Returns an interator over the chunk's sections and their Y coordinates
	pub fn sections(&self) -> SectionIter {
		use SectionIterInner::*;
		SectionIter {
			inner: match &self.inner {
				ChunkInner::V1_18 { section_map } => V1_18 {
					iter: section_map.iter(),
				},
				ChunkInner::V1_13 {
					section_map,
					biomes,
				} => V1_13 {
					iter: section_map.iter(),
					biomes,
				},
				ChunkInner::V0 {
					section_map,
					biomes,
				} => V0 {
					iter: section_map.iter(),
					biomes,
				},
				ChunkInner::Empty => Empty,
			},
		}
	}

	/// Returns the section at a [SectionY] coordinate
	fn section_at(&self, y: SectionY) -> Option<&dyn Section> {
		match &self.inner {
			ChunkInner::V1_18 { section_map } => section_map
				.get(&y)
				.map(|(section, _, _)| -> &dyn Section { section }),
			ChunkInner::V1_13 { section_map, .. } => section_map
				.get(&y)
				.map(|(section, _)| -> &dyn Section { section }),
			ChunkInner::V0 { section_map, .. } => section_map
				.get(&y)
				.map(|(section, _)| -> &dyn Section { section }),
			ChunkInner::Empty => None,
		}
	}

	/// Returns the [BlockType] at a given coordinate
	fn block_type_at(&self, y: SectionY, coords: SectionBlockCoords) -> Result<Option<&BlockType>> {
		let Some(section) = self.section_at(y) else {
			return Ok(None);
		};
		section.block_at(coords)
	}

	/// Returns the [BlockType] at the coordinates of a [de::BlockEntity]
	fn block_type_at_block_entity(
		&self,
		block_entity: &de::BlockEntity,
	) -> Result<Option<&BlockType>> {
		let x: BlockX = util::from_flat_coord(block_entity.x).2;
		let z: BlockZ = util::from_flat_coord(block_entity.z).2;
		let (section_y, block_y) = block_entity.y.shift_mask(BLOCK_BITS);

		let coords = SectionBlockCoords {
			xz: LayerBlockCoords { x, z },
			y: BlockY::new(block_y),
		};

		self.block_type_at(SectionY(section_y), coords)
	}

	/// Processes all of the chunk's block entities
	pub fn block_entities(&self) -> Result<Vec<BlockEntity>> {
		let entities: Vec<Option<BlockEntity>> = self
			.block_entities
			.iter()
			.map(|block_entity| {
				let block_type = self.block_type_at_block_entity(block_entity)?;
				Ok(BlockEntity::new(
					block_entity,
					block_type,
					self.data_version,
				))
			})
			.collect::<Result<_>>()?;
		Ok(entities.into_iter().flatten().collect())
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

/// Inner data structure of [SectionIter]
#[derive(Debug, Clone)]
enum SectionIterInner<'a> {
	/// Iterator over sections of [ChunkInner::V1_18]
	V1_18 {
		/// Inner iterator into section map
		iter: btree_map::Iter<'a, SectionY, (SectionV1_13<'a>, BiomesV1_18<'a>, BlockLight<'a>)>,
	},
	/// Iterator over sections of [ChunkInner::V1_13]
	V1_13 {
		/// Inner iterator into section map
		iter: btree_map::Iter<'a, SectionY, (SectionV1_13<'a>, BlockLight<'a>)>,
		/// Chunk biome data
		biomes: &'a BiomesV0<'a>,
	},
	/// Iterator over sections of [ChunkInner::V0]
	V0 {
		/// Inner iterator into section map
		iter: btree_map::Iter<'a, SectionY, (SectionV0<'a>, BlockLight<'a>)>,
		/// Chunk biome data
		biomes: &'a BiomesV0<'a>,
	},
	/// Empty iterator over an unpopulated chunk ([ChunkInner::Empty])
	Empty,
}

/// Iterator over the sections of a [Chunk]
#[derive(Debug, Clone)]
pub struct SectionIter<'a> {
	/// Inner iterator enum
	inner: SectionIterInner<'a>,
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
		self.next_back()
	}
}

impl DoubleEndedIterator for SectionIter<'_> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.with_iter(|iter| iter.next_back())
	}
}

impl ExactSizeIterator for SectionIter<'_> {
	fn len(&self) -> usize {
		match &self.inner {
			SectionIterInner::V1_18 { iter } => iter.len(),
			SectionIterInner::V1_13 { iter, .. } => iter.len(),
			SectionIterInner::V0 { iter, .. } => iter.len(),
			SectionIterInner::Empty => 0,
		}
	}
}

impl FusedIterator for SectionIter<'_> {}
