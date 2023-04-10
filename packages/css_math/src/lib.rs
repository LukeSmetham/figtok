#[derive(Debug, PartialEq)]
enum Token {
	Number(String),
	Unit(String),
	Variable(String),
	Operator(String),
}

fn tokenize(input: &str) -> Option<Vec<Token>> {
	let mut tokens = Vec::new();
	let mut chars = input.chars().peekable();

	while let Some(&c) = chars.peek() {
		match c {
			'0'..='9' | '.' => {
				let mut num = String::new();
				while let Some('0'..='9') | Some('.') = chars.peek() {
					num.push(chars.next().unwrap());
				}
				tokens.push(Token::Number(num));
			},
			'%' | 'a'..='z' => {
				let mut unit = String::new();
				while let Some('%') | Some('a'..='z') = chars.peek() {
					unit.push(chars.next().unwrap());
				}

				if unit.starts_with("var(--") && unit.ends_with(')') {
					tokens.push(Token::Variable(unit));
				} else {
					tokens.push(Token::Unit(unit));
				}
			}
			'+' | '-' | '*' | '/' => {
				tokens.push(Token::Operator(chars.next().unwrap().to_string()));
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
		}
	}

	!expecting_operand
}

#[cfg(test)]
mod tests {
    use super::*;

	mod tokenize {
		use super::*;

		#[test]
		fn tokenizes_basic_math() {
			// Single Operation
			let mut input = "5 + 10";
			let mut tokens = tokenize(input).unwrap();
			let expected_tokens = [Token::Number(String::from("5")), Token::Operator(String::from("+")), Token::Number(String::from("10"))];

			assert_eq!(tokens, expected_tokens);
			
			// Multiple Operations
			input = "5vh - 10px + 100%";
			tokens = tokenize(input).unwrap();
			let expected_tokens = [Token::Number(String::from("5")), Token::Unit(String::from("vh")), Token::Operator(String::from("-")), Token::Number(String::from("10")), Token::Unit(String::from("px")), Token::Operator(String::from("+")), Token::Number(String::from("100")), Token::Unit(String::from("%"))];
			
			assert_eq!(tokens, expected_tokens);
		} 
	}
}
