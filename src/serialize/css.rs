use std::{default::Default, collections::HashMap};
use std::fs;

use crate::{
    tokens::{ReplaceMethod, Token},
    Figtok,
};

use super::Serializer;

#[derive(Default)]
pub struct CssSerializer {}
impl Serializer for CssSerializer {
    fn serialize(&self, ctx: &Figtok) {
		if !ctx.themes.is_empty() {
			self.serialize_themes(ctx);
		} else {
			self.serialize_token_sets(ctx);
		}
    }
}
impl CssSerializer {
    pub fn new() -> Self {
        CssSerializer {}
    }

    fn serialize_token_sets(&self, ctx: &Figtok) {
        // Loop over the token sets and create a CSS file for each
        for (set_name, token_set) in &ctx.token_sets {
            // init the string that will hold our css file
            let mut value = String::new();
            let mut classes = String::new();

            for id in token_set {
                let token = &ctx.tokens[id];
                let token_value = &ctx.tokens[id].to_css(ctx, ReplaceMethod::StaticValues, &None);

                match token {
                    Token::Standard(_) | Token::Shadow(_) => {
                        value.push_str(token_value);
                    }
                    Token::Composition(_) => {
                        classes.push_str(token_value);
                    }
                }
            }

			// Write to the output dir 

            // Split the set name by any /'s in case they are nested but remove the
            // last portion as this will be the file name not a directory
            let dir = match set_name.rsplit_once("/") {
                Some((d, _)) => d,
                None => "",
            };
            // Ensure the directories we need exist for token sets
            fs::create_dir_all(vec![ctx.output_path.clone(), dir.to_string()].join("/")).unwrap();
            // Write the css file.
            let _ = fs::write(format!("{}/{}.css", ctx.output_path, set_name), format!(":root{{{}}} {}", value, classes));
        }
    }

    fn serialize_themes(&self, ctx: &Figtok) {
        // Iterate over the themes and create import statements for each included set.
        for (name, sets) in &ctx.themes {
			let mut value = String::new();
			let mut classes = String::new();
			
			let source_sets = sets.into_iter().filter(|(_, v)| v.as_str() == "source").collect::<HashMap<&String, &String>>();
			let enabled_sets = sets.into_iter().filter(|(_, v)| v.as_str() == "enabled").collect::<HashMap<&String, &String>>();

			for (set_name, _) in source_sets {
				let token_set = &ctx.token_sets[set_name];

				for id in token_set {
					let token = &ctx.tokens[id];
					let token_value = &ctx.tokens[id].to_css(ctx, ReplaceMethod::CssVariables, &Some(name.clone()));

					match token {
						Token::Standard(_) | Token::Shadow(_) => {
							value.push_str(token_value);
						}
						Token::Composition(_) => {
							classes.push_str(token_value);
						}
					}
				}
			}
			
			for (set_name, _) in enabled_sets {
				let token_set = &ctx.token_sets[set_name];

				for id in token_set {
					let token = &ctx.tokens[id];
					let token_value = &ctx.tokens[id].to_css(ctx, ReplaceMethod::CssVariables, &Some(name.clone()));

					match token {
						Token::Standard(_) | Token::Shadow(_) => {
							value.push_str(token_value);
						}
						Token::Composition(_) => {
							classes.push_str(token_value);
						}
					}
				}
			}

            // Themes must be output to the top level so that the import paths work
            // we can probably work around this, if we want, as things improve.
            let name_parts: Vec<&str> = name.split("/").map(|s| s.trim()).collect();

            let _ = fs::write(
                format!("{}/{}.css", ctx.output_path, name_parts.join("-")),
                format!(":root{{{}}} {}", value, classes),
            );
        }
    }
}
