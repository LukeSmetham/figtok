use once_cell::sync::Lazy;
use regex::{Regex};

/// Stores a Regex to find handlebars syntax ( i.e. {variable.property} )
pub static REGEX_HB: Lazy<Regex> = Lazy::new(|| {
	Regex::new(r"\{([x[^\{\}]]*)\}.*?").unwrap()
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
}