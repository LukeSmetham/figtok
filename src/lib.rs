extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate once_cell;

mod tokens;
use tokens::{Tokens, TokenSets, Themes};

pub mod load;

pub mod serialize;
use serialize::{Serializer};

pub struct Figtok {
    output_path: String,

	tokens: Tokens,
    token_sets: TokenSets,
    themes: Themes,

    pub serializer: Box<dyn Serializer>,
}

impl Figtok {
    pub fn new(tokens: Tokens, token_sets: TokenSets, themes: Themes, serializer: Box<dyn Serializer>, output_path: &String) -> Self {
		Figtok {
			output_path: output_path.clone(),
			tokens,
            token_sets,
            themes,
			serializer,
		}
    }

    pub fn export(&self) {
        self.serializer.serialize(self);
	}
}
