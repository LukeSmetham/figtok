use std::collections::HashMap;

extern crate serde;
extern crate serde_derive;
extern crate serde_json;

// Internal Modules
mod shadow_value;
mod utils;

// Public Modules
pub mod regex;
pub mod replace_method;
pub mod token;
pub mod token_definition;
pub mod token_kind;

// Some Type Aliases (May remove these)
pub type TokenSet = Vec<String>;
pub type TokenSets = HashMap<String, TokenSet>;
pub type Tokens = HashMap<String, token::Token>;
pub type Theme = HashMap<String, String>;
pub type Themes = HashMap<String, Theme>;