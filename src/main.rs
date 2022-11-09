use std::path::Path;

use clap::Parser;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate once_cell;

mod helpers;
mod tokens;

mod loader;
use loader::Loader;

mod serialize;
use serialize::CssSerializer;

use crate::serialize::Serializer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// The directory containing your tokens.
   #[arg(short, long, default_value = "./tokens")]
   dir: String,

   /// The format to output the tokens to. Currently only supports CSS.
   #[arg(short, long, default_value = "css")]
   format: String,

   /// The directory the output should be written to.
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
	let _ = serializer.run();

	println!("Done! Check {} for the output CSS", args.out);
}