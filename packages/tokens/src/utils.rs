use figtok::Figtok;
use regex::{Captures};
use convert_case::{Case, Casing};

use crate::replace_method::{ReplaceMethod};
use crate::regex::{REGEX_HB};

pub fn css_stringify(s: &String) -> String {
	s.replace(".", "-").to_case(Case::Kebab)
}

/// Depending on the provided replace_method, this function will either return a css variable string that points to a token elsewhere in the system,
/// or deeply follow the reference itself by searching existing tokens in ctx.tokens for a name that matches, and returning the unfurled value.
/// Lots of tokens have contain references to other tokens e.g. 
/// "value": "{ref.color.red}"
/// "value": "0px 4px 24px 0px rgba({theme.shadow}, 16%)"
/// 
/// if the provided replace_method is CssVariables, we would get this:
/// "value": "rgb(var(--ref-color-red))"
/// "value": "0px 4px 24px 0px rgba(var(--ref-theme-shadow), 16%)" 
/// 
/// if the replace_method is StaticValues we would get
/// "value": "rgb(255, 0, 0)"
/// "value": "0px 4px 24px 0px rgba(0, 0, 0, 16%)" 
// 
pub fn get_token_reference(ref_value: String, ctx: &Figtok, replace_method: ReplaceMethod, theme: &Option<String>) -> String {
    REGEX_HB
        .replace_all(&ref_value, |caps: &Captures| {
            // Get the reference (dot-notation) from the ref_value string without the surrounding curly brackets and use it to retrieve the referenced value.
            let name = &caps[1];

			match replace_method {
				// Convert the name of the token referenced in the ref_value string into a CSS var statement so CSS itself can handle the reference.
				ReplaceMethod::CssVariables => format!("var(--{})", css_stringify(&name.to_string())),
				// Get the value of the referenced token, so we can replace the handlebar ref in the original ref_value string.
				ReplaceMethod::StaticValues => {
					if let Some(t) = ctx.get_tokens(theme).iter().find(|t| t.name() == name) {
						t.value(ctx, replace_method, true, theme)
					} else {
						// No token with a matching name was found.
						// ref_value.clone()
						String::from("BROKEN_REF")
					}
				}
			}
        })
        .to_string()
}