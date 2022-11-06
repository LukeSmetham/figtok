use crate::{
    loader::Loader,
    tokens::{TokenDefinition, TokenKind},
};

use lazy_static::lazy_static;
use regex::{Captures, Regex};

/// Tests if a value is a static value or a reference. If static it's returned as is,
/// whereas if it's a reference we go and retrieve the token, and update the value.
pub fn get_token_value(loader: &Loader, og_token: &TokenDefinition) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\{([x[^\{\}]]*)\}.*?").unwrap();
    }

    // Check if the original_value contains handlebar syntax with a reference to another token.
    let mut value = if RE.is_match(&og_token.value) {
        RE.replace_all(&og_token.value, |caps: &Captures| { // this will run for each occurrence per string. (i.e. multiple tokens multiplied together)
            // Get the ref string without the surrounding curly brackets and use it to retrieve the referenced token
            let ref_name = &caps[1];

            // Find the token using the ref_name.
            match loader.tokens.values().find(|t| t.name == ref_name) {
                Some(t) => { // If we find a token
                    // Replace the reference string with a css variable that points to the other token.
					let mut new_value = RE
						.replace(&caps[0], format!("var(--{})", t.name.replace(".", "-")))
						.to_string();

					if !og_token.value.starts_with("rgb") && og_token.kind == TokenKind::Color {
						new_value = format!("rgb({})", new_value);
					}

					new_value
                }
                None => {
                    let mut new_value = RE
                        .replace_all(
                            &og_token.value.to_string(),
                            format!("var(--{})", ref_name.replace(".", "-")),
                        )
                        .to_string();

                    if !og_token.value.starts_with("rgb") && og_token.kind == TokenKind::Color {
                        new_value = format!("rgb({})", new_value);
                    }
                    new_value
                }
            }
        })
        .to_string()
    } else {
        // If there is no handlebar reference in the string, just return the value as is.
        og_token.value.clone()
    };

    // TODO: This needs improving.
    // If the value contains math operators, then wrap it in calc()
    if value.contains("*") || value.contains("+") || value.contains("/") {
        value = format!("calc({})", value);
    };

    value
}