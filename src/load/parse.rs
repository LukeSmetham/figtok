use std::collections::{HashMap};
use serde::de::DeserializeOwned;

use crate::Figtok;
use crate::tokens::{TokenDefinition, TokenKind, Token, ShadowValue};

pub fn parse_themes(ctx: &mut Figtok, themes: Vec<serde_json::Value>) {
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

		// Get the theme name, and then add the list of enabled sets under the theme name to ctx.
		ctx.themes.insert(theme_name, enabled_sets);
	}
}

pub fn parse_token_sets(ctx: &mut Figtok, token_sets: HashMap<String, HashMap<String, serde_json::Value>>) {
	// Parse all of the tokens and token_sets recursively.
	for (slug, data) in token_sets {
		// Prefix will hold individual portions of the property name, if a value is accessible at
		// colors.red.1 then prefix will eventually contain ["colors", "red", "1"] after it has
		// recursed through the JSON.
		let mut prefix: Vec<String> = vec![];

		// Insert a blank token set.
		let _ = &ctx.token_sets.insert(slug.clone(), Vec::new());

		// Parse the token set
		parse_token_set(ctx, &slug.to_string(), data, Some(&mut prefix));
	}
}

/// Recursively iterate through the token JSON, and add the data to self.tokens
fn parse_token_set(
	ctx: &mut Figtok,
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
			// If the "type" property is present, we have a token definition
			Some(k) => {
				let token_type: TokenKind = serde_json::from_value(k.clone()).unwrap();
				// do any transformations to the token data based on its kind
				let token = match token_type {
					TokenKind::BoxShadow => Token::Shadow(create_token::<ShadowValue>(id, slug, value)),
					TokenKind::Composition | TokenKind::Typography => Token::Composition(create_token::<serde_json::Value>(id, slug, value)),
					_ => Token::Standard(create_token::<String>(id, slug, value)),
				};


				// Store the token in it's respective token_set, as a KV pair of [token.id, token.name].
				// We can later use this for lookups by id, and serializing tokens under their name (the name property is relative to the theme.)
				let token_id = token.id();

				ctx.token_sets.entry(slug.to_string()).and_modify(|v| {
					v.push(token_id.clone());
				});

				ctx.tokens.insert(token_id, token);
			}
			None => {
				// If the "type" (`kind`) property is not present, we have a nested object
				let nested_data: HashMap<String, serde_json::Value> = serde_json::from_value(value).unwrap();

				let mut new_prefix = id.clone();

				parse_token_set(ctx, slug, nested_data, Some(&mut new_prefix));
			}
		}
	}
}

fn create_token<T>(id: Vec<String>, slug: &String, value: serde_json::Value) -> TokenDefinition<T> 
where
	T: DeserializeOwned
{
	let mut token: TokenDefinition<T> = serde_json::from_value(value).unwrap();

	token.name = id.join(".");

	let id_parts = vec![
		slug.split("/").collect::<Vec<&str>>().join("."),
		token.name.clone(),
	];
	token.id = id_parts.join(".");

	token
}
