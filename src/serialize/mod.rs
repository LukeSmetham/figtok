mod css;
pub use css::*;

mod utils;

use std::error::Error;

use crate::tokens::TokenDefinition;

pub trait Serializer {
	fn run(&self) -> Result<(), Box<dyn Error>>;
	fn serialize_one(&self, token: &TokenDefinition) -> String;
}