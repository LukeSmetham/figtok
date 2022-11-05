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
		fs::create_dir_all(format!("./build/{}", name)).unwrap();
		let _ = fs::write(format!("./build/{}.css", name), value);
	}
}