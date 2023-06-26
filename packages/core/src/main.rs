mod log;

use figtok::{Figtok, load::load};
use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The directory containing your tokens.
    #[arg(short, long, default_value = "./tokens")]
    entry: String,

    /// The directory the output should be written to.
    #[arg(short, long, default_value = "./build")]
    output: String,

    /// The format to output the tokens to. Currently only supports CSS.
    #[arg(short, long, default_value = "css")]
    format: String,
}

fn main() {
    let args = Args::parse();

	// Check output directory exists, and destroy it if truthy so we can clear any existing output files.
	if Path::new(&args.output).is_dir() {
		fs::remove_dir_all(&args.output).unwrap();
	}

	// Now ensure the output path dir exists.
	fs::create_dir_all(&args.output).unwrap();

	// Check if the input directory exists
	if !Path::new(&args.entry).exists() {
		panic!("No {} directory found, passed as input directory", &args.entry);
	};

	let (tokens, token_sets, themes) = load(&args.entry);

	let figtok = Figtok::new(tokens, token_sets, themes, &args.output);

	figtok.serialize();

	log!("Done! Check {} for the built files.", figtok.output_path);
}
