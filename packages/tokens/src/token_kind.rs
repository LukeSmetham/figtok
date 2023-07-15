use serde_derive::Deserialize;
use serde::{Serialize, Serializer};

/// Each individual token type, usually mapping 1:1 to a css property with the exception of `Composition` and `Dimension`
/// 
/// `Composition` tokens store their value as a nested `serde_json::Value`, and serialize to class definitions as they can contain 
/// multiple values and token references that should all be applied at once. 
/// 
/// For Example, you may define a composition token for a "glass" effect, where your `surface` color, border-color 
/// and border-width, and backdrop-blur are all applied as a single class.
/// 
/// Dimension tokens are unique in that they contain a value, as well as the desired "dimension" (i.e. `rem`, `em`, `%`, `px`, `vw`, etc.)
#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
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
impl ToString for TokenKind {
	/// Handles the mapping of Self to css property name for use in serialization.
    fn to_string(&self) -> String {
        match self {
            TokenKind::BorderRadius => "border-radius",
            TokenKind::BorderWidth => "border-width",
            TokenKind::BoxShadow => "box-shadow",
            TokenKind::Color => "color",
            TokenKind::Composition => "composition",
            TokenKind::Dimension => "dimension",
            TokenKind::FontFamily => "font-family",
            TokenKind::FontSize => "font-size",
            TokenKind::FontWeights => "font-weight",
            TokenKind::LetterSpacing => "letter-spacing",
            TokenKind::LineHeight => "line-height",
            TokenKind::Opacity => "opacity",
            TokenKind::Sizing => "sizing",
            TokenKind::Spacing => "spacing",
            TokenKind::Typography => "typography",
            TokenKind::Other => "other",
        }
        .to_string()
    }
}
impl Serialize for TokenKind {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}