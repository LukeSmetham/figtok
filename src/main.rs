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

	let mut loader = Loader::new(&args.dir, &args.out);
	loader.load().unwrap();

	let _ = loader.serialize();
}