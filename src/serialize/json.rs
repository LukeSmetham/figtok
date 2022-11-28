use super::{
	Serializer
};

use crate::tokens::TokenDefinition;
use crate::load::JsonLoader;

pub struct JsonSerializer {}
impl Serializer for JsonSerializer {
	fn new() -> Self {
		JsonSerializer{}
	}

	fn serialize() -> Result<(), Box<dyn Error>> {

	}

    fn serialize_one(&self, loader: &impl Loader, token: &TokenDefinition) -> String {
        let value = utils::get_token_value(loader, token);
        format!("--{}: {};", token.name.replace(".", "-").to_case(Case::Kebab), value)
    }
}