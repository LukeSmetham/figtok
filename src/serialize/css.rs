use std::error::Error;
use std::fs;

use crate::{
    loader::Loader,
    tokens::{TokenDefinition}
};

use super::utils;

pub struct CssSerializer {
    loader: Loader,
}
impl CssSerializer {
    pub fn new(loader: Loader) -> CssSerializer {
        CssSerializer { loader: loader }
    }

    /// Iterate over all token sets and themes, creating CSS files for each with valid references to each other.
    /// Themes import the relevant sets individually, and Token Sets are outputted to their own CSS files that
    /// can be imported individually by the user for more granularity, or if they don't use themes.
    pub fn serialize(&self) -> Result<(), Box<dyn Error>> {
        // Loop over the token sets and create a CSS file for each
        for (set_name, token_set) in &self.loader.token_sets {
            let mut value = String::new();
            value.push_str(":root{");
            for id in token_set {
                let token = &self.loader.tokens[id];
                value.push_str(self.serialize_one(token).as_str());
            }
            value.push_str("}");

            let dir = match set_name.rsplit_once("/") {
                Some((d, _)) => d,
                None => "",
            };

            fs::create_dir_all(vec![self.loader.out.clone(), dir.to_string()].join("/")).unwrap();
            let _ = fs::write(format!("{}/{}.css", &self.loader.out, set_name), value);
        }

		// TODO: Here consider keeping a map of slug to relative path for each set so we can use it to build the @import statements regardless of where the files end up.
        // Iterate over the themes and create import statements for each included set.
        for (name, sets) in &self.loader.themes {
            let set_names: Vec<String> = sets.keys().into_iter().map(|key| key.clone()).collect();

            let mut value = String::new();

            for set in set_names {
                value.push_str(format!("@import \"./{}.css\";", set).as_str());
            }

            // Themes must be output to the top level so that the import paths work
            // we can probably work around this if we want as things improve.

            let name_parts: Vec<&str> = name.split("/").map(|s| s.trim()).collect();
            let _ = fs::write(
                format!("{}/{}.css", &self.loader.out, name_parts.join("-")),
                value,
            );
        }

        Ok(())
    }

    /// Take a single TokenDefinition, and serialize it to a CSS Variable string.
    fn serialize_one(&self, token: &TokenDefinition) -> String {
        let value = utils::get_token_value(&self.loader, token);
        format!("--{}: {};", token.name.replace(".", "-"), value)
    }
}
