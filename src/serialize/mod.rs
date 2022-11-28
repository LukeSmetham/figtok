mod css;
pub use css::*;

mod json;
pub use json::*;

mod utils;

use std::error::Error;

use crate::{tokens::TokenDefinition, load::Loader};

pub trait Serializer {
	fn new() -> Self;
	fn serialize(&self, loader: &impl Loader, output_path: String) -> Result<(), Box<dyn Error>>;
	fn serialize_one(&self, loader: &impl Loader, token: &TokenDefinition) -> String;
}