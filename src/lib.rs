extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate once_cell;

mod tokens;
pub mod load;
pub mod serialize;

use std::error::Error;
use std::fs;
use std::path::Path;

use load::Loader;
use serialize::Serializer;

pub struct Figtok<T: Loader, U: Serializer> {
	entry_path: String,
	output_path: String,

	pub loader: T,
	pub serializer: U
}
impl <T: Loader, U: Serializer> Figtok<T, U> {
	pub fn create(entry_path: &String, output_path: &String) -> Result<Figtok<T, U>, Box<dyn Error>> {
		// Check output directory exists, and destroy it if truthy so we can clear any existing output files.
		if Path::new(&output_path).is_dir() {
			fs::remove_dir_all(&output_path).unwrap();
		}

		// Now ensure the output_path dir exists.
		fs::create_dir_all(&output_path).unwrap();

		// Check if the input directory exists
		if !Path::new(&entry_path).exists() {
			panic!("No {} directory found, passed as input directory", entry_path);
		}

		Ok(
			Figtok {
				entry_path: entry_path.clone(),
				output_path: output_path.clone(),
				loader: T::new(),
				serializer: U::new()
			}
		)
	}

	pub fn load(&mut self) {
		let _ = &mut self.loader.load(&self.entry_path);
	}
	
	pub fn export(&self) {
		let _ = self.serializer.serialize(&self.loader, self.output_path.to_owned());
	}
}
