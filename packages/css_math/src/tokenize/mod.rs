use crate::token::Token;

mod tokenizer;
use tokenizer::Tokenizer;
mod error;
use error::TokenizationError;


pub(crate) fn tokenize(input: &str) -> Result<Vec<Token>, TokenizationError> {
    let tokenizer = Tokenizer::new(input);
    let tokens: Result<Vec<Token>, _> = tokenizer.collect();
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
	use test_case::test_case;

	#[test_case("5 + 10px", vec![Token::Number(String::from("5")),Token::Operator(String::from("+")),Token::Number(String::from("10")),Token::Unit(String::from("px"))] ; "5 + 10px")]
	#[test_case("5vh - 10px + 100%", vec![Token::Number(String::from("5")), Token::Unit(String::from("vh")), Token::Operator(String::from("-")), Token::Number(String::from("10")), Token::Unit(String::from("px")), Token::Operator(String::from("+")), Token::Number(String::from("100")), Token::Unit(String::from("%"))] ; "5vh - 10px + 100%")]
	#[test_case("(2 * 10ch) + 4px", vec![Token::LeftParen,Token::Number(String::from("2")),Token::Operator(String::from("*")),Token::Number(String::from("10")),Token::Unit(String::from("ch")),Token::RightParen,Token::Operator(String::from("+")),Token::Number(String::from("4")),Token::Unit(String::from("px"))] ; "(2 * 10ch) + 4px")]
	#[test_case("10 - -1", vec![Token::Number(String::from("10")),Token::Operator(String::from("-")),Token::Number(String::from("-1"))] ; "10 - -1")]
	#[test_case("(400 / -23) * 1", vec![Token::LeftParen,Token::Number(String::from("400")),Token::Operator(String::from("/")),Token::Number(String::from("-23")),Token::RightParen,Token::Operator(String::from("*")),Token::Number(String::from("1"))] ; "(400 / -23) * 1")]
	fn test_output(input: &str, expected: Vec<Token>) {
		assert_eq!(tokenize(input).unwrap(), expected);
	}
}
