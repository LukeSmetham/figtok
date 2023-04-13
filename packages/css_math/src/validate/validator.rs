use crate::token::Token;
use super::error::ValidationError;

pub fn validator(t: &[Token]) -> Result<(), ValidationError> {
	// convert our token slice into a vec of tuples, containing an optional previous token, and the current token in the iterator.
	let tokens: Vec<(Option<&Token>, &Token)> = std::iter::once(None).chain(t.iter().map(Some)).zip(t.iter()).collect();

	// Use a fold on the tokens iterator to count parentheses. Parens should alwyas be closed out so if the value
	// is non-zero then we have an invalid string.
	let parentheses = tokens.iter().fold(0, |count, (_, current)| match current {
		Token::LeftParen => count + 1,
		Token::RightParen => {
			count - 1
		}
		_ => count
	});

	if parentheses != 0 {
		return Err(ValidationError::MismatchedParentheses)
	}

	for (prev, current) in tokens.iter() {
		// validate based on the previous token
		// Most useful for check a Token follows another kind of Token etc.
		match prev {
			Some(Token::Operator(_)) => {
				// Check whitespace follows an operator
				if !matches!(current, Token::Whitespace) {
					return Err(ValidationError::InvalidWhitespace)
				}
			}
			Some(Token::Number(_)) => {
				// Check that only a unit or whitespace follow a number.
				if !matches!(current, Token::Unit(_) | Token::Whitespace) {
					return Err(ValidationError::InvalidWhitespace)
				}
			}
			Some(Token::Unit(_)) => {
				// Check that whitespace follows a unit.
				if !matches!(current, Token::Whitespace) {
					return Err(ValidationError::InvalidWhitespace)
				}
			}
			Some(Token::Variable(_)) => {}
			Some(Token::Whitespace) => {
				// Check that whitespace does not follow another whitespace character.
				if matches!(current, Token::Whitespace) {
					return Err(ValidationError::InvalidWhitespace)
				}
			}
			Some(Token::LeftParen) => {}
			Some(Token::RightParen) => {}
			None => {}
		}

		// Validate based on the current token.
		match current {
			Token::Variable(value) => {
				if !value.starts_with("var(--") || !value.ends_with(")") {
					return Err(ValidationError::InvalidVariable(value.to_owned()))
				}
			}
			_ => {}
		}
	}

	Ok(())
}

#[cfg(test)]
mod test {
	use crate::token::Token;
    use super::*;

    use matches::assert_matches;

	mod parentheses {
		use super::*;
		use test_case::test_case;

		#[test_case(vec![Token::LeftParen])]
		#[test_case(vec![Token::RightParen])]
		#[test_case(vec![Token::LeftParen, Token::Number(String::from("5")), Token::Unit(String::from("px")), Token::Operator(String::from("*")), Token::Number(String::from("10"))])]
		fn invalid(tokens: Vec<Token>) {
			let res = validator(&tokens);
			assert!(res.is_err());
			assert_matches!(res.err().unwrap(), ValidationError::MismatchedParentheses)
		}
	}

	mod whitespace {
		use super::*;
		use test_case::test_case;
		
		#[test_case(vec![Token::Number(String::from("5")), Token::Unit(String::from("px")), Token::Whitespace, Token::Operator(String::from("*")), Token::Whitespace, Token::Number(String::from("10")), Token::Unit(String::from("%"))])]
		fn valid(tokens: Vec<Token>) {
			let res = validator(&tokens);
			assert!(res.is_ok());
		}

		#[test_case(vec![Token::Number(String::from("5")), Token::Unit(String::from("px")), Token::Operator(String::from("*")), Token::Number(String::from("10"))] ; "No whitespace")]
		#[test_case(vec![Token::Number(String::from("5")), Token::Unit(String::from("px")), Token::Operator(String::from("*")), Token::Whitespace, Token::Whitespace, Token::Number(String::from("10"))] ; "double whitepsace")]
		fn invalid(tokens: Vec<Token>) {
			let res = validator(&tokens);
			assert!(res.is_err());
			assert_matches!(res.err().unwrap(), ValidationError::InvalidWhitespace);
		}
	}

	mod variables {
		use super::*;
		use test_case::test_case;

		#[test_case(vec![Token::Variable(String::from("var(--color)"))])]
		fn valid(tokens: Vec<Token>) {
			let res = validator(&tokens);
			assert!(res.is_ok());
		}
		
		#[test_case(vec![Token::Variable(String::from("var(-color)"))] ; "missing hyphen")]
		#[test_case(vec![Token::Variable(String::from("var(--color"))] ; "missing paren")]
		fn invalid(tokens: Vec<Token>) {
			let res = validator(&tokens);
			assert!(res.is_err());
			assert_matches!(res.err().unwrap(), ValidationError::InvalidVariable(_));
		}
	}
}