use std::fs;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod tokens;

mod loader;
use loader::Loader;

fn main() {
	let mut loader = Loader::new("./tokens");
	loader.load().unwrap();

	let items = loader.serialize_themes();

	fs::create_dir_all("./build").unwrap();
	
	for (name, value) in items {
		let name_parts: Vec<&str>  = name.split("/").map(|s| s.trim()).collect();
		let _ = fs::write(format!("./build/{}.css", name_parts.join("-")), value);
	}
}