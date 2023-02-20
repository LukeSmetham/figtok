extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate once_cell;

mod tokens;
pub mod load;
pub mod serialize;

use tokens::{Tokens, TokenSets, Themes, Token};

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

	pub fn get_tokens(&self, theme: &Option<String>) -> Vec<&Token> {
		if let Some(key) = theme {
			let active_sets = self.themes.get(key).unwrap();
			active_sets.keys().map(|set_name| &self.token_sets[set_name]).flatten().map(|token_id| &self.tokens[token_id]).collect()
		} else {
			self.tokens.values().map(|t| t).collect::<Vec<&Token>>()
		}
	}
}
