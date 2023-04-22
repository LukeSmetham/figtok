/// Tokenizer and validation checks CSS Math statements (i.e. the string between the parens in calc() expressions)
mod token;

mod tokenize;
use tokenize::tokenize;

mod validate;
use validate::validate;

pub fn is_css_math(input: &str) -> bool {
    match tokenize(input) {
        Ok(tokens) => validate(&tokens),
        Err(_) => {
            // println!("[TokenizationError]: {:?}", error);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
	use test_case::test_case;

	#[test_case("var(--typescale-3) * 1.5")]
	fn valid(input: &str) {
		assert_eq!(true, is_css_math(input))
	}

	#[test_case("12px px" ; "consecutive units without a number")]
	#[test_case("2% rem" ; "consecutive units without a number #2")]
	#[test_case("2px rem px" ; "consecutive units without a number #3")]
	#[test_case("15. * 2px" ; "invalid number")]
	fn invalid(input: &str) {
		assert_eq!(false, is_css_math(input))
	}
}
