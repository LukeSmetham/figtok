use crate::token::Token;

mod tokenizer;
use tokenizer::Tokenizer;
mod error;
use error::TokenizationError;


pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizationError> {
    let tokenizer = Tokenizer::new(input);
    let tokens: Result<Vec<Token>, _> = tokenizer.collect();
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    mod operations {
        use super::*;

        #[test]
        fn single_op() {
            // Single Operation
            let input = "5 + 10px";
            let tokens = tokenize(input).unwrap();
            let expected_tokens = [
                Token::Number(String::from("5")),
				Token::Whitespace,
                Token::Operator(String::from("+")),
				Token::Whitespace,
                Token::Number(String::from("10")),
                Token::Unit(String::from("px")),
            ];

            assert_eq!(tokens, expected_tokens);
        }

        #[test]
        fn multiple_ops() {
            let input = "5vh - 10px + 100%";
            let tokens = tokenize(input).unwrap();
            let expected_tokens = [
                Token::Number(String::from("5")),
                Token::Unit(String::from("vh")),
				Token::Whitespace,
                Token::Operator(String::from("-")),
				Token::Whitespace,
                Token::Number(String::from("10")),
                Token::Unit(String::from("px")),
				Token::Whitespace,
                Token::Operator(String::from("+")),
				Token::Whitespace,
                Token::Number(String::from("100")),
                Token::Unit(String::from("%")),
            ];

            assert_eq!(tokens, expected_tokens);
        }

        #[test]
        fn parenthesis() {
            let input = "(2 * 10ch) + 4px";
            let tokens = tokenize(input).unwrap();
            let expected_tokens = [
                Token::LeftParen,
                Token::Number(String::from("2")),
				Token::Whitespace,
                Token::Operator(String::from("*")),
				Token::Whitespace,
                Token::Number(String::from("10")),
                Token::Unit(String::from("ch")),
                Token::RightParen,
				Token::Whitespace,
                Token::Operator(String::from("+")),
				Token::Whitespace,
                Token::Number(String::from("4")),
                Token::Unit(String::from("px")),
            ];

            assert_eq!(tokens, expected_tokens);
        }
    }

     #[test]
	fn variables() {
		let input = "var(--background)";
		let tokens = tokenize(input).unwrap();

		let expected_tokens = [Token::Variable(String::from("var(--background)"))];
		assert_eq!(tokens, expected_tokens);

		let input = "var(--typescale-base)";
		let tokens = tokenize(input).unwrap();

		let expected_tokens = [Token::Variable(String::from("var(--typescale-base)"))];
		assert_eq!(tokens, expected_tokens);

		let input = "var(--typescale-base) * var(--typescale-1)";
		let tokens = tokenize(input).unwrap();

		let expected_tokens = [
			Token::Variable(String::from("var(--typescale-base)")),
			Token::Whitespace,
			Token::Operator(String::from("*")),
			Token::Whitespace,
			Token::Variable(String::from("var(--typescale-1)")),
		];

		assert_eq!(tokens, expected_tokens);
	}

	#[test]
	fn negative_numbers_with_operators() {
		let input = "-1 - 1";
		let expected_tokens = vec![
			Token::Number(String::from("-1")),
			Token::Whitespace,
			Token::Operator(String::from("-")),
			Token::Whitespace,
			Token::Number(String::from("1")),
		];

		assert_eq!(tokenize(input).unwrap(), expected_tokens);
		
		let input = "10 - -1";
		let expected_tokens = vec![
			Token::Number(String::from("10")),
			Token::Whitespace,
			Token::Operator(String::from("-")),
			Token::Whitespace,
			Token::Number(String::from("-1")),
		];

		assert_eq!(tokenize(input).unwrap(), expected_tokens);
		
		let input = "(400 / -23) * 1";
		let expected_tokens = vec![
			Token::LeftParen,
			Token::Number(String::from("400")),
			Token::Whitespace,
			Token::Operator(String::from("/")),
			Token::Whitespace,
			Token::Number(String::from("-23")),
			Token::RightParen,
			Token::Whitespace,
			Token::Operator(String::from("*")),
			Token::Whitespace,
			Token::Number(String::from("1")),
		];

		assert_eq!(tokenize(input).unwrap(), expected_tokens);
	}
}
