mod error;
mod validator;

use error::ValidationError;

use crate::token::Token;

// TODO: Validate variable syntax
// TODO: Validate whitespace
// TODO: Validate ordering
/// Returns true if all tokens in the provided slice are valid.
/// Will return false any of the tokens are not valid css.
pub(crate) fn validate(tokens: &[Token]) -> Result<(), ValidationError> {
	if !tokens.clone().iter().any(|t| matches!(t, Token::Operator(_))) {
		return Err(ValidationError::NoOperators);
	}
	let mut previous_token: Option<&Token> = None;

	//ensures we never have two consecutive unit tokens
	let mut expecting_operand = true;
	let mut paren_count = 0;

	for token in tokens {
		match token {
			Token::Number(value) | Token::Variable(value) => {
				if value.ends_with(".") {
					return Err(ValidationError::InvalidNumber(value.to_owned()))
				}

				if !expecting_operand {
					return Err(ValidationError::InvalidToken);
				} 
				expecting_operand = false;
			}
			Token::Unit(_) => {
				if expecting_operand {
					return Err(ValidationError::InvalidToken);
				}
			}
			Token::Operator(_) => {
				if expecting_operand {
					return Err(ValidationError::InvalidToken);
				}

				expecting_operand = true;
			}
			Token::LeftParen => {
				if !expecting_operand {
					return Err(ValidationError::InvalidToken);
				}

				paren_count += 1;
			}
			Token::RightParen => {
				if expecting_operand || paren_count == 0 {
					return Err(ValidationError::InvalidToken);
				}

				paren_count -= 1;
			}
			Token::Whitespace => {
				if let Some(prev) = previous_token {
					match prev {
						Token::Operator(_) | Token::Number(_) | Token::Unit(_) | Token::Variable(_) | Token::RightParen => {},
						_ => return Err(ValidationError::InvalidWhitespace)
					}
				} else {
					return Err(ValidationError::InvalidWhitespace)
				}

 				if expecting_operand {
					return Err(ValidationError::InvalidToken);
				}
			}
		}

		previous_token = Some(token);
	}

	// !expecting_operand && !expecting_whitespace && paren_count == 0
	Ok(())
}

// TODO: Rework these tests. I think it's better to test specific aspects of the CSS Calc spec.
// This ensures things work as expected but also things like checking valid whitespace is not the responsibility of the validator so this separation becomes clearer.
#[cfg(test)]
mod tests {
	use crate::tokenize::tokenize;

	use super::*;

	mod variables {
		use super::*;

		#[test]
		fn abc() {
			let input = "var(--typescale-3) * 1.5";

			let tokens = tokenize(input).unwrap();
			assert!(validate(&tokens).is_ok())
		}
	}

	mod units {
		use super::*;
		
		#[test]
		fn disallow_consecutive_units() {
			let examples = vec![
				"12px px",
				" 2% rem",
				" 2px rem px"
			];

			for current in examples {
				let tokens = tokenize(current).unwrap();
				assert!(validate(&tokens).is_err())
			}
		}

		#[test]
		fn disallow_unit_without_number() {
			let examples = vec![
				"px + 2px",
				"ch + 10ch",
				"vh + 10%",
				"10% * px",
			];

			for current in examples {
				let tokens = tokenize(current).unwrap();
				assert!(validate(&tokens).is_err())
			}
		}
	}

	mod operators {
		use super::*;

		use test_case::test_case;
		
		#[test_case("px")]
		#[test_case("rem")]
		#[test_case("10px")]
		#[test_case("var(--test)")]
		#[test_case("-100vh")]
		fn errors_if_no_operators_present(input: &str) {
			let tokens = tokenize(&input).unwrap();
			assert!(validate(&tokens).is_err())
		}

		#[test]
		fn disallow_consecutive_operators() {
			// This one has a catch, the below string should be invalid
			let invalid = "12px + + 2px";

			let tokens = tokenize(invalid).unwrap();
			if let Err(_) = validate(&tokens) {
				assert!(true)
			}

			// However, there is an exception - negative numbers
			let valid = "12px + -2px";

			let tokens = tokenize(valid).unwrap();
			assert!(validate(&tokens).is_ok())
		}

		#[test]
		fn disallow_operators_without_leading_number() {
			let examples = vec![
				"+ 2px",
				"- -12%",
				"/ 2vw",
				" * 10rem",
				"+ var(--test)",
				" - var(--gutter) * 2",
				"* var(--test) + 10px",
				" / var(--gutter) * 2px",
			];

			for current in examples {
				let tokens = tokenize(current).unwrap();
				assert!(validate(&tokens).is_err())
			}
		}

		#[test]
		fn disallow_operators_without_trailing_number() {
			// there's a catch here again, negative numbers are okay to come first in the string

			let examples = vec![
				"2px + ",
				"-12% -",
				"2vw / ",
				"10rem *",
				"var(--test) + ",
				"var(--gutter) * 2 -",
				"var(--test) + 10px * ",
				"var(--gutter) * 2px /",
			];

			for current in examples {
				let tokens = tokenize(current).unwrap();
				assert!(validate(&tokens).is_err())
			}
		}
	}

	mod number {
		use matches::assert_matches;

		use super::*;

		#[test]
		fn checks_for_invalid_floats() {
			let examples = vec![
				"15. * 2px",
				"12.px * 10",
				"10.% + 1",
				"1.rem * 10",
			];

			for current in examples {
				let tokens = tokenize(current).unwrap();
				let result = validate(&tokens);
				println!("{:?}", tokens);
				assert_matches!(result, Err(ValidationError::InvalidNumber(_)))
			}
		}
	}
}