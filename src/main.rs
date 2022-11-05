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

	let items = loader.serialize_all();
	println!("Done {:?}", items);

	fs::create_dir_all("./build").unwrap();
	fs::write("./build/output.css", items.get(0).unwrap());
}