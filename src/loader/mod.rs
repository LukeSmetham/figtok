use std::fs;
use std::collections::HashMap;
use std::error::Error;
use regex::Regex;
use lazy_static::lazy_static;
use colors_transform::{Rgb, Color};

use crate::tokens::{TokenDefinition, TokenKind};

fn read_file(filepath: &String) -> Result<String, Box<dyn Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}

#[derive(Debug)]
pub struct Loader {
	path: String,
	out: String,
	pub tokens: HashMap<String, TokenDefinition>,
	pub token_sets: HashMap<String, HashMap<String, String>>,
	pub themes: HashMap<String, HashMap<String, String>>
}
impl Loader {
	pub fn new(path: &str, out: &str) -> Loader {
		fs::create_dir_all(out).unwrap();
		Loader {
			path: path.to_string(),
			out: out.to_string(),
			tokens: HashMap::new(),
			token_sets: HashMap::new(),
			themes: HashMap::new()
		}
	}

	/// Recursively iterate through the token JSON, and add the data to self.tokens
	fn parse_token_set(&mut self, slug: &String, data: HashMap<String, serde_json::Value>, maybe_prefix: Option<&mut Vec<String>>) {
		lazy_static! {
			static ref RE: Regex = Regex::new(r"\{(.*)\}").unwrap();
		}

		let prefix = maybe_prefix.unwrap();

		for (key, value) in data {
			let mut id = prefix.clone();
			id.push(key.clone());

			let kind = value.get("type");
			match kind {
				Some(_) => {
					// If the "type" property is present, we have a token definition
					let mut token: TokenDefinition = serde_json::from_value(value).unwrap();

					// do any transformations per token kind
					token = match token.kind {
						TokenKind::Color => {
							// if the token is not a reference to another token,
							// then convert it to rgb.
							if !RE.is_match(&token.value) {
								let rgb = Rgb::from_hex_str(&token.value).unwrap();
								token.value = format!("{}, {}, {}", rgb.get_red(), rgb.get_green(), rgb.get_blue());
							}
							token
						},
						TokenKind::BorderRadius => {
							token
						},
						TokenKind::FontFamily => {
							token
						}
						TokenKind::Spacing => {
							token
						}
						TokenKind::Other => {
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

	/// Tests if a value is a static value or a reference. If static it's returned as is,
	/// whereas if it's a reference we go and retrieve the token, and either set the value 
	/// in place, or replace the handlebar reference string with css variable syntax depending 
	/// on the replace_with_value arg.
	fn enrich_token_value(&self, value: String, replace_with_value: bool) -> String {
		lazy_static! {
			static ref RE: Regex = Regex::new(r"\{(.*)\}").unwrap();
		}

		// Check if the value contains handlebar syntax with a reference to another token.
		if RE.is_match(&value) {
			let captures = RE.captures(&value).unwrap();

			// Get the ref string without the surrounding curly brackets and use it to retrieve the referenced token
			let ref_id = &captures[1];
			let ref_token = &self.tokens.values().find(|t| t.name == ref_id);

			match ref_token {
				Some(t) => {
					if !replace_with_value {
						// Replace the reference string with a css variable that points to the other token.
						let mut value = RE.replace(&value.to_string(), format!("var(--{})", t.name.clone().replace(".", "-"))).to_string();
						if !&value.starts_with("rgb") {
							value = format!("rgb({})", value);
						}
						value
					} else {
						// replace the reference string with the value of the referenced token statically.
						RE.replace(&value.to_string(), t.value.clone()).to_string()
					}
				}
				None => {
					value
				}
			}
		} else {
			value
		}
	}

	/// Take a single TokenDefinition, and serialize it to a CSS Variable string.
	fn serialize_token(&self, token: &TokenDefinition) -> String {
		let value = self.enrich_token_value(token.value.clone(), false);
		format!("--{}: {};", token.name.replace(".", "-"), value)
	}

	/// Iterate over all token sets and themes, creating CSS files for each with valid references to each other.
	/// Themes import the relevant sets individually, and Token Sets are outputted to their own CSS files that 
	/// can be imported individually by the user for more granularity, or if they don't use themes.
	pub fn serialize(&self) -> Result<(), Box<dyn Error>> {
		// Loop over the token sets and create a CSS file for each
		for (set_name, token_set) in &self.token_sets {
			let mut value = String::new();
			value.push_str(":root{");
			for (id, _) in token_set {
				let token = &self.tokens[id];
				value.push_str(self.serialize_token(token).as_str());
			}
			value.push_str("}");

			let dir = match set_name.rsplit_once("/") {
				Some((d, _)) => {
					d
				},
				None => {
					""
				}
			};

			fs::create_dir_all(vec![self.out.clone(), dir.to_string()].join("/")).unwrap();
			let _ = fs::write(format!("{}/{}.css", &self.out, set_name), value);
		}
		
		// Iterate over the themes and create import statements for each included set.
		for (name, sets) in &self.themes {
			let set_names: Vec<String> = sets.keys().into_iter().map(|key| key.clone()).collect();
			
			let mut value = String::new();

			for set in set_names {
				value.push_str(format!("@import \"./{}.css\";", set).as_str());
			}

			
			// Themes must be output to the top level so that the import paths work
			// we can probably work around this if we want as things improve.

			let name_parts: Vec<&str>  = name.split("/").map(|s| s.trim()).collect();
			let _ = fs::write(format!("{}/{}.css", &self.out, name_parts.join("-")), value);
		}

		Ok(())
	}
}