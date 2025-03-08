use serde_derive::{Serialize, Deserialize};
use serde::{Deserializer, Deserialize};

/// A struct for handling shadow values from Figma Token Studio.
/// 
/// # Structure
/// While shadows are provided as JSON objects like composition tokens,
/// they follow a consistent schema that allows us to deserialize them into
/// a strongly-typed struct.
///
/// # Deserialization
/// The `#[serde(untagged)]` enum `ShadowValueDeserializer` handles two possible formats:
/// - A single shadow layer object
/// - An array of shadow layer objects
///
/// Both formats are converted into a `Vec<ShadowLayer>` for consistent internal representation.
/// Each `ShadowLayer` can be either a drop shadow or inner shadow, specified by its `kind` field.
#[derive(Serialize, Debug, Clone)]
pub struct ShadowValue(pub Vec<ShadowLayer>);

#[derive(Deserialize)]
#[serde(untagged)]
enum ShadowValueDeserializer {
    Single(ShadowLayer),
    Multiple(Vec<ShadowLayer>),
}

impl<'de> Deserialize<'de> for ShadowValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = ShadowValueDeserializer::deserialize(deserializer)?;
        Ok(ShadowValue(match value {
            ShadowValueDeserializer::Single(layer) => vec![layer],
            ShadowValueDeserializer::Multiple(layers) => layers,
        }))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShadowLayer {
    pub(crate) color: String,
    #[serde(alias = "type", alias = "$type")]
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
