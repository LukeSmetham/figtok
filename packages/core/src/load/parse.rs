use std::collections::{HashMap};
use serde::de::DeserializeOwned;
use tokens::{TokenDefinition, TokenKind, Token, ShadowValue, TokenSets, Tokens, Themes, TokenSet};


pub fn parse_themes(source: Vec<serde_json::Value>) -> Themes {
	let mut themes: Themes = HashMap::new();
	// Iterate over all of the theme definitions
	for theme in source {
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
		themes.insert(theme_name, enabled_sets);
	}

	themes
}

pub fn parse_tokens(source: HashMap<String, HashMap<String, serde_json::Value>>) -> (Tokens, TokenSets) {
	let mut tokens: Tokens = HashMap::new();
	let mut token_sets: TokenSets = HashMap::new();
	
	// Parse all of the tokens. All tokens are within their sets in the source files, so we loop
	// over the token sets, create an entry in `token_sets` for each, and then recurse through the 
	// serde_json::Values to create all of the tokens individually, adding them to the `tokens` map
	// and storing their ID in the previously created `token_sets` entry. 
	for (slug, token_set) in source {
		// Take the "slug" and change from slash-separated to dot-separated "selector" syntax so we can
		// use it to help construct the token ids parse_token_set.
		let set_name = slug.split("/").collect::<Vec<&str>>().join(".");

		// Parse the token set

		// Prefix will hold individual portions of the property name, if a token is accessible at
		// colors.red.1 in the original source file then prefix will eventually contain ["colors", "red", "1"] 
		// as parse_token_set recurses.
		let mut prefix: Vec<String> = vec![];

		let set_tokens = parse_token_set(&set_name, token_set, Some(&mut prefix));

		// Create the token set itself as an empty vec in the token_sets HashMap.
		token_sets.insert(
			slug.clone(), 
			TokenSet::new(Vec::with_capacity(set_tokens.len()))
		);

		// All tokens for the set are return in a Vec above. Now we can loop over them,
		// add each token to the tokens map, and store the token id in the set.
		for token in set_tokens {
			token_sets.entry(slug.to_string()).and_modify(|v| {
				v.0.push(token.id());
			});

			tokens.insert(token.id(), token);
		}
	}

	(tokens, token_sets)
}

/// Recursively iterate through the token JSON, and add the data to self.tokens
fn parse_token_set(
	set_name: &String,
	data: HashMap<String, serde_json::Value>,
	maybe_prefix: Option<&mut Vec<String>>,
) -> Vec<Token> {
	let mut tokens = vec![];
	let prefix = maybe_prefix.unwrap();
	
	for (key, value) in data {
		let mut id = prefix.clone();
		id.push(key.clone());

		let kind = value.get("type");

		match kind {
			// If the "type" property is present, we have a token definition
			Some(k) => {
				let token_type: TokenKind = serde_json::from_value(k.clone()).unwrap();
				let token_name = id.join(".");
				let token_id = vec![set_name.clone(), token_name.clone()].join(".");

				// do any transformations to the token data based on its kind
				let token = match token_type {
					TokenKind::BoxShadow => Token::Shadow(create_token::<ShadowValue>(token_id, token_name, value)),
					TokenKind::Composition | TokenKind::Typography => Token::Composition(create_token::<serde_json::Value>(token_id, token_name, value)),
					_ => Token::Standard(create_token::<String>(token_id, token_name, value)),
				};
				
				tokens.push(token);
			}
			// If the "type" (`kind`) property is not present, we have a nested token set
			None => {
				let nested_data: HashMap<String, serde_json::Value> = serde_json::from_value(value).unwrap();
				// We pass a clone of the id array along as the prefix for all proceeding tokens.
				for token in parse_token_set(set_name, nested_data, Some(&mut id.clone())) {
					tokens.push(token)
				}
			}
		}
	}

	tokens
}

fn create_token<T>(id: String, name: String, value: serde_json::Value) -> TokenDefinition<T> 
where
	T: DeserializeOwned
{
	let mut token: TokenDefinition<T> = serde_json::from_value(value).unwrap();

	token.id = id;
	token.name = name;

	token
}
