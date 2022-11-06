use crate::{
    loader::Loader,
    tokens::{TokenDefinition, TokenKind},
};

use lazy_static::lazy_static;
use regex::{Captures, Regex};

/// Tests if a value is a static value or a reference. If static it's returned as is,
/// whereas if it is a reference we go and retrieve the ref'd token, and return it's value.
pub fn get_token_value(loader: &Loader, token: &TokenDefinition) -> String {
    lazy_static! {
		// Looks for handlebar syntax and captures the inner value
        static ref REGEX_HB: Regex = Regex::new(r"\{([x[^\{\}]]*)\}.*?").unwrap();
		// Looks for CSS math expressions
		static ref REGEX_CALC: Regex = Regex::new(r"^( )?(var\(--.*\)|[\d\.]+(%|vh|vw|vmin|vmax|em|rem|px|cm|ex|in|mm|pc|pt|ch|q|deg|rad|grad|turn|s|ms|hz|khz))\s[+\-\*/]\s(\-)?(var\(--.*\)|[\d\.]+(%|vh|vw|vmin|vmax|em|rem|px|cm|ex|in|mm|pc|pt|ch|q|deg|rad|grad|turn|s|ms|hz|khz))( )?$").unwrap();
    }

    // Check if the original_value contains handlebar syntax with a reference to another token.
    let mut value = if REGEX_HB.is_match(&token.value) {
        REGEX_HB.replace_all(&token.value, |caps: &Captures| {
            // this will run for each occurrence per string. (i.e. multiple tokens multiplied together)
            // Get the ref string without the surrounding curly brackets and use it to retrieve the referenced token
            let ref_name = &caps[1];

            // Find the token using the ref_name.
            match loader.tokens.values().find(|t| t.name == ref_name) {
                Some(t) => {
                    // If we find a token
                    // Replace the reference string with a css variable that points to the other token.
                    let mut new_value = REGEX_HB
                        .replace(&caps[0], format!("var(--{})", t.name.replace(".", "-")))
                        .to_string();

                    if !token.value.starts_with("rgb") && token.kind == TokenKind::Color {
                        new_value = format!("rgb({})", new_value);
                    }

                    new_value
                }
                None => {
                    let mut new_value = REGEX_HB
                        .replace_all(
                            &token.value.to_string(),
                            format!("var(--{})", ref_name.replace(".", "-")),
                        )
                        .to_string();

                    if !token.value.starts_with("rgb") && token.kind == TokenKind::Color {
                        new_value = format!("rgb({})", new_value);
                    }
                    new_value
                }
            }
        })
        .to_string()
    } else {
        // If there is no handlebar reference in the string, just return the value as is.
        token.value.clone()
    };

    // TODO: This could be temperamental and might need improving upon.
	// We check a regex for a css arithmetic expression and if ew have a match,
	// then we wrap the value in calc() so CSS can do the lifting for us and
	// we keep references alive.
    if REGEX_CALC.is_match(&value)
    {
		value = format!("calc({})", value);
    };

    value
}
