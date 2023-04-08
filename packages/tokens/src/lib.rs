use std::collections::HashMap;

extern crate serde;
extern crate serde_derive;
extern crate serde_json;

// Internal Modules
mod replace_method;
mod shadow_value;
mod token;
mod token_store;
mod token_definition;
mod token_kind;

// Public Modules
pub mod regex;
pub mod utils;

// "Exports"
pub use shadow_value::ShadowValue;
pub use token::Token;
pub use token_store::TokenStore;
pub use token_definition::TokenDefinition;
pub use token_kind::TokenKind;
pub use replace_method::ReplaceMethod;

// Type Aliases for Collections of Tokens.
pub type TokenSet = Vec<String>;
pub type TokenSets = HashMap<String, TokenSet>;
pub type Tokens = HashMap<String, token::Token>;
pub type Theme = HashMap<String, String>;
pub type Themes = HashMap<String, Theme>;