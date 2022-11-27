use clap::Parser;
use figtok::Figtok;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// The directory containing your tokens.
   #[arg(short, long, default_value = "./tokens")]
   dir: String,
   
   /// The directory the output should be written to.
   #[arg(short, long, default_value = "./build")]
   out: String,
   
	/// The format to output the tokens to. Currently only supports CSS.
	#[arg(short, long, default_value = "css")]
	format: String,
}

fn main() {
	let args = Args::parse();

	let mut figtok = Figtok::create(&args.dir, &args.out, &args.format).unwrap();

	figtok.load();
	figtok.export();

	println!("Done! Check {} for the output CSS", args.out);
}