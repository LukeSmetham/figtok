pub mod helpers;

use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TokenKind {
    #[serde(alias = "borderRadius")]
    BorderRadius,
	#[serde(alias = "borderWidth")]
	BorderWidth,
	#[serde(alias = "boxShadow")]
	BoxShadow,
    #[serde(alias = "color")]
    Color,
    #[serde(alias = "composition")]
    Composition,
    #[serde(alias = "dimension")]
    Dimension,
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
	#[serde(alias = "typography")]
	Typography,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CompositionTokenDefinition {
	/// Stores string references to the tokens used in this composition token.
	pub value: serde_json::Value,
	/// Tells us what kind of token this is. Aliased from "type" field in the original json.
    #[serde(alias = "type")]
    pub kind: TokenKind,
	/// The name field is constructed as the dot-notated selector for the value in the original JSON file. e.g. "color.purple.1"
	#[serde(default)]
	pub name: String,
	#[serde(default)]
	pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompositionToken {
	/// Stores string references to the tokens used in this composition token.
	pub tokens: Vec<String>,
	/// The name field is constructed as the dot-notated selector for the value in the original JSON file. e.g. "color.purple.1"
	#[serde(default)]
	pub name: String,
	#[serde(default)]
	pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypographyValue {
	#[serde(alias="fontFamily")]
	font_family: Option<String>,
	#[serde(alias="fontWeight")]
	font_weight: Option<String>,
	#[serde(alias="lineHeight")]
	line_height: Option<String>,
	#[serde(alias="fontSize")]
	font_size: Option<String>,
	#[serde(alias="letterSpacing")]
	letter_spacing: Option<String>,
}
impl IntoIterator for TypographyValue {
	type Item = String;
	type IntoIter = TypographyValueIter;

	fn into_iter(self) -> Self::IntoIter {
		TypographyValueIter {
			value: self,
			index: 0
		}
	}
}

pub struct TypographyValueIter {
	value: TypographyValue,
	index: usize
}
impl Iterator for TypographyValueIter {
	type Item = String;

	fn next(&mut self) -> Option<String> {
		let result = match self.index {
			0 => self.value.font_size.clone(),
			1 => self.value.line_height.clone(),
			2 => self.value.font_family.clone(),
			_ => None
		};

		self.index += 1;
		result
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoxShadowTokenLayer {
	color: String, 
	#[serde(alias="type")]
	kind: String,
	x: String, 
	y: String,
	blur: String, 
	spread: String,
}

pub type TokenSet = Vec<String>;
pub type TokenSets = HashMap<String, TokenSet>;
pub type Tokens = HashMap<String, TokenDefinition>;
pub type CompositionTokens = HashMap<String, CompositionToken>;
pub type Theme = HashMap<String, String>;
pub type Themes = HashMap<String, Theme>;