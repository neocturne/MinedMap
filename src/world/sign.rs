//! Processing of sign text

use std::sync::Arc;

use super::{
	de,
	json_text::{FormattedText, FormattedTextList, JSONText},
};

/// Version-independent reference to (front or back) sign text
#[derive(Debug, Default)]
pub struct RawSignText<'a> {
	/// Lines of sign text
	///
	/// A regular sign always has 4 lines of text. The back of pre-1.20
	/// signs is represented as a [SignText] without any `messages`.
	pub messages: Vec<&'a JSONText>,
	/// Sign color
	///
	/// Defaults to "black".
	pub color: Option<&'a str>,
}

impl<'a> RawSignText<'a> {
	/// Decodes the [RawSignText] into a [SignText]
	pub fn decode(&self) -> SignText {
		let color = self.color.map(|c| Arc::new(c.to_owned()));
		let parent = FormattedText {
			color,
			..Default::default()
		};
		SignText(
			self.messages
				.iter()
				.map(|message| message.deserialize().linearize(&parent))
				.collect(),
		)
	}
}

impl<'a> From<&'a de::BlockEntitySignV1_20Text> for RawSignText<'a> {
	fn from(value: &'a de::BlockEntitySignV1_20Text) -> Self {
		RawSignText {
			messages: value.messages.iter().collect(),
			color: value.color.as_deref(),
		}
	}
}

/// Helper methods for [de::BlockEntitySign]
pub trait BlockEntitySignExt {
	/// Returns the front and back text of a sign in a version-indepentent format
	fn text(&self) -> (RawSignText, RawSignText);
}

impl BlockEntitySignExt for de::BlockEntitySign {
	fn text(&self) -> (RawSignText, RawSignText) {
		match self {
			de::BlockEntitySign::V0 {
				text1,
				text2,
				text3,
				text4,
				color,
			} => (
				RawSignText {
					messages: vec![text1, text2, text3, text4],
					color: color.as_deref(),
				},
				Default::default(),
			),
			de::BlockEntitySign::V1_20 {
				front_text,
				back_text,
			} => (front_text.into(), back_text.into()),
		}
	}
}

/// Deserialized and linearized sign text
pub struct SignText(pub Vec<FormattedTextList>);

impl SignText {
	/// Checks if all lines of the sign text are empty
	pub fn is_empty(&self) -> bool {
		self.0.iter().all(|line| line.is_empty())
	}
}
