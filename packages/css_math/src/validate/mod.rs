use crate::token::Token;

/// Returns true if all tokens in the provided slice are valid.
/// Will return false any of the tokens are not valid css.
pub fn validate(tokens: &[Token]) -> bool {
	let mut expecting_operand = true;
	let mut paren_count = 0;

	for token in tokens {
		match token {
			Token::Number(_) | Token::Variable(_) => {
				if !expecting_operand {
					return false;
				} 
				expecting_operand = false;
			}
			Token::Unit(_) => {
				if expecting_operand {
					return false;
				}
			}
			Token::Operator(_) => {
				if expecting_operand {
					return false
				}

				expecting_operand = true;
			}
			Token::LeftParen => {
				if !expecting_operand {
					return false;
				}
				paren_count += 1;
			}
			Token::RightParen => {
				if expecting_operand || paren_count == 0 {
					return false;
				}
				paren_count -= 1;
			}
		}
	}

	!expecting_operand
}