use convert_case::{Case, Casing};

pub fn css_stringify(s: &String) -> String {
	s.replace(".", "-").to_case(Case::Kebab)
}
