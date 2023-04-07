
use super::token_kind::{TokenKind};

use serde_derive::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

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

#[cfg(test)]
mod tests {
    use super::*;

    mod deserialize {
		use super::*;

		#[test]
		fn can_be_deserialized_from_str() {
			let token_definition  = r#"{"value":"24px","kind":"fontSizes","name":"fontSize.0","id":"fontSize.0"}"#;

			let token: TokenDefinition<String> = serde_json::from_str(token_definition).unwrap();

			assert_eq!(token.value, String::from("24px"));
			assert_eq!(token.kind.to_string(), TokenKind::FontSize.to_string());
			assert_eq!(token.name, String::from("fontSize.0"));
			assert_eq!(token.id, String::from("fontSize.0"));
		}
	}

	mod serialize {
		use super::*;

		#[test]
		fn serializes_values_to_css_str() {
			let token_definition = r#"{"value":"24px","kind":"other","name":"typescale.7","id":"fontSize.7"}"#;
			let expected = r#"--typescale-7: 24px;"#;

			let token: TokenDefinition<String> = serde_json::from_str(token_definition).unwrap();

			let result = serde_json::to_string(&token).unwrap();

			assert_eq!(result, expected);
		}
	}
}
