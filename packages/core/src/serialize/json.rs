use std::{default::Default, fs, io};
use merge_struct::merge;
use serde_json::json;

use crate::{Figtok, TokenStore, log};
use tokens::{ReplaceMethod, TokenSet};

use super::{
	Serializer,
};

#[derive(Default)]
pub struct JsonSerializer {}
impl Serializer for JsonSerializer {
	fn serialize(&self, store: &Figtok) {
		if !store.themes.is_empty() {
			self.serialize_themes(store);
		} else {
			self.serialize_token_sets(store);
		}
	}
	fn write_file(&self, file_name: String, contents: String) -> io::Result<()> {
		fs::write(
			format!("{}.json", file_name),
			contents
		)
	}
}
impl JsonSerializer {
	pub fn new() -> Self {
		JsonSerializer {}
	}

	pub fn serialize_themes(&self, store: &Figtok) {
		log!("Detected {} themes...", store.themes.len());

		for (name, sets) in &store.themes {
			let mut value = json!({});
			log!("Generating Theme: {}", name);

			for set_name in sets.into_iter().filter(|(_, v)| v.as_str() != "disabled").map(|(k, _)| k).collect::<Vec<&String>>() {
				let token_set: &TokenSet  = &store.token_sets[set_name];

				for id in token_set {
					let token = store.token(id);

					value = merge(&value, &token.to_json(store, ReplaceMethod::StaticValues, &Some(name.clone()))).unwrap();
				};
			}

			// Write the css file.
			let name_parts: Vec<&str> = name.split("/").map(|s| s.trim()).collect();
			let _ = self.write_file(
				[store.output_path.to_string(), name_parts.join("-")].join("/"),
				value.to_string(),
			);
		}
	}

	pub fn serialize_token_sets(&self, store: &Figtok) {
		log!("Detected {} token sets...", store.token_sets.len());

		for (set_name, token_set) in &store.token_sets {
			let mut value = json!({});

			for id in token_set {
				let token = store.token(id);

				value = merge(&value, &token.to_json(store, ReplaceMethod::StaticValues, &None)).unwrap();
			};

			// Now we make sure the output directory exists, and write the CSS file to disk

			// Split the set name by any /'s in case they are nested but remove the
			// last portion as this will be the file name not a directory
			let dir = if let Some((d,_)) = set_name.rsplit_once("/") {
				d
			} else {
				""
			};

			// Ensure the directories we need exist for token sets
			fs::create_dir_all([store.output_path.clone(), dir.to_string()].join("/")).unwrap();

			// Write the json file.
			let _ = self.write_file(
				[store.output_path.to_string(), set_name.to_string()].join("/"), 
				value.to_string()
			);
		}
	}
}