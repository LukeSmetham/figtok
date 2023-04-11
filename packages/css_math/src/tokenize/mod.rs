use crate::token::Token;

mod error;
use error::TokenizationError;

/// Tokenize is responsible for taking a CSS Math statement (without the `calc()`) and producing 
/// an Option<Vec<Token>>, preserving the order and value of each token.
/// 
/// The tokenizer is not concerned with syntax validity or correct ordering of tokens - this should
/// purely tokenize the input string ready to pass to the validator.
pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizationError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            // Number
            '0'..='9' | '.' | '-' => {
                let mut num = String::new();

                if let Some('-') = chars.peek() {
                    num.push(chars.next().unwrap());
                }

				if num == '-'.to_string() && matches!(chars.peek(), Some(' ') | None) {
					tokens.push(Token::Operator(num));
				} else {
					while let Some('0'..='9') | Some('.') = chars.peek() {
						num.push(chars.next().unwrap());
					}

					// Push our number token
					tokens.push(Token::Number(num));
				}
            }
            // Var/Unit ("%", "px", "vh", etc. technically this will match "%" or any a-z chars.)
            '%' | 'a'..='z' => {
				let mut output = String::new();

				// This first while loop allows us to capture every character that could
				// potentially be a unit (px|vh|vw|%|rem|...) but will also match "var".
				while let Some('%') | Some('a'..='z') = chars.peek() {
					output.push(chars.next().unwrap());
				}

				if !output.starts_with("var") {
					// If the above ran, and we don't have `var`, we must have a unit.
					tokens.push(Token::Unit(output.clone()));
					continue;
				}
				
                // Check that the next char is (
				if let Some('(') = chars.peek() {
					output.push(chars.next().unwrap());

					// Checks for "--"
					while let Some('-') = chars.peek() {
						output.push(chars.next().unwrap());

						// Next value should be a-z or "0-9" or the closing parenthesis
						while let Some('a'..='z') | Some('0'..='9') | Some(')') = chars.peek() {
							output.push(chars.next().unwrap());
						}
					}
				}

				if output.starts_with("var(--") && output.ends_with(")") {
					tokens.push(Token::Variable(output));
				}
            }
            '+' | '*' | '/' => {
                tokens.push(Token::Operator(chars.next().unwrap().to_string()));
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            ' ' => {
				tokens.push(Token::Whitespace);
                chars.next();
            }
            c => return Err(TokenizationError::UnrecognizedCharacter(c))
        }
    }

	Ok(tokens)
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
