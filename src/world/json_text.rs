//! Newtype and helper methods for handling Minecraft Raw JSON Text

use std::{collections::VecDeque, fmt::Display};

use bincode::{Decode, Encode};
use minedmap_resource::Color;
use serde::{Deserialize, Serialize};

/// A span of formatted text
///
/// A [JSONText] consists of a tree of [FormattedText] nodes (canonically
/// represented as a [FormattedTextTree], but other kinds are possible with
/// is handled by [DeserializedText].
///
/// Formatting that is not set in a node is inherited from the parent.
#[derive(
	Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Encode, Decode,
)]
pub struct FormattedText {
	#[serde(default)]
	/// Text content
	pub text: String,
	/// Text color
	#[serde(skip_serializing_if = "Option::is_none", with = "json_color")]
	pub color: Option<Color>,
	/// Bold formatting
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bold: Option<bool>,
	/// Italic formatting
	#[serde(skip_serializing_if = "Option::is_none")]
	pub italic: Option<bool>,
	/// Underlines formatting
	#[serde(skip_serializing_if = "Option::is_none")]
	pub underlined: Option<bool>,
	/// Strikethrough formatting
	#[serde(skip_serializing_if = "Option::is_none")]
	pub strikethrough: Option<bool>,
	/// Obfuscated formatting
	#[serde(skip_serializing_if = "Option::is_none")]
	pub obfuscated: Option<bool>,
}

impl FormattedText {
	/// Fills in unset formatting fields from a parent node
	pub fn inherit(self, parent: &Self) -> Self {
		FormattedText {
			text: self.text,
			color: self.color.or(parent.color),
			bold: self.bold.or(parent.bold),
			italic: self.italic.or(parent.italic),
			underlined: self.underlined.or(parent.underlined),
			strikethrough: self.strikethrough.or(parent.strikethrough),
			obfuscated: self.obfuscated.or(parent.obfuscated),
		}
	}
}

impl Display for FormattedText {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.text.fmt(f)
	}
}

/// A tree of [FormattedText] nodes
///
/// Each node including the root has a `text` and a list of children (`extra`).
#[derive(Debug, Deserialize, Default)]
pub struct FormattedTextTree {
	/// Root node content
	#[serde(flatten)]
	text: FormattedText,
	/// List of child trees
	#[serde(default)]
	extra: VecDeque<DeserializedText>,
}

impl From<String> for FormattedTextTree {
	fn from(value: String) -> Self {
		FormattedTextTree {
			text: FormattedText {
				text: value,
				..Default::default()
			},
			extra: VecDeque::new(),
		}
	}
}

/// List of [FormattedText]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Encode, Decode)]
pub struct FormattedTextList(pub Vec<FormattedText>);

impl FormattedTextList {
	/// Returns `true` when [FormattedTextList] does not contain any text
	pub fn is_empty(&self) -> bool {
		self.0.iter().all(|text| text.text.is_empty())
	}
}

impl Display for FormattedTextList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for text in &self.0 {
			text.fmt(f)?;
		}

		Ok(())
	}
}

/// Raw deserialized [JSONText]
///
/// A [JSONText] can contain various different JSON types.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DeserializedText {
	/// Unformatted string
	String(String),
	/// Unformatted number (will be converted to a string)
	Number(f32),
	/// Unformatted boolean (will be converted to a string)
	Boolean(bool),
	/// List of [DeserializedText]
	///
	/// The tail elements are appended as children of the head element.
	List(VecDeque<DeserializedText>),
	/// The canonical [FormattedTextTree] structure
	Object(FormattedTextTree),
}

impl DeserializedText {
	/// Converts a [DeserializedText] into the regular [FormattedTextTree] format
	///
	/// Most variants are simply converted to strings. A list is handled by
	/// appending all tail elements to the `extra` field of the head.
	pub fn canonicalize(self) -> FormattedTextTree {
		match self {
			DeserializedText::Object(obj) => obj,
			DeserializedText::String(s) => FormattedTextTree::from(s),
			DeserializedText::Number(n) => FormattedTextTree::from(n.to_string()),
			DeserializedText::Boolean(b) => FormattedTextTree::from(b.to_string()),
			DeserializedText::List(mut list) => {
				let mut obj = list
					.pop_front()
					.map(|t| t.canonicalize())
					.unwrap_or_default();
				obj.extra.append(&mut list);
				obj
			}
		}
	}

	/// Converts the tree of [FormattedText] nodes into a linear list by
	/// copying formatting flags into each node.
	pub fn linearize(self, parent: &FormattedText) -> FormattedTextList {
		let obj = self.canonicalize();
		let mut ret = vec![obj.text.inherit(parent)];

		for extra in obj.extra {
			ret.append(&mut extra.linearize(&ret[0]).0);
		}

		FormattedTextList(ret)
	}
}

impl Default for DeserializedText {
	fn default() -> Self {
		DeserializedText::Object(FormattedTextTree::from(String::new()))
	}
}

/// Minecraft Raw JSON Text
#[derive(Debug, Deserialize)]
pub struct JSONText(pub String);

impl JSONText {
	/// Deserializes a [JSONText] into a [DeserializedText]
	pub fn deserialize(&self) -> DeserializedText {
		serde_json::from_str(&self.0).unwrap_or_default()
	}
}

mod json_color {
	//! Helpers for serializing and deserializing [FormattedText](super::FormattedText) colors

	use minedmap_resource::Color;
	use serde::{
		Deserializer, Serializer,
		de::{self, Visitor},
		ser::Error as _,
	};

	/// Named JSON text colors
	static COLORS: phf::Map<&'static str, Color> = phf::phf_map! {
		"black" => Color([0x00, 0x00, 0x00]),
		"dark_blue" => Color([0x00, 0x00, 0xAA]),
		"dark_green" => Color([0x00, 0xAA, 0x00]),
		"dark_aqua" => Color([0x00, 0xAA, 0xAA]),
		"dark_red" => Color([0xAA, 0x00, 0x00]),
		"dark_purple" => Color([0xAA, 0x00, 0xAA]),
		"gold" => Color([0xFF, 0xAA, 0x00]),
		"gray" => Color([0xAA, 0xAA, 0xAA]),
		"dark_gray" => Color([0x55, 0x55, 0x55]),
		"blue" => Color([0x55, 0x55, 0xFF]),
		"green" => Color([0x55, 0xFF, 0x55]),
		"aqua" => Color([0x55, 0xFF, 0xFF]),
		"red" => Color([0xFF, 0x55, 0x55]),
		"light_purple" => Color([0xFF, 0x55, 0xFF]),
		"yellow" => Color([0xFF, 0xFF, 0x55]),
		"white" => Color([0xFF, 0xFF, 0xFF]),
	};

	/// serde serialize function for [FormattedText::color](super::FormattedText::color)
	pub fn serialize<S>(color: &Option<Color>, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let &Some(color) = color else {
			return Err(S::Error::custom("serialize called for None sign color"));
		};

		let text = format!("#{:02x}{:02x}{:02x}", color.0[0], color.0[1], color.0[2]);
		serializer.serialize_str(&text)
	}

	/// serde [Visitor] for use by [deserialize]
	struct ColorVisitor;

	impl Visitor<'_> for ColorVisitor {
		type Value = Option<Color>;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			formatter.write_str("a string representing a color")
		}

		fn visit_str<E>(self, color: &str) -> Result<Self::Value, E>
		where
			E: de::Error,
		{
			if let Some(hex) = color.strip_prefix("#") {
				if let Ok(value) = u32::from_str_radix(hex, 16) {
					return Ok(Some(Color([
						(value >> 16) as u8,
						(value >> 8) as u8,
						value as u8,
					])));
				}
			}

			Ok(COLORS.get(color).copied())
		}
	}

	/// serde deserialize function for [FormattedText::color](super::FormattedText::color)
	pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_str(ColorVisitor)
	}
}
