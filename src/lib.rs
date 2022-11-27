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
		// Ensure output directory exists
		// TODO: We should destroy anything existing in the output directory in the event that it exists.
		fs::create_dir_all(&out).unwrap();

		if format != "css" {
			panic!("Outputting your tokens to {} is not yet supported.", format);
		}
	
		// Check if the input directory exists
		if !Path::new(&path).is_dir() {
			panic!("No {} directory found, passed as input directory", path);
		}

		Ok(Figtok{
			path: path.clone(),
			out: out.clone(),
			loader: Loader::new(),
			serializer: CssSerializer::new()
		})
	}

	pub fn load(&mut self) {
		let _ = &mut self.loader.load(&self.path);
	}
	
	pub fn export(&self) {
		let _ = self.serializer.run(&self.loader, self.out.to_owned());
	}
}