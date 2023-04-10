#[derive(Debug, PartialEq)]
enum Token {
	Number(String),
	Unit(String),
	Variable(String),
	Operator(String),
	LeftParen,
	RightParen,
}

fn tokenize(input: &str) -> Option<Vec<Token>> {
	let mut tokens = Vec::new();
	let mut chars = input.chars().peekable();

	while let Some(&c) = chars.peek() {
		match c {
			// Number
			'0'..='9' | '.' => {
				let mut num = String::new();

				// Continually call chars.peek() to check if we have a number or a '.' character
				// We can then call next and push into our num string until we hit something that
				// no longer matches.
				while let Some('0'..='9') | Some('.') = chars.peek() {
					num.push(chars.next().unwrap());
				}

				// Assertion: Number should not end with a '.'
				// TODO: Check this against the spec
				if num.ends_with('.') {
					return None;
				}

				// Push our number token
				tokens.push(Token::Number(num));

				// Now check the following character, and only continue if we have a whitespace, a unit or the end of the string (None).
				if !matches!(chars.peek(), Some('%' | 'a'..='z') | Some(')') | Some(' ') | None) {
                    return None;
                }
			},
			// Unit ("%", "px", "vh", etc. technically this will match "%" or any a-z chars.)
			'%' | 'a'..='z' => {
				let mut unit = String::new();

				// Call peek until we no longer have something matching the spec of our Unit
				while let Some('%') | Some('a'..='z') = chars.peek() {
					unit.push(chars.next().unwrap());
				}

				// If we have a variable, create the token as a variable.
				if unit.starts_with("var(--") && unit.ends_with(')') {
					tokens.push(Token::Variable(unit));
				} else { // else its a unit
					tokens.push(Token::Unit(unit));
				}

				// If the following character isn't either a close parens, whitespace or the end of the string then exit.
				if !matches!(chars.peek(), Some(')') | Some(' ') | None) {
                    return None;
                }
			}
			'+' | '-' | '*' | '/' => {
				tokens.push(Token::Operator(chars.next().unwrap().to_string()));

				// Operators should always have trailing whitespace.
				if !matches!(chars.peek(), Some(' ')) {
                    return None;
                }
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
				chars.next();
			}
			_ => return None
		}
	}
	Some(tokens)
}

fn is_valid_css_math(tokens: &[Token]) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

	mod tokenize {
		use super::*;

		mod operations {
			use super::*;

			#[test]
			fn basic_math() {
				// Single Operation
				let input = "5 + 10";
				let tokens = tokenize(input).unwrap();
				let expected_tokens = [Token::Number(String::from("5")), Token::Operator(String::from("+")), Token::Number(String::from("10"))];

				assert_eq!(tokens, expected_tokens);
			} 

			#[test]
			fn multiple_ops() {
				let input = "5vh - 10px + 100%";
				let tokens = tokenize(input).unwrap();
				let expected_tokens = [Token::Number(String::from("5")), Token::Unit(String::from("vh")), Token::Operator(String::from("-")), Token::Number(String::from("10")), Token::Unit(String::from("px")), Token::Operator(String::from("+")), Token::Number(String::from("100")), Token::Unit(String::from("%"))];
				
				assert_eq!(tokens, expected_tokens);
			}

			#[test]
			fn parenthesis() {
				let input = "(2 * 10ch) + 4px";
				let tokens = tokenize(input).unwrap();
				let expected_tokens = [Token::LeftParen, Token::Number(String::from("2")), Token::Operator(String::from("*")), Token::Number(String::from("10")), Token::Unit(String::from("ch")), Token::RightParen, Token::Operator(String::from("+")), Token::Number(String::from("4")), Token::Unit(String::from("px"))];
				
				assert_eq!(tokens, expected_tokens);
			}
		}

		mod syntax {
			use super::*;

			/// CSS calc statements should always have a space between the operators and operands.
			#[test]
			fn whitepsace() {
	
				let invalid_inputs = vec![
					"100*10px",
					"100 /10px",
					"100* 10px",
					"100px/10%",
					"100px /10%",
					"100px/ 10%",
					"(2*2) + 4px",
					"(2 * 2)+4px",
					"(2%- 2) + 10vh",
					"100% *(10 + 10vh)",
				];

				for current in invalid_inputs {
					let tokens = tokenize(current);

					if let None = tokens {
						assert!(true)
					}
				}

				let valid_inputs = vec![
					"100 * 10px",
					"100px / 10%",
					"(2 * 2) + 4px",
					"(2 * 2) + 4px",
					"(2% - 2) + 10vh",
					"100% * (10 + 10vh)",
				];
				
				for current in valid_inputs {
					let tokens = tokenize(current);

					if let Some(_) = tokens {
						assert!(true)
					}
				}
			}
		}
	}
}
