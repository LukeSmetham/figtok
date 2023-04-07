use once_cell::sync::Lazy;

use regex::{Regex, Captures};
use convert_case::{Case, Casing};

use crate::Figtok;
use super::{ReplaceMethod};

/// Stores a Regex to find handlebars syntax ( i.e. {variable.property} )
pub static REGEX_HB: Lazy<Regex> = Lazy::new(|| {
	Regex::new(r"\{([x[^\{\}]]*)\}.*?").unwrap()
});

/// Stores a Regex to find valid CSS arithmetic expressions
pub static REGEX_CALC: Lazy<Regex> = Lazy::new(|| {
	Regex::new(r"^( )?(var\(--.*\)|[\d\.]+(%|vh|vw|vmin|vmax|em|rem|px|cm|ex|in|mm|pc|pt|ch|q|deg|rad|grad|turn|s|ms|hz|khz)?)\s[+\-\*/]\s(\-)?(var\(--.*\)|[\d\.]+(%|vh|vw|vmin|vmax|em|rem|px|cm|ex|in|mm|pc|pt|ch|q|deg|rad|grad|turn|s|ms|hz|khz)?)( )?$").unwrap()
});

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
pub fn get_token_reference(ref_value: String, ctx: &Figtok, replace_method: super::ReplaceMethod, theme: &Option<String>) -> String {
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

#[cfg(test)]
mod test {
	use super::*;

	// Handlebars

	#[test]
	fn captures_handlebars_refs() {
		let test_strings = vec![
			"{test}",
			"{ref.purple.1}",
			"{my.very.deeply.nested.ref.value}",
			"background-color: {ref.blue.1};"
		];

		for current in test_strings {
			assert!(REGEX_HB.is_match(current));
		}
	}
	
	#[test]
	fn reject_invalid_handlebars_refs() {
		let test_strings = vec![
			"{ref.pink.0",
			"radii.card}",
			"borderWidth.1}{"
		];

		for current in test_strings {
			assert!(!REGEX_HB.is_match(current));
		}
	}

	// CSS Calculation Statements
	// We are essentially testing for valid arithmetic statements as strings, that also support CSS syntax like var(--whatever) and rem/em/vh/ch etc. units.
	// We also test for these without the calc() function syntax, because we can define calculations in token studio like this: `{token} * 2` or `{token.1} * {token.2}`
	// and then add the "calc()" wrapping ourselves before outputting the css.

	#[test]
	fn captures_calc_statements() {
		let test_strings = vec![
			"5 + 10",
			"10 - 5",
			"5 * 10",
			"10 / 5",
			"5.5 + 10.5",
			"10.5 - 5.5",
			"5.5 * 10.5",
			"10.5 / 5.5",
			"5rem + 180deg",
			"10px - 5em",
			"5px * 10vw",
			"10vh / 5px",
			"var(--width) + 10%",
			"10px - var(--width)",
			"var(--width) * 10px",
			"10px / var(--width)",
			"5 + 10 + 15",
			"10 - 5 - 5",
			"10 / 5 / 5",
		];

		for current in test_strings {
			assert!(REGEX_CALC.is_match(current));
		}
	}

	#[test]
	fn reject_invalid_calc_statements() {
		let test_strings = vec![
			"5.5+10.5",
			"5 *",
			"5.5 +",
			"foo + 10",
			"10 - bar",
			"5.5 * foo",
			"10.5 / ",
		];

		for current in test_strings {
			assert!(!REGEX_CALC.is_match(current));
		}
	}
}