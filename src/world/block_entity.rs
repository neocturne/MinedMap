//! Processing of block entity data

use bincode::{Decode, Encode};
use minedmap_resource::{BlockFlag, BlockType};
use serde::Serialize;

use super::{
	de,
	sign::{BlockEntitySignExt, SignText},
};

/// Kind of sign block
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignKind {
	/// Standing sign
	Sign,
	/// Sign attached to wall
	WallSign,
	/// Hanging sign
	HangingSign,
	/// Hanging sign attached to wall
	HangingWallSign,
}

/// Processed sign data
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, Serialize)]
pub struct Sign {
	/// The kind of the sign
	pub kind: SignKind,
	/// The material of the sign
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub material: Option<String>,
	/// The sign's front text
	#[serde(skip_serializing_if = "SignText::is_empty", default)]
	pub front_text: SignText,
	/// The sign's back text
	#[serde(skip_serializing_if = "SignText::is_empty", default)]
	pub back_text: SignText,
}

impl Sign {
	/// Processes a [de::BlockEntitySign] into a [Sign]
	fn new(
		sign: &de::BlockEntitySign,
		kind: SignKind,
		material: Option<String>,
		data_version: u32,
	) -> Sign {
		let (front_text, back_text) = sign.text();
		let front_text = front_text.decode(data_version);
		let back_text = back_text.decode(data_version);
		Sign {
			kind,
			material,
			front_text,
			back_text,
		}
	}
}

/// Data for different kinds of [BlockEntity]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlockEntityData {
	/// A sign block
	Sign(Sign),
}

/// A processed block entity
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, Serialize)]
pub struct BlockEntity {
	/// Global X coordinate
	pub x: i32,
	/// Global Y coordinate
	pub y: i32,
	/// Global Z coordinate
	pub z: i32,
	/// Entity data
	#[serde(flatten)]
	pub data: BlockEntityData,
}

impl BlockEntity {
	/// Processes a [de::BlockEntity] into a [BlockEntity]
	pub fn new(
		entity: &de::BlockEntity,
		block_type: Option<&BlockType>,
		data_version: u32,
	) -> Option<Self> {
		let wall_sign = block_type
			.map(|block_type| block_type.block_color.is(BlockFlag::WallSign))
			.unwrap_or_default();
		let (kind, sign) = match (&entity.data, wall_sign) {
			(de::BlockEntityData::Sign(sign), false) => (SignKind::Sign, sign),
			(de::BlockEntityData::Sign(sign), true) => (SignKind::WallSign, sign),
			(de::BlockEntityData::HangingSign(sign), false) => (SignKind::HangingSign, sign),
			(de::BlockEntityData::HangingSign(sign), true) => (SignKind::HangingWallSign, sign),
			(de::BlockEntityData::Other, _) => return None,
		};
		let material = block_type
			.as_ref()
			.and_then(|block_type| block_type.sign_material.as_ref());
		let data = BlockEntityData::Sign(Sign::new(sign, kind, material.cloned(), data_version));

		Some(BlockEntity {
			x: entity.x,
			y: entity.y,
			z: entity.z,
			data,
		})
	}
}
