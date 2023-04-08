use crate::token::Token;
use crate::replace_method::ReplaceMethod;

/// `TokenStore` is a trait that defines a storage interface for managing, retrieving,
/// and transforming design tokens.
pub trait TokenStore {
    /// Retrieves a token by its ID, returning a reference to the token.
    ///
    /// # Arguments
    ///
    /// * `id` - A reference to a `String` representing the unique token ID.
    ///
    /// # Returns
    ///
    /// * A reference to the `Token` object associated with the provided ID.
    fn token(&self, id: &String) -> &Token;

    /// Returns a list of all tokens in the store. Optionally, filters the list by theme.
    ///
    /// # Arguments
    ///
    /// * `theme` - An `Option<String>` representing an optional theme for filtering tokens. If `None`,
    ///   returns all tokens in the store without filtering.
    ///
    /// # Returns
    ///
    /// * A `Vec<&Token>` containing references to the tokens in the store, filtered by the
    ///   specified theme if provided.
    fn tokens(&self, theme: &Option<String>) -> Vec<&Token>;

    /// Transforms a token reference (a Handlebars-style string) into either the actual token value
    /// or a CSS variable selector, depending on the specified `ReplaceMethod` and theme.
    ///
    /// # Arguments
    ///
    /// * `reference` - A `String` representing the Handlebars-style token reference.
    /// * `replace_method` - A `ReplaceMethod` indicating how to replace the token reference.
    /// * `theme` - An `Option<String>` representing the optional theme to use for the replacement.
    ///
    /// # Returns
    ///
    /// * A `String` containing either the resolved token value or a CSS variable selector,
    ///   based on the `ReplaceMethod`.
    fn enrich(
        &self,
        reference: String,
        replace_method: ReplaceMethod,
        theme: &Option<String>,
    ) -> String;
}

#[cfg(test)]
pub mod test_utils {
	use regex::Captures;

	use super::TokenStore;
	use crate::regex::REGEX_HB;
	use crate::utils::css_stringify;
	use crate::{ReplaceMethod, Tokens, TokenSets, Themes};
	use crate::Token;

	#[derive(Default)]
	pub struct MockStore {
		pub tokens: Tokens,
		pub token_sets: TokenSets,
		pub themes: Themes
	}

	impl MockStore {
		pub fn new(tokens: Tokens, token_sets: TokenSets, themes: Themes) -> Self {
			Self { 
				tokens,
				token_sets,
				themes
			}
		}
	}

	impl TokenStore for MockStore {
		fn token(&self, id: &String) -> &Token {
			&self.tokens[id]
		}

		fn tokens(&self, theme: &Option<String>) -> Vec<&Token> {
			if let Some(key) = theme {
				// If the theme arg is provided, get the theme to check which sets should be active, and then filter to return only these tokens.
				let active_sets = self.themes.get(key).unwrap();
				active_sets.keys().map(|set_name| &self.token_sets[set_name]).flatten().map(|token_id| &self.tokens[token_id]).collect()
			} else {
				self.tokens.values().map(|t| t).collect::<Vec<&Token>>()
			}
		}

		fn enrich(&self, reference: String, replace_method: ReplaceMethod, theme: &Option<String>) -> String {
			REGEX_HB
				.replace_all(&reference, |caps: &Captures| {
					let name = &caps[1];

					match replace_method {
						ReplaceMethod::CssVariables => format!("var(--{})", css_stringify(&name.to_string())),
						ReplaceMethod::StaticValues => {
							if let Some(t) = self.tokens(theme).iter().find(|t| t.name() == name) {
								t.value(self, replace_method, true, theme)
							} else {
								String::from("BROKEN_REF")
							}
						}
					}
				})
				.to_string()
		}
	}
}