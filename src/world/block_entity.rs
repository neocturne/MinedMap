//! Processing of block entity data

use serde::{Deserialize, Serialize};

use super::{
	de,
	sign::{BlockEntitySignExt, SignText},
};

/// Kind of sign block
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignKind {
	/// Standing or attached sign
	Sign,
	/// Hanging sign
	HangingSign,
}

/// Processed sign data
#[derive(Debug, Serialize, Deserialize)]
pub struct Sign {
	/// The kind of the sign
	pub kind: SignKind,
	/// The sign's front text
	#[serde(skip_serializing_if = "SignText::is_empty", default)]
	pub front_text: SignText,
	/// The sign's back text
	#[serde(skip_serializing_if = "SignText::is_empty", default)]
	pub back_text: SignText,
}

impl Sign {
	/// Processes a [de::BlockEntitySign] into a [Sign]
	fn new(sign: &de::BlockEntitySign, kind: SignKind) -> Sign {
		let (front_text, back_text) = sign.text();
		let front_text = front_text.decode();
		let back_text = back_text.decode();
		Sign {
			kind,
			front_text,
			back_text,
		}
	}
}

/// Data for different kinds of [BlockEntity]
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlockEntityData {
	/// A sign block
	Sign(Sign),
}

/// A processed block entity
#[derive(Debug, Serialize, Deserialize)]
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
	pub fn new(entity: &de::BlockEntity) -> Option<Self> {
		let data = match &entity.data {
			de::BlockEntityData::Sign(sign) => {
				BlockEntityData::Sign(Sign::new(sign, SignKind::Sign))
			}
			de::BlockEntityData::HangingSign(sign) => {
				BlockEntityData::Sign(Sign::new(sign, SignKind::HangingSign))
			}
			de::BlockEntityData::Other => return None,
		};

		Some(BlockEntity {
			x: entity.x,
			y: entity.y,
			z: entity.z,
			data,
		})
	}
}
