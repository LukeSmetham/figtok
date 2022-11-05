use std::fs;

use clap::Parser;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod tokens;

mod loader;
use loader::Loader;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value = "./tokens")]
   dir: String,

   /// Number of times to greet
   #[arg(short, long, default_value = "css")]
   format: String,

   #[arg(short, long, default_value = "./build")]
   out: String
}

fn main() {
	let args = Args::parse();

	if args.format != "css" {
		panic!("Outputting your tokens to {} is not yet supported.", args.format);
	}

	let mut loader = Loader::new(&args.dir);
	loader.load().unwrap();
	println!("{:?}", loader);
	let items = loader.serialize();

	fs::create_dir_all(&args.out).unwrap();
	
	for (name, value) in items {
		let name_parts: Vec<&str>  = name.split("/").map(|s| s.trim()).collect();
		let _ = fs::write(format!("{}/{}.css", &args.out, name_parts.join("-")), value);
	}
}