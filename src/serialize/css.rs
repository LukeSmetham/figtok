use std::default::Default;
use std::fs;

use convert_case::{Case, Casing};

use crate::{
    tokens::{TokenDefinition}, 
	Figtok
};

use super::{
	Serializer,
	utils
};

#[derive(Default)]
pub struct CssSerializer {}
impl Serializer for CssSerializer {
	/// Iterate over all token sets and themes, creating CSS files for each with valid references to each other.
    /// Themes import the relevant sets individually, and Token Sets are outputted to their own CSS files that
    /// can be imported individually by the user for more granularity, or if they don't use themes.
    fn serialize(&self, ctx: &Figtok) {

        self.serialize_token_sets(ctx);

		// Themes are not just collections of tokens, but collection of sets. 
		// We already output each set as a CSS file above, so all we need are
		// @import statements. 
		// 
		// However, for more complex setups in the future,
		// or for things like composition tokens, we may want a 
		// way to also write classes, or namespace variables 
		// via class name/id inside the themes root css file.
		self.serialize_themes(ctx);
    }
}
impl CssSerializer {
	pub fn new() -> Self {
        CssSerializer {}
    }

	fn serialize_token_sets(&self, ctx: &Figtok) {
		// Loop over the token sets and create a CSS file for each
        for (set_name, token_set) in ctx.get_token_sets() {
			// init the string that will hold our css file
            let mut value = String::new();

			// add the opening line
            value.push_str(":root{");

            for id in token_set { // serialize each token to a CSS String and add it to value
                let token = &ctx.get_tokens()[id];
                value.push_str(self.serialize_one(ctx, &token).as_str());
            }

			// add the final curly bracket
            value.push_str("}");

			// Now we make sure the output directory exists, and write the CSS file to disk

			// Split the set name by any /'s in case they are nested but remove the
			// last portion as this will be the file name not a directory
            let dir = match set_name.rsplit_once("/") {
                Some((d, _)) => d,
                None => "",
			};

			// Ensure the directories we need exist
            fs::create_dir_all(vec![ctx.output_path.clone(), dir.to_string()].join("/")).unwrap();
			// Write the css file.
            let _ = fs::write(format!("{}/{}.css", ctx.output_path, set_name), value);
        }
	}

	fn serialize_themes(&self, ctx: &Figtok) {
		// TODO: Here consider keeping a map of slug to relative path for each set so we can use it to build the @import statements regardless of where the files end up.
        // Iterate over the themes and create import statements for each included set.
        for (name, sets) in ctx.get_themes() {
            let set_names: Vec<String> = sets.keys().into_iter().map(|key| key.clone()).collect();

            let mut value = String::new();

            for set in set_names {
                value.push_str(format!("@import \"./{}.css\";", set).as_str());
            }

			// TODO: We may want to eventually handle some theme-specific css here too like classes, namespaced styles etc.

            // Themes must be output to the top level so that the import paths work
            // we can probably work around this if we want as things improve.
            let name_parts: Vec<&str> = name.split("/").map(|s| s.trim()).collect();

            let _ = fs::write(
                format!("{}/{}.css", ctx.output_path, name_parts.join("-")),
                value,
            );
        }
	}

	/// Take a single TokenDefinition, and serialize it to a CSS string. This function will also follow any tokens containing a reference
	/// and enrich the value to use the var() syntax to keep the relationship between values alive once serialized to CSS.
    fn serialize_one(&self, ctx: &Figtok, token: &TokenDefinition) -> String {
        let value = utils::get_token_value(ctx, token, utils::ReplaceMethod::CssVariables, false);
        format!("--{}: {};", token.name.replace(".", "-").to_case(Case::Kebab), value)
    }
}
