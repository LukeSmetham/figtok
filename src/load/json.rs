use colors_transform::{Color, Rgb};
use std::collections::HashMap;

use super::{Loader, utils::read_file};
use crate::tokens::{TokenDefinition, TokenKind};
use crate::tokens::helpers::{REGEX_HB};

#[derive(Debug)]
pub struct JsonLoader {
    pub tokens: HashMap<String, TokenDefinition>,
    pub token_sets: HashMap<String, Vec<String>>,
    pub themes: HashMap<String, HashMap<String, String>>,
}
impl Loader for JsonLoader {
	fn new() -> JsonLoader {
        JsonLoader {
            tokens: HashMap::new(),
            token_sets: HashMap::new(),
            themes: HashMap::new(),
        }
    }

	/// Loads all the tokens from the input directory into memory.
    fn load(&mut self, entry_path: &String) {
        self.load_tokens(entry_path);
        self.load_themes(entry_path);
    }

	fn get_tokens(&self) -> &HashMap<String, TokenDefinition> {
		&self.tokens
	}

	fn get_token_sets(&self) -> &HashMap<String, Vec<String>> {
		&self.token_sets
	}

	fn get_themes(&self) -> &HashMap<String, HashMap<String, String>> {
		&self.themes
	}
}
impl JsonLoader {
    /// Recursively iterate through the token JSON, and add the data to self.tokens
    fn parse_token_set(
        &mut self,
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

                    // do any transformations per token kind
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
                    let _ = &self.token_sets.entry(slug.to_string()).and_modify(|v| {
                        v.push(token.id.clone());
                    });

                    let _ = &self.tokens.insert(token.id.clone(), token);
                }
                None => {
                    // If the "type" property is not present, we have a nested object
                    let nested_data: HashMap<String, serde_json::Value> =
                        serde_json::from_value(value).unwrap();
                    let mut new_prefix = id.clone();

                    let _ = &self.parse_token_set(slug, nested_data, Some(&mut new_prefix));
                }
            }
        }
    }

    fn load_tokens(&mut self, path: &String) {
        let metadata_path = &mut path.clone();
        metadata_path.push_str("/$metadata.json");

        // This gives us an HashMap containing the "tokenSetOrder", a Vec<String> with
        // all of the token sets in order, matching their positions in figma tokens UI.
        let metadata: HashMap<String, Vec<String>> =
            match serde_json::from_str(&read_file(metadata_path).unwrap()) {
                Ok(json) => json,
                Err(error) => panic!("Error reading $metdata.json: {}", error),
            };

        // Parse all of the tokens and token_sets recursively.
        for entry in metadata.get("tokenSetOrder") {
            for slug in entry {
                // use the slug to create the path to the relevant JSON file.
                let path = format!("./tokens/{}.json", slug);

                // Read the file as a string and convert to JSON with serde
                let file: HashMap<String, serde_json::Value> = match read_file(&path) {
                    Ok(file) => match serde_json::from_str(&file) {
                        Ok(data) => data,
                        Err(error) => panic!("Error parsing token set: {}", error),
                    },
                    Err(error) => panic!("Problem opening the file: {:?}", error),
                };

                // Prefix will hold individual portions of the property name, if a value is accessible at
                // colors.red.1 then prefix will eventually contain ["colors", "red", "1"] after it has
                // recursed through the JSON.
                let mut prefix: Vec<String> = vec![];

                // Insert a blank token set.
                let _ = &self.token_sets.insert(slug.clone(), Vec::new());

                // Parse the token set
                let _ = &self.parse_token_set(&slug.to_string(), file, Some(&mut prefix));
            }
        }
    }

    fn load_themes(&mut self, path: &String) {
        let themes_path = &mut path.clone();
        themes_path.push_str("/$themes.json");

        // Use themes_path to get the $themes.json file with serde
        let themes: Vec<serde_json::Value> =
            match serde_json::from_str(&read_file(themes_path).unwrap()) {
                Ok(themes) => themes,
                Err(error) => panic!("Error loaded themes: {}", error),
            };

        // Iterate over all of the theme definitions
        for theme in themes {
            // Get the theme's name
            let theme_name = serde_json::from_value::<String>(theme.get("name").unwrap().to_owned()).unwrap();

            // Get the selectedTokenSets property as a serde_json::Value
            let value = theme.get("selectedTokenSets").unwrap().to_owned();
            let token_sets = serde_json::from_value::<HashMap<String, String>>(value).unwrap();

            // Remove any disabled token sets from the HashMap, leaving only "enabled" and "source"
            let enabled_sets: HashMap<String, String> = token_sets
                .into_iter()
                .filter(|(_, v)| v != "disabled")
                .collect();

            // Get the theme name, and then add the list of enabled sets under the theme name to self.themes.
            let _ = &self.themes.insert(theme_name, enabled_sets);
        }
    }
}
