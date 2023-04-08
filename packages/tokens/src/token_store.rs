use crate::token::Token;
use crate::replace_method::ReplaceMethod;

pub trait TokenStore {
	/// Returns a list of all tokens in the store, and can optionally be scoped by theme to return all tokens in the store that belong to the given theme.
	fn get_tokens(&self, theme: &Option<String>) -> Vec<&Token>;
	/// Given a reference (A handlebars-style string that references a token in the store) return either
	/// the referenced values directly, or a valid css variable selector depending on ReplaceMethod
	fn enrich(&self, reference: String, replace_method: ReplaceMethod, theme: &Option<String>) -> String;
}

impl dyn TokenStore {
	pub fn test(&self) -> String {
		String::from("works")
	}
}