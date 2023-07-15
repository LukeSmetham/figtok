use once_cell::sync::Lazy;
use regex::Regex;

/// Stores a Regex to find handlebars syntax ( i.e. {variable.property} )
pub static REGEX_HB: Lazy<Regex> = Lazy::new(|| {
	Regex::new(r"\{([x[^\{\}]]*)\}.*?").unwrap()
});

#[cfg(test)]
mod test {
	use super::*;
	use test_case::test_case;

	// Handlebars

	#[test_case("{test}")]
	#[test_case("{ref.purple.1}")]
	#[test_case("{my.very.deeply.nested.value}")]
	#[test_case("background-color: {ref.blue.1}")]
	fn captures_handlebars_refs(input: &str) {
		assert!(REGEX_HB.is_match(input));
	}
	
	#[test_case("{ref.pink.0")]
	#[test_case("radii.card}")]
	#[test_case("borderWidth.1}{")]
	fn reject_invalid_handlebars_refs(input: &str) {
		assert!(!REGEX_HB.is_match(input));
	}
}