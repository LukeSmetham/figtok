use colors_transform::{Color, Rgb};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::tokens::{TokenDefinition, TokenKind};
use crate::helpers::{REGEX_HB};

fn read_file(filepath: &String) -> Result<String, Box<dyn Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}

#[derive(Debug)]
pub struct Loader {
    pub path: String,
    pub out: String,
    pub tokens: HashMap<String, TokenDefinition>,
    pub token_sets: HashMap<String, Vec<String>>,
    pub themes: HashMap<String, HashMap<String, String>>,
}
impl Loader {
    pub fn new(path: &str, out: &str) -> Loader {
        fs::create_dir_all(out).unwrap();
        Loader {
            path: path.to_string(),
            out: out.to_string(),
            tokens: HashMap::new(),
            token_sets: HashMap::new(),
            themes: HashMap::new(),
        }
    }

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
                        TokenKind::FontFamily => token,
                        TokenKind::FontSize => token,
                        TokenKind::LetterSpacing => token,
                        TokenKind::LineHeight => token,
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

    fn load_tokens(&mut self) {
        let metadata_path = &mut self.path.clone();
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

    fn load_themes(&mut self) {
        let themes_path = &mut self.path.clone();
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

    /// Loads all the tokens from the input directory into memory.
    pub fn load(&mut self) {
        self.load_tokens();
        self.load_themes();
    }
}