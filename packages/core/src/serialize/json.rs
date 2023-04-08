use std::{default::Default, fs};
use merge_struct::merge;

use crate::Figtok;
use tokens::ReplaceMethod;

use super::{
	Serializer,
};

// ! For StaticValues there is an issue, currently with the CSS Variables approach you can include the CSS file for the theme you want to include,
// ! and it will resolve to the correct variables to that theme i.e. background is different if your in the dark or light theme.
// ! But resolving static values on "enabled" sets, we need to take into account which theme is being used. This means that for serializing JSON themes 
// ! we need to create each "enabled" set (not "source" as these are truly static) multiple times (once for each theme that uses the set.)
// ! i.e. dark-syntax-theme / light-syntax-theme / dark-theme / light-theme so that each time, we can make sure we only use the tokens from the correct sets.

#[derive(Default)]
pub struct JsonSerializer {}
impl Serializer for JsonSerializer {
	fn serialize(&self, ctx: &Figtok) {
		if !ctx.themes.is_empty() {
			self.serialize_themes(ctx);
		} else {
			self.serialize_token_sets(ctx);
		}

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
				value = merge(&value, &token.to_json(ctx, ReplaceMethod::StaticValues, &None)).unwrap();
			}

			// Now we make sure the output directory exists, and write the CSS file to disk

			// Split the set name by any /'s in case they are nested but remove the
			// last portion as this will be the file name not a directory
            let dir = if let Some((d,_)) = set_name.rsplit_once("/") {
                d
			} else {
				""
			};

			// Ensure the directories we need exist for token sets
            fs::create_dir_all([ctx.output_path.clone(), dir.to_string()].join("/")).unwrap();
			// Write the json file.
            let _ = fs::write(format!("{}.json", [ctx.output_path.to_string(), set_name.to_string()].join("/")), value.to_string());
		}
	}

	pub fn serialize_themes(&self, ctx: &Figtok) {
        for (name, sets) in &ctx.themes {
			let mut value = serde_json::from_str("{}").unwrap();
			
			let source_sets = sets.into_iter().filter(|(_, v)| v.as_str() == "source").map(|(k, _)| k).collect::<Vec<&String>>();
			let enabled_sets = sets.into_iter().filter(|(_, v)| v.as_str() == "enabled").map(|(k, _)| k).collect::<Vec<&String>>();
			
			for set_name in source_sets {
				let token_set = &ctx.token_sets[set_name];

				for id in token_set {
					let token = &ctx.tokens[id];
					value = merge(&value, &token.to_json(ctx, ReplaceMethod::StaticValues, &Some(name.clone()))).unwrap();
				}
			}
			
			for set_name in enabled_sets {
				let token_set = &ctx.token_sets[set_name];

				for id in token_set {
					let token = &ctx.tokens[id];
					value = merge(&value, &token.to_json(ctx, ReplaceMethod::StaticValues, &Some(name.clone()))).unwrap();
				}
			}
            
			let name_parts: Vec<&str> = name.split("/").map(|s| s.trim()).collect();
            let _ = fs::write(
                format!("{}.json", [ctx.output_path.to_string(), name_parts.join("-")].join("/")),
                serde_json::to_value(value).unwrap().to_string(),
            );
        }
	}
}
