use std::{
	fs::{read_to_string}, 
	error::{Error}, 
	collections::HashMap
};

use crate::{specs::WorldExt};
use crate::specs::Builder;
use specs::World;

use crate::{TokenComponent, IdentityComponent, ValueComponent};

#[derive(Serialize, Deserialize, Debug)]
pub enum TokenKind {
    #[serde(alias = "borderRadius")]
    BorderRadius,
    #[serde(alias = "color")]
    Color,
    #[serde(alias = "fontFamilies")]
    FontFamily,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenDefinition {
    value: String,
    #[serde(alias = "type")]
    kind: TokenKind,
}

fn read_file(filepath: &str) -> Result<String, Box<dyn Error>> {
    let data = read_to_string(filepath)?;
    Ok(data)
}

/// Parses a token set, provided as a HashMap provided from deserializing the JSON token sets.
/// 
/// On find a token, we create a TokenDefinition and return it.
/// 
/// If we can't find a "type" field on the current element in iteration, we recurse by converting
/// the serde_json::Value to a HashMap<String, serde_json::Value> once again, and calling parse_token_set again.
fn parse_token_set(ecs: &mut World, token_set: HashMap<String, serde_json::Value>) {
    for (key, value) in token_set {
        let kind = value.get("type");

        match kind {
            Some(_) => {
                // Found a Token definition.
                let token: TokenDefinition = serde_json::from_value(value).unwrap();
				ecs
					.create_entity()
					.with(IdentityComponent { id: key })
					.with(TokenComponent{})
					.with(ValueComponent{ value: token.value.to_string(), _current: token.value.to_string() })
					.build();
            }
            None => {
                // Nested object, parse and recurse.
                println!("{:?} No Type Key: {:?}", key, kind);
                let new_set: HashMap<String, serde_json::Value> = serde_json::from_value(value).unwrap();
                parse_token_set(ecs, new_set);
            }
        }
    }
}

pub struct Loader {
	dir_path: String,
}
impl Loader {
	pub fn new(dir_path: String) -> Loader {
		Loader {
			dir_path
		}
	}

	pub fn load(&self, ecs: &mut World) -> Result<(), Box<dyn Error>> {
		let string = read_file(&self.dir_path).unwrap();

		// let serialized: HashMap<String, TokenDefinition> = serde_json::from_str(&data)?;
		let metadata: HashMap<String, Vec<String>> = serde_json::from_str(&string)?;
		
		let mut files: HashMap<String, String> = HashMap::new();

		// This gives us a Vec<String> containing slugs of all available token sets to iterate over.
		for entry in metadata.get("tokenSetOrder") {
			for slug in entry {
				// use the slug to create the path to the relevant JSON file.
				let path = format!("./tokens/{}.json", slug);

				// Read the file as a string, and insert into the files map
				let file = read_file(&path)?;
				files.insert(slug.to_string(), file);
			}
		}

		// We don't get a guarantee of order with a HashMap - so instead, we loop over tokenSetOrder
		// again to process everything in order.
		for entry in metadata.get("tokenSetOrder") {
			for slug in entry {
				let token_set: HashMap<String, serde_json::Value> = serde_json::from_str(&files[slug])?;
				parse_token_set(ecs, token_set);
			}
		};

		Ok(())
	}
}