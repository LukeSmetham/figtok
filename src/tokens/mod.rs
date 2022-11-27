use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TokenKind {
    #[serde(alias = "borderRadius")]
    BorderRadius,
    #[serde(alias = "color")]
    Color,
    #[serde(alias = "fontFamilies")]
    FontFamily,
	#[serde(alias = "fontSizes")]
	FontSize,
	#[serde(alias = "letterSpacing")]
	LetterSpacing,
	#[serde(alias = "lineHeights")]
	LineHeight,
    #[serde(alias = "spacing")]
    Spacing,
    #[serde(alias = "other")]
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenDefinition {
    pub value: String,
    #[serde(alias = "type")]
    pub kind: TokenKind,
	#[serde(default)]
	pub name: String,
	#[serde(default)]
	pub id: String,
}