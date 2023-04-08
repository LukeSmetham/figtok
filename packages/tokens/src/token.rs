use std::collections::HashMap;

use figtok::Figtok;
use serde_json::json;
use convert_case::{Case, Casing};

use crate::token_definition::TokenDefinition;
use crate::shadow_value::ShadowValue;
use crate::replace_method::ReplaceMethod;
use crate::regex::REGEX_CALC;
use crate::utils::{css_stringify, get_token_reference};

/// The Token enum encapsulates our different TokenDefinition variants, allowing us to store
/// them all together a single type (i.e. in a collection) whilst parsing/serializing each one
/// differently where necessary.
/// 
/// The Token enum also has some "getter" functions that alias the shared properties between token types
/// to give us an easy way to access inner values by a ref to an enum Token, and reduce the amount of match
/// statements everywhere.
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
    pub fn value(&self, ctx: &Figtok, replace_method: ReplaceMethod, nested: bool, theme: &Option<String>) -> String {
        let mut value = match self {
            Token::Standard(t) => t.get_value(ctx, replace_method, nested, theme),
            Token::Shadow(t) => t.get_value(ctx, replace_method, theme),
			// We never call value() on Composition tokens as it currently stands, instead we access the value directly to process the inner values of the composition token.
			// Composition tokens also can't be referenced by other tokens, which means this arm never runs when get_value is called to unfurl a token ref.
            Token::Composition(_) => todo!(), 
        };

		// We check a regex for a css arithmetic expression and if we have a match,
        // then we wrap the value in calc() so CSS can do the actual calculations for us,
        // and we still keep the references to token variables alive.
        if REGEX_CALC.is_match(&value) {
            value = format!("calc({})", value);
        };

        value
    }

	pub fn to_css(&self, ctx: &Figtok, replace_method: ReplaceMethod, theme: &Option<String>) -> String {
		match self {
			Token::Standard(_) | Token::Shadow(_) => {
				format!(
					"--{}: {};",
					css_stringify(&self.name()),
					self.value(ctx, replace_method, false, theme)
				)
			}
			Token::Composition(t) => {
				let mut class = String::new();

				class.push_str(format!(".{} {{", css_stringify(&t.name)).as_str());

				for (key, value) in t.value.as_object().unwrap() {
					// Here we call get_token_reference directly as the inner values of a composition token are not tokens in their own right, 
					//so don't already exist on ctx - but may still contain references to tokens.
					let token_value = get_token_reference(serde_json::from_value::<String>(value.to_owned()).unwrap(), ctx, replace_method, theme);
					class.push_str(
					format!(
							"{}: {};", 
							key.replace(".", "-").to_case(Case::Kebab),
							token_value
						).as_str()
					);
				};

				class.push_str("}");

				class
			},
		}
	}
	
	pub fn to_json(&self, ctx: &Figtok, replace_method: ReplaceMethod, theme: &Option<String>) -> serde_json::Value {
		match &self {
			Token::Standard(_) | Token::Shadow(_) => {
				let token_name = self.name();
				let mut key_parts = token_name.split(".").collect::<Vec<&str>>();
				key_parts.reverse();

				let value = self.value(ctx, replace_method, false, theme);
				
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
					let inner_value = get_token_reference(serde_json::from_value::<String>(property_value.to_owned()).unwrap(), ctx, replace_method, &theme);
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