mod css;
pub use css::*;

mod utils;

use std::error::Error;

use crate::{tokens::TokenDefinition, load::Loader};

pub trait Serializer {
	fn new() -> Self;
	fn run(&self, loader: &Loader, output_path: String) -> Result<(), Box<dyn Error>>;
	fn serialize_one(&self, loader: &Loader, token: &TokenDefinition) -> String;
}