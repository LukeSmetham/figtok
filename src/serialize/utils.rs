use crate::{
    tokens::helpers::{REGEX_CALC, REGEX_HB},
    tokens::{TokenDefinition, TokenKind}, Figtok,
};

use regex::Captures;

#[derive(Clone, Copy)]
pub enum ReplaceMethod {
	CssVariables,
	StaticValues,
}

fn to_variable_name(name: &String) -> String {
	format!("var(--{})", name.replace(".", "-"))
}

/// Tests if a value is a static value or a reference. If static it's returned as is,
/// whereas if it is a reference we go and retrieve the ref'd token, and return it's value.
pub fn get_token_value(ctx: &Figtok, token: &TokenDefinition, replace_method: ReplaceMethod, nested: bool) -> String {
    // Check if the original_value contains handlebar syntax with a reference to another token.
    let mut value = if REGEX_HB.is_match(&token.value) {
        REGEX_HB
            .replace_all(&token.value, |caps: &Captures| {
                // this will run for each occurrence per string. (i.e. multiple tokens multiplied together)
                // Get the ref string without the surrounding curly brackets and use it to retrieve the referenced token
                let ref_name = &caps[1];

                // Find the token using the ref_name.
                match ctx.tokens.values().find(|t| t.name == ref_name) {
                    Some(t) => {
						// If we find a token
                        // Replace the handlebar ref with a css variable that points to the relevant variable for the referenced token.

						let replacement = match replace_method {
							ReplaceMethod::CssVariables => {
								to_variable_name(&t.name)
							},
							ReplaceMethod::StaticValues => {
								// when returning a static value, we recursively call get_token_value to ensure we have
								// unfurled any tokens that depend on other tokens, and may be indefinitely "nested" in this way.
								get_token_value(ctx, &t, replace_method, true)
							}
						};

                        let mut value_str = REGEX_HB
                            .replace(&caps[0], replacement)
                            .to_string();

						if !nested {
							// If we are not nested in this iteration, check the token.kind value and apply any
							// final transformations. e.g. We convert all colors to rgb/rgba when parsing, so any
							// color token that doesn't already start with RGB should be wrapped with `rgb()`
							value_str = match &token.kind {
								TokenKind::Color => {
									if !token.value.starts_with("rgb") {
										value_str = format!("rgb({})", value_str);
									}
									value_str
								},
								_ => value_str
							}
						}

						value_str
                    }
                    None => {
						let replacement = match replace_method {
							ReplaceMethod::CssVariables => {
								to_variable_name(&String::from(ref_name))
							},
							ReplaceMethod::StaticValues => {
								String::from("ERR_NOT_FOUND")
							}
						};

                        let mut value_str = REGEX_HB
                            .replace_all(
                                &token.value.to_string(),
                                replacement,
                            )
                            .to_string();

                        if !nested && !token.value.starts_with("rgb") && token.kind == TokenKind::Color {
                            value_str = format!("rgb({})", value_str);
                        }
                        value_str
                    }
                }
            })
            .to_string()
    } else {
        // If there is no handlebar reference in the string, just return the value as is.
        token.value.clone()
    };

    // We check a regex for a css arithmetic expression and if we have a match,
    // then we wrap the value in calc() so CSS can do the actual calculations for us, 
	// and we still keep the references to token variables alive.
    if REGEX_CALC.is_match(&value) {
        value = format!("calc({})", value);
    };

    value
}
