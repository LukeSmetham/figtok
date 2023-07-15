use convert_case::{Boundary, Case, Casing};

pub fn css_stringify(s: &String) -> String {
	let boundaries: [Boundary; 5] = [Boundary::LowerUpper, Boundary::Underscore, Boundary::Hyphen, Boundary::Space, Boundary::Acronym];
	s.replace(".", "-").with_boundaries(&boundaries).to_case(Case::Kebab)
}

#[cfg(test)]
mod tests {
	use super::*;
	use test_case::test_case;

	#[test_case("global.color.purple.100", "global-color-purple-100")]
	#[test_case("text.headings.h1.fontSize", "text-headings-h1-font-size")]
	fn dot_notation_to_css_compat(input: &str, expected: &str) {
		assert_eq!(css_stringify(&input.to_string()), expected.to_string());
	}
}