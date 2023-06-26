extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod serialize;
pub mod load;
mod log;

use std::{fs, io};
use merge_struct::merge;

use serde_json::json;
use tokens::{
	Tokens, 
	TokenSets, 
	Themes, 
	Token,
	ReplaceMethod,
	regex::{REGEX_HB},
	utils::css_stringify,
	TokenStore, TokenSet
};
use regex::Captures;

pub struct Figtok {
    pub output_path: String,
	pub tokens: Tokens,
    pub token_sets: TokenSets,
    pub themes: Themes,
}

impl Figtok {
    pub fn new(tokens: Tokens, token_sets: TokenSets, themes: Themes, output_path: &String) -> Self {
		Figtok {
			output_path: output_path.clone(),
			tokens,
            token_sets,
            themes,
		}
    }
}

impl TokenStore for Figtok {
	fn token(&self, id: &String) -> &Token {
		&self.tokens[id]
	}

	fn tokens(&self, theme: &Option<String>) -> Vec<&tokens::Token> {
		if let Some(key) = theme {
			let active_sets = self.themes.get(key).unwrap();
			active_sets.keys().map(|set_name| &self.token_sets[set_name]).flatten().map(|token_id| &self.tokens[token_id]).collect()
		} else {
			self.tokens.values().map(|t| t).collect::<Vec<&Token>>()
		}
	}

	fn enrich(&self, reference: String, replace_method: ReplaceMethod, theme: &Option<String>) -> String {
		REGEX_HB
			.replace_all(&reference, |caps: &Captures| {
				// Get the reference (dot-notation) from the reference string without the surrounding curly brackets and use it to retrieve the referenced value.
				let name = &caps[1];

				match replace_method {
					// Convert the name of the token referenced in the reference string into a CSS var statement so CSS itself can handle the reference.
					ReplaceMethod::CssVariables => format!("var(--{})", css_stringify(&name.to_string())),
					// Get the value of the referenced token, so we can replace the handlebar ref in the original reference string.
					ReplaceMethod::StaticValues => {
						if let Some(t) = self.tokens(theme).iter().find(|t| t.name() == name) {
							t.value(self, replace_method, true, theme)
						} else {
							// No token with a matching name was found.
							// reference.clone()
							// TODO: Should we panic here instead? Wondering if it\s better to fail and let the user know that there is a token missing...
							// TODO: Returning "BROKEN_REF" is closer to the behavior with ReplaceMethod:CssVariables as if the ref is broken, the css will still be output, but won't work in practice.
							String::from("BROKEN_REF")
						}
					}
				}
			})
			.to_string()
	}
}
