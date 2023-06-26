extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod load;
mod log;

use std::{fs, io};
use merge_struct::merge;

use serde_json::json;
use tokens::{
	Tokens, 
	TokenSets, 
	Themes, 
	Token,
	ReplaceMethod,
	regex::{REGEX_HB},
	utils::css_stringify,
	TokenStore, TokenSet
};
use regex::Captures;

pub struct Figtok {
    pub output_path: String,
	pub tokens: Tokens,
    pub token_sets: TokenSets,
    pub themes: Themes,
}

impl Figtok {
    pub fn new(tokens: Tokens, token_sets: TokenSets, themes: Themes, output_path: &String) -> Self {
		Figtok {
			output_path: output_path.clone(),
			tokens,
            token_sets,
            themes,
		}
    }

	/// Writes the CSS strings to a file. file_name should not include the extension (.css)
	fn write_to_css_file(&self, file_name: String, variables: String, styles: String) -> io::Result<()> {
		fs::write(
			format!("{}.css", file_name), 
			format!(":root{{{}}}\n{}", variables, styles)
		)
	}

	fn serialize_themes(&self) {
		for (name, sets) in &self.themes {
			log!("Generating Theme: {}", name);

			let mut variables = String::new();
			let mut classes = String::new();

			for set_name in sets.into_iter().filter(|(_, v)| v.as_str() != "disabled").map(|(k, _)| k).collect::<Vec<&String>>() {
				let token_set: &TokenSet  = &self.token_sets[set_name];

				let output = token_set.serialize(self, &Some(name.clone()));
				variables.push_str(&output.0);
				classes.push_str(&output.1);
			}

			let name_parts: Vec<&str> = name.split("/").map(|s| s.trim()).collect();

			// Write the css file.
			let _ = self.write_to_css_file(
				[self.output_path.to_string(), name_parts.join("-")].join("/"),
				variables,
				classes
			);
		}
	}

	fn serialize_token_sets(&self) {
		// create a .css file for every token set
		for (set_name, token_set) in &self.token_sets {
			log!("Generating Token Set: {}", set_name);

			let mut variables = String::new();
			let mut classes = String::new();

			let output = token_set.serialize(self, &None);
			variables.push_str(&output.0);
			classes.push_str(&output.1);

			// Write the file.

			// Split the set name by any /'s in case they are nested but remove the
			// last portion as this will be the file name not a directory
			let dir = if let Some((d, _)) = set_name.rsplit_once("/") {
				d
			} else {
				""
			};

			// Ensure the directories we need exist for the token set
			fs::create_dir_all(vec![self.output_path.clone(), dir.to_string()].join("/")).unwrap();

			// Write the css file.
			let _ = self.write_to_css_file(
				[self.output_path.to_string(), set_name.to_string()].join("/"),
				variables,
				classes
			);
		}
	}

	pub fn serialize(&self) {
		if !self.themes.is_empty() {
			log!("Detected {} themes...", self.themes.len());
			self.serialize_themes();
		} else {
			log!("Detected {} token sets...", self.token_sets.len());
			self.serialize_token_sets();
		}
	}

	pub fn to_json(&self) {
		if !self.themes.is_empty() { 
			log!("Detected {} themes...", self.themes.len());

			for (name, sets) in &self.themes {
				let mut value = json!({});
				log!("Generating Theme: {}", name);

				for set_name in sets.into_iter().filter(|(_, v)| v.as_str() != "disabled").map(|(k, _)| k).collect::<Vec<&String>>() {
					let token_set: &TokenSet  = &self.token_sets[set_name];

					value = merge(&value, &token_set.to_json(self, &Some(name.clone()))).unwrap();
				}

				// Write the css file.
				let name_parts: Vec<&str> = name.split("/").map(|s| s.trim()).collect();
				let _ = fs::write(
					format!("{}.json", [self.output_path.to_string(), name_parts.join("-")].join("/")),
					value.to_string(),
				);
			}
		} else {
			log!("Detected {} token sets...", self.token_sets.len());

			for (set_name, token_set) in &self.token_sets {
				let mut value = serde_json::from_str("{}").unwrap();

				for id in token_set {
					let token = &self.tokens[id];
					value = merge(&value, &token.to_json(self, ReplaceMethod::StaticValues, &None)).unwrap();
				}

				// Now we make sure the output directory exists, and write the CSS file to disk

				// Split the set name by any /'s in case they are nested but remove the
				// last portion as this will be the file name not a directory
				let dir = if let Some((d,_)) = set_name.rsplit_once("/") {
					d
				} else {
					""
				};

				// Ensure the directories we need exist for token sets
				fs::create_dir_all([self.output_path.clone(), dir.to_string()].join("/")).unwrap();
				// Write the json file.
				let _ = fs::write(format!("{}.json", [self.output_path.to_string(), set_name.to_string()].join("/")), value.to_string());
			}
		}
	}
}

impl TokenStore for Figtok {
	fn token(&self, id: &String) -> &Token {
		&self.tokens[id]
	}

	fn tokens(&self, theme: &Option<String>) -> Vec<&tokens::Token> {
		if let Some(key) = theme {
			let active_sets = self.themes.get(key).unwrap();
			active_sets.keys().map(|set_name| &self.token_sets[set_name]).flatten().map(|token_id| &self.tokens[token_id]).collect()
		} else {
			self.tokens.values().map(|t| t).collect::<Vec<&Token>>()
		}
	}

	fn enrich(&self, reference: String, replace_method: ReplaceMethod, theme: &Option<String>) -> String {
		REGEX_HB
			.replace_all(&reference, |caps: &Captures| {
				// Get the reference (dot-notation) from the reference string without the surrounding curly brackets and use it to retrieve the referenced value.
				let name = &caps[1];

				match replace_method {
					// Convert the name of the token referenced in the reference string into a CSS var statement so CSS itself can handle the reference.
					ReplaceMethod::CssVariables => format!("var(--{})", css_stringify(&name.to_string())),
					// Get the value of the referenced token, so we can replace the handlebar ref in the original reference string.
					ReplaceMethod::StaticValues => {
						if let Some(t) = self.tokens(theme).iter().find(|t| t.name() == name) {
							t.value(self, replace_method, true, theme)
						} else {
							// No token with a matching name was found.
							// reference.clone()
							// TODO: Should we panic here instead? Wondering if it\s better to fail and let the user know that there is a token missing...
							// TODO: Returning "BROKEN_REF" is closer to the behavior with ReplaceMethod:CssVariables as if the ref is broken, the css will still be output, but won't work in practice.
							String::from("BROKEN_REF")
						}
					}
				}
			})
			.to_string()
	}
}
