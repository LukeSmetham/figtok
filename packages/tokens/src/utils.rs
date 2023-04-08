use convert_case::{Boundary, Case, Casing};

pub fn css_stringify(s: &String) -> String {
	s.replace(".", "-").with_boundaries(&[Boundary::LowerUpper, Boundary::Underscore, Boundary::Hyphen, Boundary::Space, Boundary::Acronym]).to_case(Case::Kebab)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn dot_notation_to_css_compat() {
		assert_eq!(
			css_stringify(&"global.color.purple.100".to_string()),
			String::from("global-color-purple-100")
		);
		
		assert_eq!(
			css_stringify(&"text.headings.h1.fontSize".to_string()),
			String::from("text-headings-h1-font-size")
		);
	}
}