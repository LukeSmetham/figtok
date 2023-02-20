use std::{default::Default, fs};
use merge_struct::merge;

use crate::{Figtok, tokens::ReplaceMethod};

use super::{
	Serializer,
};

#[derive(Default)]
pub struct JsonSerializer {}
impl Serializer for JsonSerializer {
	fn serialize(&self, ctx: &Figtok) {
		self.serialize_token_sets(ctx);
		self.serialize_themes(ctx);

		// TODO: Serialize Themes.
		// Need to think of a way to serialize the themes as JSON as they are essentially just collections of sets, i.e. because we can't use references to other files in JSON
	}
}
impl JsonSerializer {
	pub fn new() -> Self {
		JsonSerializer{}
	}

	pub fn serialize_token_sets(&self, ctx: &Figtok) {
		for (set_name, token_set) in &ctx.token_sets {
			let mut value = serde_json::from_str("{}").unwrap();

			for id in token_set {
				let token = &ctx.tokens[id];
				value = merge(&value, &token.to_json(ctx, ReplaceMethod::StaticValues)).unwrap();
			}

			// Now we make sure the output directory exists, and write the CSS file to disk

			// Split the set name by any /'s in case they are nested but remove the
			// last portion as this will be the file name not a directory
            let dir = match set_name.rsplit_once("/") {
                Some((d, _)) => d,
                None => "",
			};

			// Ensure the directories we need exist for token sets
            fs::create_dir_all(vec![ctx.output_path.clone(), dir.to_string()].join("/")).unwrap();
			// Write the json file.
            let _ = fs::write(format!("{}/{}.{}", ctx.output_path, set_name, "json"), value.to_string());
		}
	}

	pub fn serialize_themes(&self, ctx: &Figtok) {
        for (name, sets) in &ctx.themes {
            // Themes must be output to the top level so that the import paths work
            // we can probably work around this, if we want, as things improve.
            let name_parts: Vec<&str> = name.split("/").map(|s| s.trim()).collect();

            let _ = fs::write(
                format!("{}/{}.json", ctx.output_path, name_parts.join("-")),
                serde_json::to_value(sets).unwrap().to_string(),
            );
        }
	}
}
