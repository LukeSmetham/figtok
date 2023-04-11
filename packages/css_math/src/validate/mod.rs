use crate::token::Token;

/// Returns true if all tokens in the provided slice are valid.
/// Will return false any of the tokens are not valid css.
pub fn validate(tokens: &[Token]) -> bool {
	//ensures we never have two consecutive unit tokens
	let mut block_unit = true;
	let mut expecting_operand = true;
	let mut paren_count = 0;

	for token in tokens {
		match token {
			Token::Number(_) | Token::Variable(_) => {
				if !expecting_operand {
					return false;
				} 
				expecting_operand = false;
				block_unit = false;
			}
			Token::Unit(_) => {
				if expecting_operand || block_unit {
					return false;
				}

				block_unit = true;
			}
			Token::Operator(_) => {
				if expecting_operand {
					return false
				}

				block_unit = true;
				expecting_operand = true;
			}
			Token::LeftParen => {
				if !expecting_operand {
					return false;
				}

				block_unit = true;
				paren_count += 1;
			}
			Token::RightParen => {
				if expecting_operand || paren_count == 0 {
					return false;
				}

				block_unit = true;
				paren_count -= 1;
			}
			Token::Whitespace => {
				if expecting_operand {
					return false;
				}
			}
		}
	}

	!expecting_operand && paren_count == 0
}

// TODO: Rework these tests. I think it's better to test specific aspects of the CSS Calc spec.
// This ensures things work as expected but also things like checking valid whitespace is not the responsibility of the validator so this separation becomes clearer.
#[cfg(test)]
mod tests {
	use crate::tokenize::tokenize;

	use super::*;

	mod units {
		use super::*;
		
		#[test]
		fn disallow_consecutive_units() {
			let examples = vec![
				"12px px",
				"px 2% rem",
				"rem px 2px"
			];

			for current in examples {
				if let Some(tokens) = tokenize(current) {
					assert!(!validate(&tokens));
				}
			}
		}
	}

	mod operators {
		use super::*;

		#[test]
		fn disallow_consecutive_operators() {
			// This one has a catch, the below string should be invalid
			let invalid = "12px + + 2px";

			if let Some(tokens) = tokenize(invalid) {
				assert!(!validate(&tokens))
			}

			// However, there is an exception - negative numbers
			let valid = "12px + -2px";

			if let Some(tokens) = tokenize(valid) {
				println!("tokens: {:?}", tokens);
				assert!(validate(&tokens))
			} else {
				assert!(false)
			}
		}
	}

	#[test]
	fn checks_validity() {
		let valid_examples: Vec<&str> = vec![
			"75% - 15px",
			"90% / 3 - 10px",
			"(100% - 60px) / 4",
			"2rem + 3vh * 2",
			"(4em * 3) - 60%",
			"30% + 5em",
			"20px * 3 + 10%",
			"50% - 10vw + 5rem",
			"80% - (100px + 10px)",
			"(75% - 2 * 1em) / 2",
			"12em - 6rem",
			"100px / 2 + 25%",
			"2vw * 3 + 2vh",
			"3rem + 10px * 2",
			"20% + 10% - 1em",
			"25px * var(--size) - 10%",
			"1fr - 4em + 2%",
			"80vh / 10 + 5%",
			"60% + 5em - 2rem",
			"1in - var(--size-1) + 10px",
			"100pt / 4 + 5%",
			"4em * 3 - 50%",
			"50% - (5 * 10px)",
			"2cm + 5mm * 2",
			"50% + 20px / 2",
			"50% / 2 + 20px",
			"100% - 50px - 5%",
			"1rem * 2 + 2vh",
			"(50% - 30px) / 5",
			"100px + 10% + 5vw",
			"3em - 50% / 2",
			"4rem + 2vw * 4",
			"1fr - 2em / 2",
			"20px * (50% - 30px)",
			"100vh / 6 + 10%",
			"50% - 20px * 3",
			"75% + 10px / 2",
			"2rem + 50% * 2",
			"20px + 10% * 3",
			"100% / (3 + 2)",
			"100px - 10% + 5vw",
			"2rem * (1em + 1px)",
			"10px + 20px - 5%",
			"50% - (2 * 1em)",
			"75% + 30px / 2",
			"2rem * 50% * 2",
			"100px + 20% / 2",
			"50% - 20px + 1em",
		];

		for current in valid_examples {
			if let Some(tokens) = tokenize(current) {
				assert!(validate(&tokens));
			}
		}
	}

	#[test]
	fn checks_invalidity() {
		let invalid_examples = vec![
			"50% + + 20px",
			"100px * / 2",
			"100% - 50% -",
			"3em 4em var(0r)",
			"1rem + 2vh * / 3",
			"10px / (3em +))",
			"5px + (2 * (6px - 10)",
			"50% - (20px - 5px",
			"100vw / ",
			"2em + (30px * 2",
			"4em * 2 / * 2",
			"2em)) + 50%",
			"(3em * 2) - 50% -",
			"1rem + 2vh 3",
			"(100% - 40px / 3",
			"2rem * 50% 3",
		];

		for current in invalid_examples {
			if let Some(tokens) = tokenize(current) {
				let invalid = !validate(&tokens);
				if !invalid {
					println!("\n\n{}: \n{:?}\n\n", current, tokens);
				}
				assert!(invalid);
			} else {
				assert!(true);
			}
		}
	}
}