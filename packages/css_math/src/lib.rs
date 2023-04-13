/// Tokenizer and validation checks CSS Math statements (i.e. the string between the parens in calc() expressions)
mod token;

mod tokenize;
use tokenize::tokenize;

mod validate;
use validate::validate;

pub fn is_css_math(input: &str) -> bool {
	match tokenize(input) {
		Ok(tokens) => match validate(&tokens) {
			Ok(_) => true,
			Err(_) => false,
		}
		Err(_) => false
	}
}