extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod serialize;
mod load;
mod log;

pub use load::load;
pub use serialize::{Serializer, CssSerializer, JsonSerializer};
use figtok_tokens::{
	regex::REGEX_HB, utils::css_stringify, Themes, Token, TokenKind, TokenSets, TokenStore, Tokens, ValueAs
};
use regex::Captures;

pub struct Figtok {
    pub output_path: String,
	pub tokens: Tokens,
    pub token_sets: TokenSets,
    pub themes: Themes,
    pub token_set_order: Vec<String>,
	pub strategy: ValueAs,
}

impl Figtok {
    pub fn new(tokens: Tokens, token_sets: TokenSets, themes: Themes, token_set_order: Vec<String>, output_path: &String, strategy: ValueAs) -> Self {
		Figtok {
			output_path: output_path.clone(),
			tokens,
            token_sets,
            themes,
            token_set_order,
			strategy,
		}
    }

	pub fn serialize(&self, serializer: Box<dyn Serializer>) {
		serializer.serialize(self)
	}
}

impl TokenStore for Figtok {
	fn token(&self, id: &String) -> &Token {
		&self.tokens[id]
	}

	fn tokens(&self, theme: &Option<String>) -> Vec<&figtok_tokens::Token> {
		if let Some(key) = theme {
			let active_sets = self.themes.get(key).unwrap();
			active_sets.keys().map(|set_name| &self.token_sets[set_name]).flatten().map(|token_id| &self.tokens[token_id]).collect()
		} else {
			self.tokens.values().map(|t| t).collect::<Vec<&Token>>()
		}
	}

	fn enrich(&self, reference: String, value_as: ValueAs, theme: &Option<String>) -> String {
		REGEX_HB
			.replace_all(&reference, |caps: &Captures| {
				// Get the reference (dot-notation) from the reference string without the surrounding curly brackets and use it to retrieve the referenced value.
				let name = &caps[1];

				match value_as {
					ValueAs::CssVariables => {
						if let Some(t) = self.tokens(theme).iter().find(|t| t.name() == name) {
							match t.kind() {
								TokenKind::Color => {
									if t.is_reference() {
										self.enrich(t.value(self, value_as, theme), value_as, theme)
									} else {
										format!("var(--{})", css_stringify(&name.to_string()))
									}
								},
								_ => {
									format!("var(--{})", css_stringify(&name.to_string()))
								}
							}
						} else {
							String::from("BROKEN_REF")
						}
						
					},
					// Get the value of the referenced token, so we can replace the handlebar ref in the original reference string.
					ValueAs::StaticValues => {
						if let Some(t) = self.tokens(theme).iter().find(|t| t.name() == name) {
							t.value(self, value_as, theme)
						} else {
							// No token with a matching name was found.

							// TODO: Should we panic here instead? Wondering if it\s better to fail and let the user know that there is a token missing...
							// TODO: Returning "BROKEN_REF" is closer to the behavior with ValueAs:CssVariables as if the ref is broken, the css will still be output, but won't work in practice.
							String::from("BROKEN_REF")
						}
					}
				}
			})
			.to_string()
	}
}
