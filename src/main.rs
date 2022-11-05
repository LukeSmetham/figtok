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
	println!("loader: {:?}", loader);
}