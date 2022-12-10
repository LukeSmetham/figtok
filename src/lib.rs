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
use serialize::{Serializer, CssSerializer, JsonSerializer};

pub struct Figtok<T: Loader> {
    entry_path: String,
    output_path: String,

    pub loader: T,
    pub serializer: Box<dyn Serializer<T>>,
}

impl <T: Loader + Default> Figtok<T> {
    pub fn create(format: &String, entry_path: &String, output_path: &String) -> Result<Figtok<T>, Box<dyn Error>> {
		let serializer: Box<dyn Serializer<T>> = match format.as_str() {
			"css" => Box::new(CssSerializer::new()),
			"json" => Box::new(JsonSerializer::new()),
			f => panic!("Unsupported output format {}", f)
		};

		let ft = Figtok {
			entry_path: entry_path.clone(),
			output_path: output_path.clone(),
			loader: T::default(),
			serializer,
		};

		let _ = ft.prepare();

        Ok(ft)
    }

	fn prepare(&self) -> Result<(), Box<dyn Error>> {
		// Check output directory exists, and destroy it if truthy so we can clear any existing output files.
        if Path::new(&self.output_path).is_dir() {
            fs::remove_dir_all(&self.output_path)?;
        }

        // Now ensure the output_path dir exists.
        fs::create_dir_all(&self.output_path)?;

        // Check if the input directory exists
        if !Path::new(&self.entry_path).exists() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("No {} directory found, passed as input directory", self.entry_path),
            )));
        };

		Ok(())
	}

    pub fn load(&mut self) {
        let _ = &mut self.loader.load(&self.entry_path);
    }

    pub fn export(&self) {
        let _ = self.serializer.serialize(&self.loader, self.output_path.to_owned());
    }
}
