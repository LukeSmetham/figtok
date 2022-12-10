mod css;
pub use css::*;

mod json;
pub use json::*;

mod utils;

use std::error::Error;

use crate::{load::Loader};

pub trait Serializer<T: Loader> {
	fn serialize(&self, loader: &T, output_path: String) -> Result<(), Box<dyn Error>>;
}