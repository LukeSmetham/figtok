mod css;
pub use css::*;

mod json;
pub use json::*;

mod utils;

use std::error::Error;

use crate::Figtok;

pub trait Serializer {
	fn serialize(&self, ctx: &Figtok) -> Result<(), Box<dyn Error>>;
}