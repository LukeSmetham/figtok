use std::{default::Default, fs};
use serde_json::json;
use merge_struct::merge;

use crate::{tokens::TokenDefinition, Figtok};

use super::{
	Serializer,
	utils,
};

#[derive(Default)]
pub struct JsonSerializer {}
impl Serializer for JsonSerializer {
	fn serialize(&self, ctx: &Figtok) {
		self.serialize_token_sets(ctx);

		// TODO: Serialize Themes.
		// Need to think of a way to serialize the themes as JSON as they are essentially just collections of sets, i.e. because we can't use references to other files in JSON
	}
}
impl JsonSerializer {
	pub fn new() -> Self {
		JsonSerializer{}
	}

	pub fn serialize_token_sets(&self, ctx: &Figtok) {
		for (set_name, token_set) in ctx.get_token_sets() {
			let mut value = serde_json::from_str("{}").unwrap();

			for id in token_set {
				let token = &ctx.get_tokens()[id];
				value = merge(&value, &self.serialize_one(ctx, &token)).unwrap();
			}

			// Now we make sure the output directory exists, and write the CSS file to disk

			// Split the set name by any /'s in case they are nested but remove the
			// last portion as this will be the file name not a directory
            let dir = match set_name.rsplit_once("/") {
                Some((d, _)) => d,
                None => "",
			};

			// Ensure the directories we need exist
            fs::create_dir_all(vec![ctx.output_path.clone(), dir.to_string()].join("/")).unwrap();
			// Write the json file.
            let _ = fs::write(format!("{}/{}.{}", ctx.output_path, set_name, "json"), value.to_string());
		}
	}

	fn serialize_one(&self, ctx: &Figtok, token: &TokenDefinition) -> serde_json::Value {
		let mut key_parts = token.name.split(".").collect::<Vec<&str>>();
		key_parts.reverse();

        let value = utils::get_token_value(ctx, token, utils::ReplaceMethod::StaticValues, false);
		
		let mut j = json!(value);
		for key in key_parts {
			j = json!({ key: j })
		};

		j
    }
}
