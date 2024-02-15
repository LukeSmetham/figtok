use std::{default::Default, fs, io};

use crate::{log, Figtok, TokenStore};
use tokens::{Token, TokenSet};

use super::Serializer;

#[derive(Default)]
pub struct CssSerializer {}
impl Serializer for CssSerializer {
    fn serialize(&self, store: &Figtok) {
        if !store.themes.is_empty() {
            self.serialize_themes(store);
        } else {
            self.serialize_token_sets(store);
        }
    }

    fn write_file(&self, file_name: String, content: String) -> io::Result<()> {
        fs::write(format!("{}.css", file_name), content)
    }
}
impl CssSerializer {
    pub fn new() -> Self {
        CssSerializer {}
    }

    fn serialize_token_set(
        &self,
        store: &Figtok,
        token_set: &TokenSet,
        theme_name: &Option<String>,
    ) -> (String, String) {
        let mut variables = String::new();
        let mut styles = String::new();

        for id in token_set {
            let token = store.token(id);
            let token_value =
                &token.serialize(store, tokens::ReplaceMethod::StaticValues, theme_name);

            match token {
                Token::Standard(_) | Token::Shadow(_) => {
                    variables.push_str(token_value);
                }
                Token::Composition(_) => {
                    styles.push_str(token_value);
                }
            }
        }

        (variables, styles)
    }

    pub fn serialize_themes(&self, store: &Figtok) {
        log!("Detected {} themes...", store.themes.len());

        for (name, sets) in &store.themes {
            log!("Generating Theme: {}", name);

            let mut variables = String::new();
            let mut classes = String::new();

            for set_name in sets
                .into_iter()
                .filter(|(_, v)| v.as_str() != "disabled")
                .map(|(k, _)| k)
                .collect::<Vec<&String>>()
            {
                let token_set: &TokenSet = &store.token_sets[set_name];

                let output = self.serialize_token_set(store, token_set, &Some(name.clone()));
                variables.push_str(&output.0);
                classes.push_str(&output.1);
            }

            // Write the css file.
            let name_parts: Vec<&str> = name.split("/").map(|s| s.trim()).collect();
            let file_name = [store.output_path.to_string(), name_parts.join("-")].join("/");

            let _ = self.write_file(file_name, format!(":root{{{}}}\n{}", variables, classes));
        }
    }

    pub fn serialize_token_sets(&self, store: &Figtok) {
        log!("Detected {} token sets...", store.token_sets.len());

        // create a .css file for every token set
        for (set_name, token_set) in &store.token_sets {
            log!("Generating Token Set: {}", set_name);

            let (variables, styles) = self.serialize_token_set(store, token_set, &None);

            // Split the set name by any /'s in case they are nested but remove the
            // last portion as this will be the file name not a directory
            let dir = if let Some((d, _)) = set_name.rsplit_once("/") {
                d
            } else {
                ""
            };

            // Ensure the directories we need exist for the token set
            fs::create_dir_all(vec![store.output_path.clone(), dir.to_string()].join("/")).unwrap();

            // Write the css file.
            let file_name = [store.output_path.to_string(), set_name.to_string()].join("/");
            let _ = self.write_file(file_name, format!(":root{{{}}}\n{}", variables, styles));
        }
    }
}
