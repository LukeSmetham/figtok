use std::collections::HashMap;
use colors_transform::{Color, Rgb};
use convert_case::{Case, Casing};

use crate::Figtok;
use crate::tokens::helpers::REGEX_HB;
use crate::tokens::{TokenDefinition, TokenKind, TypographyValue};

pub fn parse_token_sets(ctx: &mut Figtok, token_sets: HashMap<String, HashMap<String, serde_json::Value>>) {
	// Parse all of the tokens and token_sets recursively.
	for (slug, data) in token_sets {
		// Prefix will hold individual portions of the property name, if a value is accessible at
		// colors.red.1 then prefix will eventually contain ["colors", "red", "1"] after it has
		// recursed through the JSON.
		let mut prefix: Vec<String> = vec![];

		// Insert a blank token set.
		let _ = &ctx.add_token_set(slug.clone(), Vec::new());

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
				match token_type {
					TokenKind::BorderRadius => create_token(ctx, id, slug, value),
					TokenKind::BorderWidth => create_token(ctx, id, slug, value),
					TokenKind::BoxShadow => {
						// TokenDefinition {
						// 	name: String::from("UNDEF"),
						// 	value: String::from("UNDEF"),
						// 	kind: TokenKind::BoxShadow,
						// 	id: String::from("UNDEF")
						// }
					},
					TokenKind::Color => create_token(ctx, id, slug, value),
					TokenKind::Composition => {
						// TokenDefinition {
						// 	name: String::from("UNDEF"),
						// 	value: String::from("UNDEF"),
						// 	kind: TokenKind::Composition,
						// 	id: String::from("UNDEF")
						// }
					},
					TokenKind::Dimension => create_token(ctx, id, slug, value),
					TokenKind::FontFamily => create_token(ctx, id, slug, value),
					TokenKind::FontWeights => create_token(ctx, id, slug, value),
					TokenKind::FontSize => create_token(ctx, id, slug, value),
					TokenKind::LetterSpacing => create_token(ctx, id, slug, value),
					TokenKind::LineHeight => create_token(ctx, id, slug, value),
					TokenKind::Opacity => create_token(ctx, id, slug, value),
					TokenKind::Sizing => create_token(ctx, id, slug, value),
					TokenKind::Spacing => create_token(ctx, id, slug, value),
					TokenKind::Typography => {
						// Here we don't know which fields are given by the user, a typography token can be any combination
						// of typographic properties, i.e. anything from just font-size, to font-size and line-height, up to
						// every typography property in one map.

						// Because of this, rather than deserialize into a struct, we use a hashmap, iterate over the entries
						// and push them to a string to construct the token value.
						let def: HashMap<String, String> = serde_json::from_value(value.get("value").unwrap().clone()).unwrap();

						for (k,v) in def {
							println!("{}, {}", k.to_case(Case::Kebab), v);
						}

						// TODO: Rework this - ideally we should keep track of these composition values and create classes that can be applied by the implementing app.

						// Because composition tokens are a little more involved, we construct the TokenDefinition by hand
						//  although we may want to change this if there's a more idiomatic solution.
						// TokenDefinition {
						// 	name: String::from(""),
						// 	value: String::from("UNDEF"),
						// 	kind: TokenKind::Typography,
						// 	id: String::from("")
						// }
					},
					TokenKind::Other => create_token(ctx, id, slug, value),
				};
			}
			None => {
				// If the "type" (`kind`) property is not present, we have a nested object
				let nested_data: HashMap<String, serde_json::Value> =
					serde_json::from_value(value).unwrap();

				let mut new_prefix = id.clone();

				parse_token_set(ctx, slug, nested_data, Some(&mut new_prefix));
			}
		}
	}
}

fn create_token(ctx: &mut Figtok, id: Vec<String>, slug: &String, value: serde_json::Value) {
	let mut token: TokenDefinition = serde_json::from_value(value).unwrap();

	if token.kind == TokenKind::Color {
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
	}

	token.name = id.join(".");

	let id_parts = vec![
		slug.split("/").collect::<Vec<&str>>().join("."),
		token.name.clone(),
	];
	token.id = id_parts.join(".");

	// Store the token in it's respective token_set, as a KV pair of [token.id, token.name].
	// We can later use this for lookups by id, and serializing tokens under their name (the name property is relative to the theme.)
	ctx.token_sets.entry(slug.to_string()).and_modify(|v| {
		v.push(token.id.clone());
	});

	ctx.add_token(token.id.clone(), token);
}

fn create_composition_token(ctx: &mut Figtok) {
	// Here we can loop over each inner key of the composition/typo/shadow token, creating a unique token for each,
	// followed by creating a CompositionTokenDefinition for each that references them all.
}

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

		// Get the theme name, and then add the list of enabled sets under the theme name to self.themes.
		ctx.add_theme(theme_name, enabled_sets);
	}
}