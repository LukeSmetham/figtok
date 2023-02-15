use std::collections::HashMap;
use colors_transform::{Color, Rgb};
use convert_case::{Case, Casing};

use crate::Figtok;
use crate::tokens::helpers::REGEX_HB;
use crate::tokens::{TokenDefinition, TokenKind, TypographyValue, CompositionTokenDefinition, CompositionToken};

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
					TokenKind::BoxShadow => todo!(),
					TokenKind::Color => create_token(ctx, id, slug, value),
					TokenKind::Composition => todo!(),
					TokenKind::Dimension => create_token(ctx, id, slug, value),
					TokenKind::FontFamily => create_token(ctx, id, slug, value),
					TokenKind::FontWeights => create_token(ctx, id, slug, value),
					TokenKind::FontSize => create_token(ctx, id, slug, value),
					TokenKind::LetterSpacing => create_token(ctx, id, slug, value),
					TokenKind::LineHeight => create_token(ctx, id, slug, value),
					TokenKind::Opacity => create_token(ctx, id, slug, value),
					TokenKind::Sizing => create_token(ctx, id, slug, value),
					TokenKind::Spacing => create_token(ctx, id, slug, value),
					TokenKind::Typography => todo!(),
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

fn create_composition_token(ctx: &mut Figtok, id: Vec<String>, slug: &String, value: serde_json::Value) {
	let mut token: CompositionTokenDefinition = serde_json::from_value(value).unwrap();

	token.name = id.join(".");

	let id_parts = vec![
		slug.split("/").collect::<Vec<&str>>().join("."),
		token.name.clone(),
	];
	token.id = id_parts.join(".");

	// Here we can loop over each inner key of the composition/typo/shadow token, creating a unique token for each,
	// followed by creating a CompositionTokenDefinition for each that references them all.

	// CompositionTokens are almost identical to a regular token with the exception that the value property of the object is itself an object rather than a string.
	let comp_value: HashMap<String, String> = serde_json::from_value(token.value).unwrap();

	let mut inner_tokens: Vec<String> = Vec::new();

	for (k,v) in comp_value {
		println!("{:?}, {}, {}", token.id, k.to_case(Case::Kebab), v);

		let inner_token = TokenDefinition {
			name: format!("{}.{}", token.name, k),
			id: format!("{}.{}", token.id, k),
			kind: token.kind.clone(),
			value: v
		};

		inner_tokens.push(inner_token.id.clone());

		ctx.add_token(inner_token.id.clone(), inner_token);
	}

	// Store the token in it's respective token_set, as a KV pair of [token.id, token.name].
	// We can later use this for lookups by id, and serializing tokens under their name (the name property is relative to the theme.)
	ctx.token_sets.entry(slug.to_string()).and_modify(|v| {
		v.push(token.id.clone());
	});

	ctx.add_composition_token(token.id.clone(), CompositionToken {
		name: token.name, 
		id: token.id,
		tokens: inner_tokens
	});

	// ! This doesn't quite work although the compiler is happy
	// ! - adding to the token set means we need to do a second lookup when parsing if the first one into "tokens" fails, to see if the value is a compositional token.
	// ! - we have created a token for each "inner token", and referenced them in the CompositionToken, but how do we serialize this? Right now we have no way of turning the inner token called *.*.font-size a class ".*.* { font-size: value }"
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