use std::error::Error;
use crate::tokens::TokenDefinition;
use crate::load::Loader;

use super::{
	Serializer,
	utils,
};

pub struct JsonSerializer {}
impl Serializer for JsonSerializer {
	fn new() -> Self {
		JsonSerializer{}
	}

	fn serialize(&self, loader: &impl Loader, output_path: String) -> Result<(), Box<dyn Error>> {

		Ok(())
	}

    fn serialize_one(&self, loader: &impl Loader, token: &TokenDefinition) -> String {
        let value = utils::get_token_value(loader, token);
        format!("--{}: {};", token.name.replace(".", "-"), value)
    }
}

#[cfg(test)]
mod test {
    use crate::{load::{Loader, JsonLoader}, tokens::{TokenDefinition, TokenKind}};

    use super::{Serializer, JsonSerializer};

	#[test]
	fn test_serialize_one() {
		let mut loader = JsonLoader::new();
		loader.load(&String::from("./tokens/single_file_test.json"));

		let serializer = JsonSerializer{};
		let token = TokenDefinition {
			name: String::from("ref.purple.1"),
			id: String::from("purple.1"),
			value: String::from("#03001d"),
			kind: TokenKind::Color
		};

		let value = serializer.serialize_one(&loader, &token);
		assert_eq!(value, "--ref-purple-1: #03001d;");
	}
}