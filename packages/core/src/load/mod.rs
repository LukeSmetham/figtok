use std::collections::HashMap;
use std::path::Path;
use serde_json::Value;
use figtok_tokens::{TokenSets, Themes, Tokens};

mod parse;
use parse::{parse_themes, parse_tokens};

mod utils;
use utils::read_file;

/// Figma Token Studio gives us two options, either one big JSON file with all the tokens in,
/// or multiple JSON files within a directory.
#[derive(Debug, PartialEq, Eq)]
enum FileMode {
    SingleFile,
    MultiFile,
}

/// Loads all the tokens from the input directory into memory.
pub fn load(entry_path: &String) -> (Tokens, TokenSets, Themes) {
    let mode = get_file_mode(&entry_path);

    // Load in the raw data using serde, either from a single json file, or by traversing
    // all json files in the directory (entry_path)
    //
	// All tokens are provided within their sets - from loading the JSON files with serde we get
	// token_sets_source, provided as a HashMap where the key is the token set name, and the value
	// is a HashMap<String, serde_json::Value> containing the contents of the file.
	//
	// We also get themes_source, a Vec of serde_json::Value's containing each theme definition from the
	// $themes file
    let (source_token_sets, source_themes) = match mode {
        FileMode::SingleFile => load_from_file(&entry_path),
        FileMode::MultiFile => load_from_dir(&entry_path),
    };

    let (tokens, token_sets) = parse_tokens(source_token_sets);
    let themes = parse_themes(source_themes);

	(tokens, token_sets, themes)
}

fn load_from_file(entry_path: &str) -> (HashMap<String, HashMap<String, Value>>, Vec<Value>) {
    let data: serde_json::Value = match serde_json::from_str(&read_file(&entry_path.to_string()).unwrap()) {
        Ok(json) => json,
        Err(error) => panic!("Error reading $metdata.json: {}", error),
    };

    let metadata = data.get("$metadata").unwrap();
    let themes: Vec<serde_json::Value> =
        serde_json::from_value(data.get("$themes").unwrap().to_owned()).unwrap();

    let mut token_sets: HashMap<String, HashMap<String, serde_json::Value>> = HashMap::new();

    for slug in
        serde_json::from_value::<Vec<String>>(metadata.get("tokenSetOrder").unwrap().to_owned())
            .unwrap()
    {
        let token_set: HashMap<String, serde_json::Value> =
            serde_json::from_value(data.get(&slug).unwrap().to_owned()).unwrap();

        token_sets.insert(slug.clone(), token_set);
    }

    (token_sets, themes)
}

fn load_from_dir(entry_path: &str) -> (HashMap<String, HashMap<String, Value>>, Vec<Value>) {
    // This gives us an HashMap containing the "tokenSetOrder", a Vec<String> with
    // all of the token sets in order, matching their positions in figma tokens UI.
    let metadata: HashMap<String, Vec<String>> = match serde_json::from_str(
        &read_file(&format!("{}/$metadata.json", entry_path)).unwrap(),
    ) {
        Ok(json) => json,
        Err(error) => panic!("Error reading $metadata.json: {}", error),
    };

    let themes: Vec<serde_json::Value> =
        match serde_json::from_str(&read_file(&format!("{}/$themes.json", entry_path)).unwrap()) {
            Ok(themes) => themes,
            Err(error) => panic!("Error loaded themes: {}", error),
        };

    // Init a new map to hold the token sets
    let mut token_sets: HashMap<String, HashMap<String, serde_json::Value>> = HashMap::new();

    // Using the tokenSetOrder array in the metadata file we can construct the path slugs for every json
    // file that contains tokens. Below we read the files in order, and add them to the above HashMap
    // ready to be parsed.
    for slug in metadata.get("tokenSetOrder").unwrap() {
        let data: HashMap<String, serde_json::Value> =
            match read_file(&format!("./tokens/{}.json", &slug)) {
                Ok(file) => match serde_json::from_str(&file) {
                    Ok(data) => data,
                    Err(error) => panic!("Error parsing token set: {}", error),
                },
                Err(error) => panic!("Problem opening the file: {:?}", error),
            };

        token_sets.insert(slug.clone(), data);
    }

    (token_sets, themes)
}

fn get_file_mode(path: &str) -> FileMode {
    let extension = Path::new(path).extension();

    match extension {
        Some(ext) => {
            if ext == "json" {
                FileMode::SingleFile
            } else {
                panic!("Unsupported input file extension: {:?}", ext)
            }
        }
        None => FileMode::MultiFile,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // FileMode
    #[test]
    fn test_get_file_mode() {
        assert_eq!(
            get_file_mode("./tokens/single_file.json"),
            FileMode::SingleFile
        );
        assert_eq!(get_file_mode("./tokens"), FileMode::MultiFile);
    }

    #[test]
    #[should_panic]
    fn test_invalid_single_file_entry() {
        let entry_path = "./tokens/variables.css";

        get_file_mode(entry_path);
    }
}
