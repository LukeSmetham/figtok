mod utils;
mod json;
pub use json::*;

use std::collections::HashMap;

use crate::tokens::TokenDefinition;

pub trait Loader {
	fn new() -> Self;
	fn load(&mut self, entry_path: &String);
	fn get_tokens(&self) -> &HashMap<String, TokenDefinition>;
	fn get_token_sets(&self) -> &HashMap<String, Vec<String>>;
	fn get_themes(&self) -> &HashMap<String, HashMap<String, String>>;
}