//! Newtype and helper methods for handling Minecraft Raw JSON Text

use std::{collections::VecDeque, sync::Arc};

use serde::Deserialize;

/// A span of formatted text
///
/// A [JSONText] consists of a tree of [FormattedText] nodes (canonically
/// represented as a [FormattedTextTree], but other kinds are possible with
/// is handled by [DeserializedText].
///
/// Formatting that is not set in a node is inherited from the parent.
#[derive(Debug, Deserialize, Default)]
pub struct FormattedText {
	#[serde(default)]
	/// Text content
	pub text: String,
	/// Text color
	pub color: Option<Arc<String>>,
	/// Bold formatting
	pub bold: Option<bool>,
	/// Italic formatting
	pub italic: Option<bool>,
	/// Underlines formatting
	pub underlined: Option<bool>,
	/// Strikethrough formatting
	pub strikethrough: Option<bool>,
	/// Obfuscated formatting
	pub obfuscated: Option<bool>,
}

impl FormattedText {
	/// Fills in unset formatting fields from a parent node
	pub fn inherit(self, parent: &Self) -> Self {
		FormattedText {
			text: self.text,
			color: self.color.or_else(|| parent.color.clone()),
			bold: self.bold.or(parent.bold),
			italic: self.italic.or(parent.italic),
			underlined: self.underlined.or(parent.underlined),
			strikethrough: self.strikethrough.or(parent.strikethrough),
			obfuscated: self.obfuscated.or(parent.obfuscated),
		}
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
#[derive(Debug)]
pub struct FormattedTextList(pub Vec<FormattedText>);

impl FormattedTextList {
	/// Returns `true` when [FormattedTextList] does not contain any text
	pub fn is_empty(&self) -> bool {
		self.0.iter().all(|text| text.text.is_empty())
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
