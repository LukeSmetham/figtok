use std::ops::{Deref, DerefMut};
use std::slice::Iter;
use merge_struct::merge;

use serde_json::json;

use crate::{ReplaceMethod, TokenStore, Token};

/// A TokenSet stores a Vec of Token IDs, these Tokens are stored in the Figtok TokenStore
/// and can therefore be accessed by iterating over the ids.
pub struct TokenSet(pub Vec<String>);

impl TokenSet {
	pub fn new(tokens: Vec<String>) -> Self {
		TokenSet(tokens)
	}
}

impl Deref for TokenSet {
	type Target = Vec<String>;
	
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for TokenSet {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<'a> IntoIterator for &'a TokenSet {
	type Item = &'a String; 
	type IntoIter = Iter<'a, String>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.iter()
	}
}

impl <'a> IntoIterator for &'a mut TokenSet {
	type Item = &'a mut String;
	type IntoIter = std::slice::IterMut<'a, String>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.iter_mut()
	}
}