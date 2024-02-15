use crate::token_kind::TokenKind;
use crate::shadow_value::{ShadowLayerKind, ShadowValue};
use crate::replace_method::ReplaceMethod;
use crate::regex::REGEX_HB;
use crate::token_store::TokenStore;

use colors_transform::{Color, Rgb};
use serde_derive::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

/// A TokenDefinition stores the raw data of a token, with a generic property denoting the type of token (Standard, Shadow, Composition, etc.)
/// Most tokens are expressed as `Standard` tokens - where the value is a simple `String`, although `Shadow` and `Composition` tokens require 
/// different serialization methods, and therefore we can impl `get_value` for each type. 
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(bound(deserialize = "T: DeserializeOwned"))]
pub struct TokenDefinition<T> {
    /// The value from the original json file for this token. May be a static value, or a reference using handlebars syntax e.g. {color.purple.1}
    pub value: T,
    /// Tells us what kind of token this is. Aliased from "type" field in the original json.
    #[serde(alias = "type")]
    pub kind: TokenKind,
    /// The name field is constructed as the dot-notated selector for the value in the original JSON file. e.g. "color.purple.1"
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub id: String,
}

impl TokenDefinition<String> {
    pub fn get_value(&self, store: &dyn TokenStore, replace_method: ReplaceMethod, nested: bool, theme: &Option<String>) -> String {
        let value = if REGEX_HB.is_match(&self.value) {
            let mut v = store.enrich(self.value.to_string(), replace_method, &theme);

            if self.kind == TokenKind::Color && !v.starts_with("rgb") && !nested {
                v = format!("rgb({})", v);
            }

            v
        } else {
            if TokenKind::Color == self.kind {
                Rgb::from_hex_str(&self.value)
                    .map(|rgb| format!("{}, {}, {}", rgb.get_red(), rgb.get_green(), rgb.get_blue()))
                    .unwrap_or_else(|_| self.value.clone())  // gracefully handle error
            } else {
                self.value.clone()
            }
        };

        value
    }
}

impl TokenDefinition<ShadowValue> {
    pub fn get_value(&self, store: &dyn TokenStore, replace_method: ReplaceMethod, theme: &Option<String>) -> String {
        let mut value = String::new();

        for layer in &self.value.0 {
            let color = if !layer.color.starts_with("rgb") { format!("rgb({})", layer.color) } else { layer.color.clone() };

            match layer.kind {
                ShadowLayerKind::DropShadow => value.push_str(&format!(
                    "{}px {}px {}px {}px {}, ",
                    layer.x, 
                    layer.y, 
                    layer.blur,
                    layer.spread,
                    color
                )),
                ShadowLayerKind::InnerShadow => value.push_str(&format!(
                    "inset {}px {}px {}px {}px {}, ",
                    layer.x, 
                    layer.y, 
                    layer.blur,
                    layer.spread,
                    color
                )),
            };
        }

        // Remove the trailing comma and space
        value.pop();
        value.pop();

        store.enrich(value, replace_method, &theme)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod deserialize {
		use super::*;
	use test_case::test_case;

		#[test_case("{\"value\":\"24px\",\"kind\":\"fontSizes\",\"name\":\"fontSize.0\",\"id\":\"fontSize.0\"}" ; "Example Definition #1")]
		fn can_be_deserialized_from_str(def: &str) {
			// In practice we use std to read the string from JSON files on disk.
			let token: TokenDefinition<String> = serde_json::from_str(def).unwrap();

			assert_eq!(token.value, String::from("24px"));
			assert_eq!(token.kind.to_string(), TokenKind::FontSize.to_string());
			assert_eq!(token.name, String::from("fontSize.0"));
			assert_eq!(token.id, String::from("fontSize.0"));
		}
	}
}
