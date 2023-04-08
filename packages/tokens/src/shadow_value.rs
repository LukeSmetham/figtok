use serde_derive::{Serialize, Deserialize};

/// Figma Token Studio provides Shadow token values as Objects (similarly to a composition token)
/// However, unlike a composition token they have a predictable schema that we can build a struct from.
/// ShadowValue stores these values as a Vec of `ShadowLayer` structs that can be either a drop shadow
/// or an inner shadow.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShadowValue(pub Vec<ShadowLayer>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShadowLayer {
    pub(crate) color: String,
    #[serde(alias = "type")]
    pub(crate) kind: ShadowLayerKind,
    pub(crate) x: String,
    pub(crate) y: String,
    pub(crate) blur: String,
    pub(crate) spread: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ShadowLayerKind {
    #[serde(alias = "innerShadow")]
    InnerShadow,
    #[serde(alias = "dropShadow")]
    DropShadow,
}
