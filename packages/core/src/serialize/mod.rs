mod css;
use std::io;

pub use css::*;

mod json;
pub use json::*;

use crate::Figtok;

pub trait Serializer {
	fn serialize(&self, store: &Figtok);
	fn write_file(&self, file_name: String, contents: String) -> io::Result<()>;
}