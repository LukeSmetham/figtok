use std::{error::Error, default::Default, fs};
use serde_json::json;
use merge_struct::merge;

use crate::tokens::TokenDefinition;
use crate::load::Loader;

use super::{
	Serializer,
	utils,
};

#[derive(Default)]
pub struct JsonSerializer {}
impl JsonSerializer {
	pub fn new() -> Self {
		JsonSerializer{}
	}

	pub fn serialize_token_sets(&self, loader: &impl Loader, output_path: &String) {
		for (set_name, token_set) in loader.get_token_sets() {
			let mut value = serde_json::from_str("{}").unwrap();

			for id in token_set {
				let token = &loader.get_tokens()[id];
				value = merge(&value, &self.serialize_one(loader, &token)).unwrap();
			}

			// Now we make sure the output directory exists, and write the CSS file to disk

			// Split the set name by any /'s in case they are nested but remove the
			// last portion as this will be the file name not a directory
            let dir = match set_name.rsplit_once("/") {
                Some((d, _)) => d,
                None => "",
			};

			// Ensure the directories we need exist
            fs::create_dir_all(vec![output_path.clone(), dir.to_string()].join("/")).unwrap();
			// Write the json file.
            let _ = fs::write(format!("{}/{}.{}", output_path, set_name, "json"), value.to_string());
		}
	}

	fn serialize_one(&self, loader: &impl Loader, token: &TokenDefinition) -> serde_json::Value {
		let mut key_parts = token.name.split(".").collect::<Vec<&str>>();
		key_parts.reverse();

        let value = utils::get_token_value(loader, token, utils::ReplaceMethod::StaticValues, false);
		
		let mut j = json!(value);
		for key in key_parts {
			j = json!({ key: j })
		};

		j
    }
}
impl <T:Loader> Serializer<T> for JsonSerializer {
	fn serialize(&self, loader: &T, output_path: String) -> Result<(), Box<dyn Error>> {
		self.serialize_token_sets(loader, &output_path);

		// TODO: Serialize Themes.
		Ok(())
	}
}

#[cfg(test)]
mod test {
    use crate::{load::{Loader, JsonLoader}, tokens::{TokenDefinition, TokenKind}};

    use super::{JsonSerializer};

	#[test]
	fn test_serialize_one() {
		let mut loader = JsonLoader::new();
		loader.load(&String::from("./tokens/single_file_test.json"));

		let serializer = JsonSerializer{};
		let token = TokenDefinition {
			name: String::from("ref.purple.1"),
			id: String::from("purple.1"),
			value: String::from("#03001d"),
			kind: TokenKind::Color
		};

		let value = serializer.serialize_one(&loader, &token);
		assert_eq!(value, "{\"ref-purple-1\":\"#03001d\"}");
	}
}
