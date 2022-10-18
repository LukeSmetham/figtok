#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::{fs::{File, read_to_string}, error::{Error}, collections::HashMap};

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

fn read_file(filepath: &str) -> Result<HashMap<String, TokenDefinition>, Box<dyn Error>> {
    let data = read_to_string(filepath)?;
    let j: HashMap<String, TokenDefinition> = serde_json::from_str(&data)?;
    Ok(j)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let j = read_file("/Users/lukesmetham/Projects/smetham.dev/token-parser/tokens/color/clay/dark.json").unwrap();
    println!("{:#?}", j);

    Ok(())
}
