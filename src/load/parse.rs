use std::collections::HashMap;
use colors_transform::{Color, Rgb};

use crate::Figtok;
use crate::tokens::helpers::REGEX_HB;
use crate::tokens::{TokenDefinition, TokenKind};

pub fn parse_token_sets(store: &mut Figtok, token_sets: HashMap<String, HashMap<String, serde_json::Value>>) {
	// Parse all of the tokens and token_sets recursively.
	for (slug, data) in token_sets {
		// Prefix will hold individual portions of the property name, if a value is accessible at
		// colors.red.1 then prefix will eventually contain ["colors", "red", "1"] after it has
		// recursed through the JSON.
		let mut prefix: Vec<String> = vec![];

		// Insert a blank token set.
		let _ = &store.add_token_set(slug.clone(), Vec::new());

		// Parse the token set
		parse_token_set(store, &slug.to_string(), data, Some(&mut prefix));
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
				store.token_sets.entry(slug.to_string()).and_modify(|v| {
					v.push(token.id.clone());
				});

				store.add_token(token.id.clone(), token);
			}
			None => {
				// If the "type" property is not present, we have a nested object
				let nested_data: HashMap<String, serde_json::Value> =
					serde_json::from_value(value).unwrap();
				let mut new_prefix = id.clone();

				parse_token_set(store, slug, nested_data, Some(&mut new_prefix));
			}
		}
	}
}

pub fn parse_themes(store: &mut Figtok, themes: Vec<serde_json::Value>) {
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
		store.add_theme(theme_name, enabled_sets);
	}
}