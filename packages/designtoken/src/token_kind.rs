use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
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