mod css;
pub use css::*;

// mod json;
// pub use json::*;

use crate::Figtok;

pub trait Serializer {
	fn serialize(&self, ctx: &Figtok);
}