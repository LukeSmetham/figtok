mod utils;
use utils::read_file;

use colors_transform::{Color, Rgb};
use std::collections::HashMap;

use crate::Figtok;
use crate::tokens::helpers::REGEX_HB;
use crate::tokens::{TokenDefinition, TokenKind, TokenSet};

enum FileMode {
	SingleFile,
	MultiFile
}

fn parse_token_sets(store: &mut Figtok, token_sets: HashMap<String, TokenSet>) {
	// Parse all of the tokens and token_sets recursively.
	for (slug, data) in token_sets {
		// Prefix will hold individual portions of the property name, if a value is accessible at
		// colors.red.1 then prefix will eventually contain ["colors", "red", "1"] after it has
		// recursed through the JSON.
		let mut prefix: Vec<String> = vec![];

		// Insert a blank token set.
		let _ = &store.token_sets.insert(slug.clone(), Vec::new());

		// Parse the token set
		parse_token_set(store, &slug.to_string(), data, Some(&mut prefix));
	}
}

fn parse_themes(store: &mut Figtok, themes: Vec<serde_json::Value>) {
	// Iterate over all of the theme definitions
	for theme in themes {
		// Get the theme's name
		let theme_name =
			serde_json::from_value::<String>(theme.get("name").unwrap().to_owned()).unwrap();

		// Get the selectedTokenSets property as a serde_json::Value
		let value = theme.get("selectedTokenSets").unwrap().to_owned();
		let token_sets = serde_json::from_value::<HashMap<String, String>>(value).unwrap();

		// Remove any disabled token sets from the HashMap, leaving only "enabled" and "source"
		let enabled_sets: HashMap<String, String> = token_sets
			.into_iter()
			.filter(|(_, v)| v != "disabled")
			.collect();

		// Get the theme name, and then add the list of enabled sets under the theme name to self.themes.
		let _ = &store.themes.insert(theme_name, enabled_sets);
	}
}

/// Recursively iterate through the token JSON, and add the data to self.tokens
fn parse_token_set(
	store: &mut Figtok,
	slug: &String,
	data: HashMap<String, serde_json::Value>,
	maybe_prefix: Option<&mut Vec<String>>,
) {
	let prefix = maybe_prefix.unwrap();

	for (key, value) in data {
		let mut id = prefix.clone();
		id.push(key.clone());

		let kind = value.get("type");
		match kind {
			Some(_) => {
				// If the "type" property is present, we have a token definition
				let mut token: TokenDefinition = serde_json::from_value(value).unwrap();

				// do any transformations to the token data based on its kind
				token = match token.kind {
					TokenKind::Color => {
						// if the token doesn't contain a reference to
						// another token, then convert it to rgb.
						if !REGEX_HB.is_match(&token.value) {
							let rgb = Rgb::from_hex_str(&token.value).unwrap();
							token.value = format!(
								"{}, {}, {}",
								rgb.get_red(),
								rgb.get_green(),
								rgb.get_blue()
							);
						}
						token
					}
					TokenKind::BorderRadius => token,
					TokenKind::BorderWidth => token,
					TokenKind::FontFamily => token,
					TokenKind::FontWeights => token,
					TokenKind::FontSize => token,
					TokenKind::LetterSpacing => token,
					TokenKind::LineHeight => token,
					TokenKind::Opacity => token,
					TokenKind::Sizing => token,
					TokenKind::Spacing => token,
					TokenKind::Other => token,
				};

				token.name = id.join(".");

				let id_parts = vec![
					slug.split("/").collect::<Vec<&str>>().join("."),
					token.name.clone(),
				];
				token.id = id_parts.join(".");

				// Store the token in it's respective token_set, as a KV pair of [token.id, token.name].
				// We can later use this for lookups by id, and serializing tokens under their name (the name property is relative to the theme.)
				let _ = &store.token_sets.entry(slug.to_string()).and_modify(|v| {
					v.push(token.id.clone());
				});

				let _ = &store.tokens.insert(token.id.clone(), token);
			}
			None => {
				// If the "type" property is not present, we have a nested object
				let nested_data: HashMap<String, serde_json::Value> =
					serde_json::from_value(value).unwrap();
				let mut new_prefix = id.clone();

				let _ = parse_token_set(store, slug, nested_data, Some(&mut new_prefix));
			}
		}
	}
}

/// Loads all the tokens from the input directory into memory.
pub fn load(store: &mut Figtok) {
	let mode = match store.entry_path.ends_with(".json") {
		true => FileMode::SingleFile,
		false => FileMode::MultiFile,
	};

	let (token_sets, themes) = match mode {
		FileMode::SingleFile => {
			let data: serde_json::Value = match serde_json::from_str(&read_file(&store.entry_path).unwrap()) {
				Ok(json) => json,
				Err(error) => panic!("Error reading $metdata.json: {}", error),
			};

			let metadata = data.get("$metadata").unwrap();
			let themes: Vec<serde_json::Value> = serde_json::from_value(data.get("$themes").unwrap().to_owned()).unwrap();

			let mut token_sets: HashMap<String, TokenSet> = HashMap::new();

			for slug in serde_json::from_value::<Vec<String>>(metadata.get("tokenSetOrder").unwrap().to_owned()).unwrap() {
				
				let token_set: TokenSet = serde_json::from_value(
					data.get(&slug).unwrap().to_owned()
				).unwrap();

				token_sets.insert(slug.clone(), token_set);
			}

			(token_sets, themes)
		},
		FileMode::MultiFile => {
			// This gives us an HashMap containing the "tokenSetOrder", a Vec<String> with
			// all of the token sets in order, matching their positions in figma tokens UI.
			let metadata: HashMap<String, Vec<String>> = match serde_json::from_str(&read_file(&format!("{}/$metadata.json", store.entry_path)).unwrap()) {
				Ok(json) => json,
				Err(error) => panic!("Error reading $metdata.json: {}", error),
			};

			let themes: Vec<serde_json::Value> =
				match serde_json::from_str(&read_file(&format!("{}/$themes.json", store.entry_path)).unwrap()) {
					Ok(themes) => themes,
					Err(error) => panic!("Error loaded themes: {}", error),
				};
			

			let mut token_sets: HashMap<String, TokenSet> = HashMap::new();

			for slug in metadata.get("tokenSetOrder").unwrap() {
				let data: TokenSet = match read_file(&format!("./tokens/{}.json", &slug)) {
					Ok(file) => match serde_json::from_str(&file) {
						Ok(data) => data,
						Err(error) => panic!("Error parsing token set: {}", error),
					},
					Err(error) => panic!("Problem opening the file: {:?}", error),
				};

				token_sets.insert(slug.clone(), data);
			}

			(token_sets, themes)
		},
	};

	parse_token_sets(store, token_sets);
	parse_themes(store, themes);
}
