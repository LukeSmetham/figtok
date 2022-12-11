extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate once_cell;

mod tokens;
use tokens::{TokenDefinition, Tokens, TokenSet, TokenSets, Themes, Theme};

pub mod load;
use load::load;

pub mod serialize;
use serialize::{Serializer, CssSerializer, JsonSerializer};

use std::{error::Error, collections::HashMap};
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

		let figtok = Figtok {
			entry_path: entry_path.clone(),
			output_path: output_path.clone(),
			tokens: HashMap::new(),
            token_sets: HashMap::new(),
            themes: HashMap::new(),
			serializer,
		};

		let _ = figtok.prepare();

		figtok
    }

	fn prepare(&self) -> Result<(), Box<dyn Error>> {
		// Check output directory exists, and destroy it if truthy so we can clear any existing output files.
        if Path::new(&self.output_path).is_dir() {
            fs::remove_dir_all(&self.output_path)?;
        }

        // Now ensure the output_path dir exists.
        fs::create_dir_all(&self.output_path)?;

        // Check if the input directory exists
        if !Path::new(&self.entry_path).exists() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("No {} directory found, passed as input directory", self.entry_path),
            )));
        };

		Ok(())
	}

    pub fn load(&mut self) {
		load(self);
    }

    pub fn export(&self) {
        self.serializer.serialize(self);
	}

	pub fn get_tokens(&self) -> &Tokens {
		&self.tokens
	}

	pub fn get_token_sets(&self) -> &TokenSets {
		&self.token_sets
	}

	pub fn get_themes(&self) -> &Themes {
		&self.themes
	}

	pub fn add_token(&mut self, key: String, value: TokenDefinition) {
		self.tokens.insert(key, value);
	}
	
	pub fn add_token_set(&mut self, key: String, value: TokenSet) {
		self.token_sets.insert(key, value);
	}

	pub fn add_theme(&mut self, key: String, value: Theme) {
		self.themes.insert(key, value);
	}
}
