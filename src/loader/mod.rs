use std::error::Error;
use std::fs::read_to_string;
use std::collections::HashMap;

use crate::tokens::{TokenDefinition};

fn read_file(filepath: &String) -> Result<String, Box<dyn Error>> {
    let data = read_to_string(filepath)?;
    Ok(data)
}

#[derive(Debug)]
pub struct Loader {
	path: String,
	pub tokens: HashMap<String, TokenDefinition>,
	pub token_sets: HashMap<String, HashMap<String, String>>,
	pub themes: HashMap<String, HashMap<String, String>>,
}
impl Loader {
	pub fn new(path: &str) -> Loader {
		Loader {
			path: path.to_string(),
			tokens: HashMap::new(),
			token_sets: HashMap::new(),
			themes: HashMap::new()
		}
	}

	pub fn parse_token_set(&mut self, slug: &String, data: HashMap<String, serde_json::Value>, maybe_prefix: Option<&mut Vec<String>>) {
		let prefix = maybe_prefix.unwrap();

		for (key, value) in data {
			let mut id = prefix.clone();
			id.push(key.clone());

			let kind = value.get("type");
			match kind {
				Some(_) => {
					// If the "type" property is present, we have a token definition
					let mut token: TokenDefinition = serde_json::from_value(value).unwrap();

					// This can definitely be improved as far as a more robust check,
					// but we check here if the token value contains a reference to 
					// another token.
					// If so, because of the tokenSetOrder we can ensure that this token
					// has already been parsed, so we can enrich this token definition with
					// it's referenced value.
					token = match token.value.contains("{") {
						true => {
							token
						}
						false => {
							token
						}
					};

					token.name = id.join(".");

					let id_parts = vec![slug.split("/").collect::<Vec<&str>>().join("."), token.name.clone()];
					token.id = id_parts.join(".");
					
					// Store the token in it's respective token_set, as a KV pair of [token.id, token.name].
					// We can later use this for lookups by id, and serializing tokens under their name (the name property is relative to the theme.)
					let _ = &self.token_sets.entry(slug.to_string()).and_modify(|v| {
						v.insert(token.id.clone(), token.name.clone());
					});

					let _ = &self.tokens.insert(token.id.clone(), token);
				}
				None => {
					// If the "type" property is not present, we have a nested object
					let nested_data: HashMap<String, serde_json::Value> = serde_json::from_value(value).unwrap();
					let mut new_prefix = id.clone();

					let _ = &self.parse_token_set(slug, nested_data, Some(&mut new_prefix));
				}
			}
		}
	}

	pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
		let metadata_path = &mut self.path.clone();
		metadata_path.push_str("/$metadata.json");
		
		let themes_path = &mut self.path.clone();
		themes_path.push_str("/$themes.json");

		// This gives us an HashMap containing the "tokenSetOrder", a Vec<String> with
		// all of the token sets in order, matching their positions in figma tokens UI.
		let metadata: HashMap<String, Vec<String>> = serde_json::from_str(&read_file(metadata_path).unwrap())?;

		// Parse all of the tokens and token_sets recursively.
		for entry in metadata.get("tokenSetOrder") {
			for slug in entry {
				// use the slug to create the path to the relevant JSON file.
				let path = format!("./tokens/{}.json", slug);

				// Read the file as a string, and insert into the files map
				let file = read_file(&path)?;

				let token_set_data: HashMap<String, serde_json::Value> = serde_json::from_str(&file)?;
				let mut prefix: Vec<String> = vec![];

				let _ = &self.token_sets.insert(slug.clone(), HashMap::new());

				let _ = &self.parse_token_set(&slug.to_string(), token_set_data, Some(&mut prefix));
			}
		}

		let themes: Vec<serde_json::Value> = serde_json::from_str(&read_file(themes_path).unwrap())?;
		for theme in themes {
			let value = theme.get("selectedTokenSets").unwrap().to_owned();

			let token_sets = serde_json::from_value::<HashMap<String, String>>(value).unwrap();

			let enabled_sets: HashMap<String, String> = token_sets.into_iter().filter(|(_, v)| v != "disabled").collect();

			let theme_name = serde_json::from_value::<String>(theme.get("name").unwrap().to_owned()).unwrap();
			let _ = &self.themes.insert(theme_name, enabled_sets);
		}

		Ok(())
	}

	// fn serialize(&self, theme: &str) -> String {

	// }

	pub fn serialize_all(&self) -> Vec<String> {
		let mut themes: HashMap<String, Vec<&TokenDefinition>> = HashMap::new();
		
		for (name, sets) in &self.themes {
			let set_names: Vec<String> = sets.keys().into_iter().map(|key| key.clone()).collect();

			let mut tokens: Vec<&TokenDefinition> = Vec::new();
			for set_name in set_names {
				let token_id_map = self.token_sets[&set_name].clone();

				for (id, name) in token_id_map {
					let token = &self.tokens.get(&id).unwrap();
					// println!("{:?}", token);
					tokens.push(*token);
				}
			}

			themes.insert(name.clone(), tokens);
		}

		let mut output: Vec<String> = Vec::new();

		for (name, tokens) in themes {
			let mut theme_str = String::new();
			theme_str.push_str(":root{");
			for token in tokens {
				theme_str.push_str(format!("--{}: {};", token.name.replace(".", "-"), token.value).as_str());
			}
			theme_str.push_str("}");
			output.push(theme_str);
		}

		output
	}
}