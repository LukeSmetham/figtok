use std::path::Path;

use clap::Parser;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod tokens;

mod loader;
use loader::Loader;

mod serialize;
use serialize::CssSerializer;

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

	// Check if the input directory exists
	if !Path::new(&args.dir).is_dir() {
		panic!("No {} directory found, passed as input directory", args.dir);
	}

	let mut loader = Loader::new(&args.dir, &args.out);
	loader.load();

	let serializer = CssSerializer::new(loader);
	let _ = serializer.serialize();

	println!("Done! Check {} for the output CSS", args.out);
}