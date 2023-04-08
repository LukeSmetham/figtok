use std::collections::HashMap;

use serde_json::json;
use convert_case::{Case, Casing};

use crate::token_definition::TokenDefinition;
use crate::shadow_value::ShadowValue;
use crate::replace_method::ReplaceMethod;
use crate::regex::REGEX_CALC;
use crate::token_store::TokenStore;
use crate::utils::{css_stringify};

/// The Token enum holds a TokenDefinition<T> and provides an abstraction with getters for the 
/// properties of a Token (name, id, kind, value.)
/// 
/// Other than the obvious advantage of being able to treat these different token variants as a single
/// type, we also get the benefit of being able to enrich token values against Figtok before returning
/// from a getter method. This powers the reference tokens, where a Tokens value is a handlebar-style
/// reference to another token in the store.
/// 
#[derive(Debug, Clone)]
pub enum Token {
    Standard(TokenDefinition<String>),
    Composition(TokenDefinition<serde_json::Value>),
    Shadow(TokenDefinition<ShadowValue>),
}
impl Token {
	/// Get the token name from the underlying TokenDefinition<T>
	pub fn name(&self) -> String {
		match self {
			Token::Standard(t) => t.name.clone(),
			Token::Composition(t) => t.name.clone(),
			Token::Shadow(t) => t.name.clone(),
		}
	}
	
	/// Get the token id from the underlying TokenDefinition<T>
	pub fn id(&self) -> String {
		match self {
			Token::Standard(t) => t.id.clone(),
			Token::Composition(t) => t.id.clone(),
			Token::Shadow(t) => t.id.clone(),
		}
	}

	/// Get the token value. This method calls the get_value() method of a TokenDefinition<T>, we can impl a different 
	/// get_value for each possible value of T that we want to support, ultimately producing a string containing the value
	/// of the token.
	/// 
	/// This is primarily used to access the value of a token, when we are expanding a token value that references another token.
	/// Because of this, it's only ever called directly for Standard tokens and Shadow tokens. Composition tokens are processed
	/// differently as they are serialized as CSS classes containing multiple properties, as appose to CSS Variables. 
    pub fn value(&self, store: &dyn TokenStore, replace_method: ReplaceMethod, nested: bool, theme: &Option<String>) -> String {
        let mut value = match self {
            Token::Standard(t) => t.get_value(store, replace_method, nested, theme),
            Token::Shadow(t) => t.get_value(store, replace_method, theme),
            Token::Composition(t) => {
				// Composition tokens are output as classes, containing properties for each inner value of the token.
				// Because of this, below instead of calling get_value directly on the token, we get the token value as_object() and
				// iterate through its members, enriching each inner value and writing it to a string.
				let mut result = String::new();

				for (key, value) in t.value.as_object().unwrap() {
					// Here we call enrich directly as the inner values of a composition token are not tokens in their own right, 
					//so don't already exist on store - but may still contain references to tokens.
					let token_value = store.enrich(serde_json::from_value::<String>(value.to_owned()).unwrap(), replace_method, theme);
					
					result.push_str(
					format!(
							"{}: {};", 
							key.replace(".", "-").to_case(Case::Kebab),
							token_value
						).as_str()
					);
				}

				result
			}, 
        };

		// We check a regex for a css arithmetic expression and if we have a match,
        // then we wrap the value in calc() so CSS can do the actual calculations for us,
        // and we still keep the references to token variables alive.
        if REGEX_CALC.is_match(&value) {
            value = format!("calc({})", value);
        };

        value
    }

	// TODO: Test this.
	pub fn serialize(&self, store: &dyn TokenStore, replace_method: ReplaceMethod, theme: &Option<String>) -> String {
		match self {
			Token::Standard(_) | Token::Shadow(_) => {
				format!(
					"--{}: {};",
					css_stringify(&self.name()),
					self.value(store, replace_method, false, theme)
				)
			}
			Token::Composition(_) => {
				let selector_name = &css_stringify(&self.name());
				format!(
					".{} {{{}}}", 
					selector_name, 
					&self.value(store, replace_method, false, theme)
				)
			},
		}
	}
	
	pub fn to_json(&self, store: &dyn TokenStore, replace_method: ReplaceMethod, theme: &Option<String>) -> serde_json::Value {
		match &self {
			Token::Standard(_) | Token::Shadow(_) => {
				let token_name = self.name();
				let mut key_parts = token_name.split(".").collect::<Vec<&str>>();
				key_parts.reverse();

				let value = self.value(store, replace_method, false, theme);
				
				let mut j = json!(value);
				for key in key_parts {
					j = json!({ key: j })
				};

				j
			}
			Token::Composition(t) => {
				let token_name = self.name();
				let mut key_parts = token_name.split(".").collect::<Vec<&str>>();
				key_parts.reverse();

				let mut properties: HashMap<String, String> = HashMap::new();

				for (property_name, property_value) in t.value.as_object().unwrap() {
					let inner_value = store.enrich(serde_json::from_value::<String>(property_value.to_owned()).unwrap(), replace_method, &theme);
					properties.insert(property_name.clone(), inner_value);
				}

				let mut j = serde_json::to_value(properties).unwrap();
				for key in key_parts {
					j = json!({ key: j })
				}

				j
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::token_store::test_utils::MockStore;
	use crate::TokenKind;

	mod value {
		use super::*;

		#[test]
		fn standard() {
			let store = MockStore::default();

			let token_definition = TokenDefinition {
				id: String::from("global.typescale.4"),
				name: String::from("typescale.4"),
				value: String::from("24px"),
				kind: TokenKind::Other
			};
			
			let token = Token::Standard(token_definition);

			assert_eq!(token.value(&store, ReplaceMethod::CssVariables, false, &None), "24px".to_string());
		}
		
		#[test]
		fn standard_reference() {
			// Define a token that we will reference later on
			let ref_definition = TokenDefinition {
				id: String::from("global.ref.grey.0"),
				name: String::from("ref.grey.0"),
				value: String::from("#000000"),
				kind: TokenKind::Color,
			};

			// Init a HashMap to store token, that we'll pass along to MockStore.
			let mut tokens = HashMap::new();
			// Insert the reference token into the HashMap.
			tokens.insert(ref_definition.id.clone(), Token::Standard(ref_definition));

			// Initialize a MockStore with our tokens, and empty HashMap's for token_sets and themes.
			let store = MockStore::new(tokens, HashMap::new(), HashMap::new());

			// Now create a token definition who's value is a reference to the token we created earlier.
			let token_definition = TokenDefinition {
				id: String::from("color.text"),
				name: String::from("color.text"),
				value: String::from("{ref.grey.0}"),
				kind: TokenKind::Color,
			};

			// Create a Token from the token_definition, and get a reference to our ref_token in the store.
			let token = Token::Standard(token_definition);
			let ref_token = &store.token(&String::from("global.ref.grey.0"));

			// Check the static replace method produces the expected output
			assert_eq!(
				token.value(&store, ReplaceMethod::StaticValues, false, &None), 
				String::from("rgb(0, 0, 0)")
			);

			// Check the css variables replace method produces the expected output.
			assert_eq!(
				token.value(&store, ReplaceMethod::CssVariables, false, &None), 
				format!(
					"rgb(var(--{}))", 
					css_stringify(&ref_token.name())
				)
			);
		}

		#[test]
		fn shadow() { todo!() }

		#[test]
		fn shadow_reference() { todo!() }

		#[test]
		fn composition() { todo!() }

		#[test]
		fn composition_reference() { todo!() }
	}
}