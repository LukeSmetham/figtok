use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::fs;

use crate::{loader::Loader, tokens::TokenDefinition};

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
                value.push_str(self.serialize_token(token).as_str());
            }
            value.push_str("}");

            let dir = match set_name.rsplit_once("/") {
                Some((d, _)) => d,
                None => "",
            };

            fs::create_dir_all(vec![self.loader.out.clone(), dir.to_string()].join("/")).unwrap();
            let _ = fs::write(format!("{}/{}.css", &self.loader.out, set_name), value);
        }

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
    fn serialize_token(&self, token: &TokenDefinition) -> String {
        let value = self.enrich_token_value(token.value.clone(), false);
        format!("--{}: {};", token.name.replace(".", "-"), value)
    }

    /// Tests if a value is a static value or a reference. If static it's returned as is,
    /// whereas if it's a reference we go and retrieve the token, and either set the value
    /// in place, or replace the handlebar reference string with css variable syntax depending
    /// on the replace_with_value arg.
    fn enrich_token_value(&self, value: String, replace_with_value: bool) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\{(.*)\}").unwrap();
        }

        // Check if the value contains handlebar syntax with a reference to another token.
        if RE.is_match(&value) {
            let captures = RE.captures(&value).unwrap();

            // Get the ref string without the surrounding curly brackets and use it to retrieve the referenced token
            let ref_id = &captures[1];
            let ref_token = &self.loader.tokens.values().find(|t| t.name == ref_id);

            match ref_token {
                Some(t) => {
                    if !replace_with_value {
                        // Replace the reference string with a css variable that points to the other token.
                        let mut value = RE
                            .replace(
                                &value.to_string(),
                                format!("var(--{})", t.name.clone().replace(".", "-")),
                            )
                            .to_string();
                        if !&value.starts_with("rgb") {
                            value = format!("rgb({})", value);
                        }
                        value
                    } else {
                        // replace the reference string with the value of the referenced token statically.
                        RE.replace(&value.to_string(), t.value.clone()).to_string()
                    }
                }
                None => value,
            }
        } else {
            value
        }
    }
}
