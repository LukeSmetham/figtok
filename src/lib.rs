mod helpers;
mod tokens;
mod load;
mod serialize;

use std::error::Error;
use std::fs;
use std::path::Path;
use load::Loader;
use serialize::{CssSerializer, Serializer};

pub struct Figtok {
	path: String,
	out: String,

	pub loader: Loader,
	pub serializer: CssSerializer
}
impl Figtok {
	pub fn create(path: &String, out: &String, format: &String) -> Result<Figtok, Box<dyn Error>> {
		// Check output directory exists, and destroy it if truthy so we can clear any existing output files.
		if Path::new(&out).is_dir() {
			fs::remove_dir(&out).unwrap();
		}

		// Now ensure the out dir exists.
		fs::create_dir_all(&out).unwrap();

		// TODO We only support CSS right now and use it as default, so this check should only trip if the user specifically tries to export with a different format via the CLI.
		if format != "css" {
			panic!("Outputting your tokens to {} is not yet supported.", format);
		}
	
		// Check if the input directory exists
		if !Path::new(&path).is_dir() {
			panic!("No {} directory found, passed as input directory", path);
		}

		let inst = Figtok{
			path: path.clone(),
			out: out.clone(),
			loader: Loader::new(),
			serializer: CssSerializer::new()
		};

		Ok(inst)
	}

	pub fn load(&mut self) {
		let _ = &mut self.loader.load(&self.path);
	}
	
	pub fn export(&self) {
		let _ = self.serializer.run(&self.loader, self.out.to_owned());
	}
}