#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::{fs::{read_to_string}, error::{Error}, collections::HashMap};

#[derive(Serialize, Deserialize, Debug)]
enum TokenKind {
    #[serde(alias = "borderRadius")]
    BorderRadius,
    #[serde(alias = "color")]
    Color,
    #[serde(alias = "fontFamilies")]
    FontFamily,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenDefinition {
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
fn parse_token_set(token_set: HashMap<String, serde_json::Value>) {
    for (key, value) in token_set {
        let kind = value.get("type");

        match kind {
            Some(_) => {
                // Token definition
                let token: TokenDefinition = serde_json::from_value(value).unwrap();
                println!("{:?}", token);
                println!("\n\n");
            }
            None => {
                // Nested object.
                println!("{:?} No Type Key: {:?}", key, kind);
                let new_set: HashMap<String, serde_json::Value> = serde_json::from_value(value).unwrap();
                parse_token_set(new_set);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let string = read_file("/Users/lukesmetham/Projects/smetham.dev/token-parser/tokens/$metadata.json").unwrap();
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
            parse_token_set(token_set);
        }
    }

    Ok(())
}
