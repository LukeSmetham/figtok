use once_cell::sync::Lazy;

use regex::{Regex};

/// Stores a Regex to find handlebars syntax ( i.e. {variable.property} )
pub static REGEX_HB: Lazy<Regex> = Lazy::new(|| {
	Regex::new(r"\{([x[^\{\}]]*)\}.*?").unwrap()
});

/// Stores a Regex to find valid CSS arithmetic expressions
pub static REGEX_CALC: Lazy<Regex> = Lazy::new(|| {
	Regex::new(r"^( )?(var\(--.*\)|[\d\.]+(%|vh|vw|vmin|vmax|em|rem|px|cm|ex|in|mm|pc|pt|ch|q|deg|rad|grad|turn|s|ms|hz|khz)?)\s[+\-\*/]\s(\-)?(var\(--.*\)|[\d\.]+(%|vh|vw|vmin|vmax|em|rem|px|cm|ex|in|mm|pc|pt|ch|q|deg|rad|grad|turn|s|ms|hz|khz)?)( )?$").unwrap()
});

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
		];

		for current in test_strings {
			assert!(REGEX_CALC.is_match(current));
		}
	}

	#[test]
	fn reject_invalid_calc_statements() {
		let test_strings = vec![
			"5.5+10.5",
			"5-72",
			"12/11",
			"100*6",
			"5 + 10 + 15",
			"10 - 5 - 5", // multiple operations like this should probably be supported? (Check CSS Spec.) https://css-tricks.com/a-complete-guide-to-calc-in-css/
			"5 *",
			"10 / 5 / 5",
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