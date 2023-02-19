extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate once_cell;

mod tokens;
use tokens::{Tokens, TokenSets, Themes};

pub mod load;
use load::load;

pub mod serialize;
use serialize::{Serializer, CssSerializer, JsonSerializer};

use std::{collections::HashMap};
use std::fs;
use std::path::Path;

pub struct Figtok {
    entry_path: String,
    output_path: String,

	tokens: Tokens,
    token_sets: TokenSets,
    themes: Themes,

    pub serializer: Box<dyn Serializer>,
}

impl Figtok {
    pub fn new(format: &String, entry_path: &String, output_path: &String) -> Self {
		let serializer: Box<dyn Serializer> = match format.as_str() {
			"css" => Box::new(CssSerializer::new()),
			"json" => Box::new(JsonSerializer::new()),
			f => panic!("Unsupported output format {}", f)
		};

		let mut figtok = Figtok {
			entry_path: entry_path.clone(),
			output_path: output_path.clone(),
			tokens: HashMap::new(),
            token_sets: HashMap::new(),
            themes: HashMap::new(),
			serializer,
		};

		// Check output directory exists, and destroy it if truthy so we can clear any existing output files.
        if Path::new(&figtok.output_path).is_dir() {
            fs::remove_dir_all(&figtok.output_path).unwrap();
        }

        // Now ensure the output_path dir exists.
        fs::create_dir_all(&figtok.output_path).unwrap();

        // Check if the input directory exists
        if !Path::new(&figtok.entry_path).exists() {
            panic!("No {} directory found, passed as input directory", figtok.entry_path);
        };

		load(&mut figtok);

		figtok
    }

    pub fn export(&self) {
        self.serializer.serialize(self);
	}
}
