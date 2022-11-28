use figtok::Figtok;
use figtok::load::JsonLoader;
use figtok::serialize::CssSerializer;
use clap::Parser;

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

	let mut figtok = match args.format.as_str() {
		"css" => {
			Figtok::<JsonLoader, CssSerializer>::create(&args.entry, &args.output).unwrap()
		},
		x => {
			panic!("Outputting to {} is not yet support", x)
		}
	};

    figtok.load();
    figtok.export();

    println!("Done! Check {} for the built files.", args.output);
}
