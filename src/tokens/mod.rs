#[derive(Serialize, Deserialize, Debug)]
pub enum TokenKind {
    #[serde(alias = "borderRadius")]
    BorderRadius,
    #[serde(alias = "color")]
    Color,
    #[serde(alias = "fontFamilies")]
    FontFamily,
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