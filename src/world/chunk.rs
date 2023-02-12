use std::{
	collections::{btree_map, BTreeMap},
	iter::{self, FusedIterator},
};

use anyhow::{bail, Context, Result};

use super::{
	de,
	section::{OldSection, PaletteSection, PaletteSectionBiomes, Section},
};
use crate::types::*;

pub enum Chunk<'a> {
	V1_18 {
		section_map: BTreeMap<SectionY, (PaletteSection<'a>, PaletteSectionBiomes<'a>)>,
	},
	V1_13 {
		section_map: BTreeMap<SectionY, PaletteSection<'a>>,
		biomes: &'a de::BiomesOld,
	},
	Old {
		section_map: BTreeMap<SectionY, OldSection<'a>>,
		biomes: &'a de::BiomesOld,
	},
	Empty,
}

#[derive(Debug, Clone)]
enum SectionIterInner<'a> {
	V1_18 {
		iter: btree_map::Iter<'a, SectionY, (PaletteSection<'a>, PaletteSectionBiomes<'a>)>,
	},
	V1_13 {
		iter: btree_map::Iter<'a, SectionY, PaletteSection<'a>>,
	},
	Old {
		iter: btree_map::Iter<'a, SectionY, OldSection<'a>>,
	},
	Empty,
}

#[derive(Debug, Clone)]
pub struct SectionIter<'a> {
	inner: SectionIterInner<'a>,
}

impl<'a> Chunk<'a> {
	pub fn new(data: &'a de::Chunk) -> Result<Self> {
		let data_version = data.data_version.unwrap_or_default();

		match &data.chunk {
			de::ChunkVariants::V1_18 { sections } => Self::new_v1_18(data_version, sections),
			de::ChunkVariants::Old { level } => Self::new_old(data_version, level),
		}
	}

	fn new_v1_18(data_version: u32, sections: &'a Vec<de::SectionV1_18>) -> Result<Self> {
		let mut section_map = BTreeMap::new();

		for section in sections {
			section_map.insert(
				SectionY(section.y),
				(
					PaletteSection::new(
						data_version,
						section.block_states.data.as_ref(),
						&section.block_states.palette,
					)
					.with_context(|| format!("Failed to load section at Y={}", section.y))?,
					PaletteSectionBiomes::new(
						section.biomes.data.as_ref(),
						&section.biomes.palette,
					)
					.with_context(|| format!("Failed to load section biomes at Y={}", section.y))?,
				),
			);
		}

		Ok(Chunk::V1_18 { section_map })
	}

	fn new_old(data_version: u32, level: &'a de::LevelOld) -> Result<Self> {
		let mut section_map_v1_13 = BTreeMap::new();
		let mut section_map_old = BTreeMap::new();

		for section in &level.sections {
			match &section.section {
				de::SectionOldVariants::V1_13 {
					block_states,
					palette,
				} => {
					section_map_v1_13.insert(
						SectionY(section.y.into()),
						PaletteSection::new(data_version, Some(block_states), palette)
							.with_context(|| {
								format!("Failed to load section at Y={}", section.y)
							})?,
					);
				}
				de::SectionOldVariants::Old { blocks, data } => {
					section_map_old.insert(
						SectionY(section.y.into()),
						OldSection::new(blocks, data).with_context(|| {
							format!("Failed to load section at Y={}", section.y)
						})?,
					);
				}
				de::SectionOldVariants::Empty {} => {}
			}
		}

		// TODO Check biomes length
		let biomes = level.biomes.as_ref().context("Invalid biome data");

		Ok(
			match (section_map_v1_13.is_empty(), section_map_old.is_empty()) {
				(true, true) => Chunk::Empty,
				(false, true) => Chunk::V1_13 {
					section_map: section_map_v1_13,
					biomes: biomes?,
				},
				(true, false) => Chunk::Old {
					section_map: section_map_old,
					biomes: biomes?,
				},
				(false, false) => {
					bail!("Mixed section versions");
				}
			},
		)
	}

	pub fn sections(&self) -> SectionIter {
		use SectionIterInner::*;
		SectionIter {
			inner: match self {
				Chunk::V1_18 { section_map } => V1_18 {
					iter: section_map.iter(),
				},
				Chunk::V1_13 { section_map, .. } => V1_13 {
					iter: section_map.iter(),
				},
				Chunk::Old { section_map, .. } => Old {
					iter: section_map.iter(),
				},
				Chunk::Empty => Empty,
			},
		}
	}
}

trait SectionIterTrait<'a>:
	Iterator<Item = (SectionY, &'a dyn Section)>
	+ DoubleEndedIterator
	+ ExactSizeIterator
	+ FusedIterator
{
}

impl<'a, T> SectionIterTrait<'a> for T where
	T: Iterator<Item = (SectionY, &'a dyn Section)>
		+ DoubleEndedIterator
		+ ExactSizeIterator
		+ FusedIterator
{
}

impl<'a> SectionIter<'a> {
	fn with_iter<F, T>(&mut self, f: F) -> T
	where
		F: FnOnce(&mut dyn SectionIterTrait<'a>) -> T,
	{
		match &mut self.inner {
			SectionIterInner::V1_18 { iter } => f(
				&mut iter.map(|(y, section)| -> (SectionY, &'a dyn Section) { (*y, &section.0) })
			),
			SectionIterInner::V1_13 { iter } => {
				f(&mut iter.map(|(y, section)| -> (SectionY, &'a dyn Section) { (*y, section) }))
			}
			SectionIterInner::Old { iter } => {
				f(&mut iter.map(|(y, section)| -> (SectionY, &'a dyn Section) { (*y, section) }))
			}
			SectionIterInner::Empty => f(&mut iter::empty()),
		}
	}
}

impl<'a> Iterator for SectionIter<'a> {
	type Item = (SectionY, &'a dyn Section);

	fn next(&mut self) -> Option<Self::Item> {
		self.with_iter(|iter| iter.next())
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		match &self.inner {
			SectionIterInner::V1_18 { iter } => iter.size_hint(),
			SectionIterInner::V1_13 { iter } => iter.size_hint(),
			SectionIterInner::Old { iter } => iter.size_hint(),
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
			SectionIterInner::V1_13 { iter } => iter.len(),
			SectionIterInner::Old { iter } => iter.len(),
			SectionIterInner::Empty => 0,
		}
	}
}

impl<'a> FusedIterator for SectionIter<'a> {}
