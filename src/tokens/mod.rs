pub mod helpers;

use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TokenKind {
    #[serde(alias = "borderRadius")]
    BorderRadius,
	#[serde(alias = "borderWidth")]
	BorderWidth,
    #[serde(alias = "color")]
    Color,
    #[serde(alias = "fontFamilies")]
    FontFamily,
	#[serde(alias = "fontSizes")]
	FontSize,
	#[serde(alias = "fontWeights")]
	FontWeights,
	#[serde(alias = "letterSpacing")]
	LetterSpacing,
	#[serde(alias = "lineHeights")]
	LineHeight,
	#[serde(alias = "opacity")]
	Opacity,
	#[serde(alias = "sizing")]
	Sizing,
    #[serde(alias = "spacing")]
    Spacing,
    #[serde(alias = "other")]
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenDefinition {
	/// The value from the original json file for this token. May be a static value, or a reference using handlebars syntax e.g. {color.purple.1}
    pub value: String,
	/// Tells us what kind of token this is. Aliased from "type" field in the original json.
    #[serde(alias = "type")]
    pub kind: TokenKind,
	/// The name field is constructed as the dot-notated selector for the value in the original JSON file. e.g. "color.purple.1"
	#[serde(default)]
	pub name: String,
	#[serde(default)]
	pub id: String,
}

pub type TokenSet = Vec<String>;
pub type TokenSets = HashMap<String, TokenSet>;
pub type Tokens = HashMap<String, TokenDefinition>;
pub type Theme = HashMap<String, String>;
pub type Themes = HashMap<String, Theme>;