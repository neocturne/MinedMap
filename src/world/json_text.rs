//! Newtype and helper methods for handling Minecraft Raw JSON Text

use serde::Deserialize;

/// Minecraft Raw JSON Text
#[derive(Debug, Deserialize)]
pub struct JSONText(String);
