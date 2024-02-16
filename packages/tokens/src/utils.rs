use convert_case::{Boundary, Case, Casing};

/// Converts strings of various casings to a valid KebabCase CSS string that can be used for variable names
/// class-names etc.
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
	#[test_case("myCompositionToken", "my-composition-token")]
	#[test_case("ColorPalette.primaryColor.100", "color-palette-primary-color-100")]
	fn to_css_compatible_string(input: &str, expected: &str) {
		assert_eq!(css_stringify(&input.to_string()), expected.to_string());
	}
}