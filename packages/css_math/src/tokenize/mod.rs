use crate::token::Token;
use std::{iter::Peekable, str::Chars};

mod error;
use error::TokenizationError;

fn handle_variable_or_unit(chars: &mut Peekable<Chars>) -> Token {
    let mut variable = String::new();

    // This first while loop allows us to capture every character that could
    // potentially be a unit (px|vh|vw|%|rem|...) but will also match "var".
    while let Some('%') | Some('a'..='z') = chars.peek() {
        variable.push(chars.next().unwrap());
    }

    if !variable.starts_with("var") {
        // If the above ran, and we don't have `var`, we must have a unit.
        return Token::Unit(variable);
    }

    // Check that the next char is (
    if let Some('(') = chars.peek() {
        variable.push(chars.next().unwrap());

        // Checks for "--"
        while let Some('-') = chars.peek() {
            variable.push(chars.next().unwrap());

            // Next value should be a-z " 0-9"
            while let Some('a'..='z') | Some('0'..='9') = chars.peek() {
                variable.push(chars.next().unwrap());
            }

            // If the above loops are done, we should have a ')' to close the var() statement.
            if let Some(')') = chars.peek() {
                variable.push(chars.next().unwrap());
            }
        }
    }

    Token::Variable(variable)
}

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

				// Check for a negative sign before the number
                if let Some('-') = chars.peek() {
                    num.push(chars.next().unwrap());
                }

				if num == '-'.to_string() && matches!(chars.peek(), Some(' ') | Some('(') | None) {
					tokens.push(Token::Operator(num));
				} else {
					// Continually call chars.peek() to check if we have a number or a '.' character
					// We can then call next and push into our num string until we hit something that
					// no longer matches.
					while let Some('0'..='9') | Some('.') = chars.peek() {
						num.push(chars.next().unwrap());
					}
	
					if num == '-'.to_string() {
						// We handle the `-` operator here, as it may be a negative number if it is immediately followed by a number.
						if matches!(chars.peek(), Some(' ')) {
							tokens.push(Token::Operator(num));
						} else {
							return Err(TokenizationError::InvalidNegativeOperator(num));
						}
					} else {
						// Push our number token
						tokens.push(Token::Number(num));
					}
				}
            }
            // Var/Unit ("%", "px", "vh", etc. technically this will match "%" or any a-z chars.)
            '%' | 'a'..='z' => {
                let token = handle_variable_or_unit(&mut chars);

                match token {
                    Token::Variable(t) => {
                        tokens.push(Token::Variable(t));
                    }
                    Token::Unit(t) => tokens.push(Token::Unit(t)),
                    t => {
                        return Err(TokenizationError::UnrecognizedToken(t));
                    }
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
                Token::Number(String::from("-10")),
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
