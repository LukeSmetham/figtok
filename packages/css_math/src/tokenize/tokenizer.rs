use std::iter::Peekable;
use std::str::Chars;

use crate::token::Token;

use super::error::TokenizationError;

pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            chars: input.chars().peekable(),
        }
    }

    fn process_number(&mut self, init: Option<String>) -> Result<Token, TokenizationError> {
        let mut num = String::from(init.unwrap_or(String::from("")));

        while let Some('0'..='9') | Some('.') = self.chars.peek() {
            num.push(self.chars.next().unwrap());
        }

        // Push our number token
        Ok(Token::Number(num))
    }

    fn process_operator(&mut self) -> Result<Token, TokenizationError> {
        let mut op = String::new();

        op.push(self.chars.next().unwrap());

        if op == String::from("-") && matches!(self.chars.peek(), Some('0'..='9')) {
            return self.process_number(Some(op.clone()));
        }

        Ok(Token::Operator(op))
    }

    fn process_unit(&mut self) -> Result<Token, TokenizationError> {
        let mut unit = String::new();

        // This first while loop allows us to capture every character that could
        // potentially be a unit (px|vh|vw|%|rem|...) but will also match "var".
        while let Some('%') | Some('a'..='z') | Some('A'..='Z') = self.chars.peek() {
            unit.push(self.chars.next().unwrap());
        }

        if unit.starts_with("var") {
            return self.process_variable(unit);
        } else {
            Ok(Token::Unit(unit))
        }
    }

    fn process_variable(&mut self, input: String) -> Result<Token, TokenizationError> {
		let mut variable = String::from(input);

		// Check that the next char is (
		if let Some('(') = self.chars.peek() {
			variable.push(self.chars.next().unwrap());

			// Checks for "--"
			while let Some('-') = self.chars.peek() {
				variable.push(self.chars.next().unwrap());

				// Next value should be a-z or "0-9" or the closing parenthesis
				while let Some('a'..='z') | Some('A'..='Z') | Some('0'..='9') | Some(')') = self.chars.peek() {
					variable.push(self.chars.next().unwrap());
				}
			}
		}
		if variable.starts_with("var(--") && variable.ends_with(")") {
			Ok(Token::Variable(variable))
		} else {
			Err(TokenizationError::InvalidVariable(variable))
		}
	}
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, TokenizationError>;

    fn next(&mut self) -> Option<Self::Item> {
        // Implement the logic for tokenizing the input string
        match self.chars.peek() {
            Some(&c) => match c {
                '0'..='9' => Some(self.process_number(None)),
                '%' | 'a'..='z' | 'A'..='Z' => Some(self.process_unit()),
                '+' | '-' | '*' | '/' => Some(self.process_operator()),
                ' ' => {
                    self.chars.next();
                    Some(Ok(Token::Whitespace))
                }
                '(' => {
                    self.chars.next();
                    Some(Ok(Token::LeftParen))
                }
                ')' => {
                    self.chars.next();
                    Some(Ok(Token::RightParen))
                }
                _ => Some(Err(TokenizationError::UnrecognizedCharacter(c))),
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use matches::assert_matches;
	use test_case::test_case;

     #[test_case("5")]
    #[test_case("10")]
    #[test_case("1.12")]
    #[test_case("100")]
    #[test_case("6.2")]
    #[test_case("-60.0001")]
    #[test_case("-1")]
    #[test_case("10.5")]
    fn handles_number(input: &str) {
        let mut tokenizer = Tokenizer::new(input);
        assert_matches!(tokenizer.next().unwrap().unwrap(), Token::Number(_));
    }

    #[test_case("px")]
    #[test_case("%")]
    #[test_case("vh")]
    #[test_case("vw")]
    #[test_case("ch")]
    #[test_case("cw")]
    #[test_case("em")]
    #[test_case("rem")]
    fn handles_unit(input: &str) {
        let mut tokenizer = Tokenizer::new(input);
		
        assert_matches!(tokenizer.next().unwrap().unwrap(), Token::Unit(_));
    }
    
	#[test_case("var(--color)")]
	#[test_case("var(--typescale-1)")]
	#[test_case("var(--ref-purple-100)")]
	#[test_case("var(--gutter)")]
    fn handles_variable(input: &str) {
        let mut tokenizer = Tokenizer::new(input);
		
        assert_matches!(tokenizer.next().unwrap().unwrap(), Token::Variable(_));
    }

	#[test_case("varcolor)" ; "missing hyphens in variable name")]
	#[test_case("var(-color)"  ; "incorrect hyphenation")]
	#[test_case("var(--color"  ; "missing closing paren")]
	fn handles_invalid_variable(input: &str) {
		let mut tokenizer = Tokenizer::new(input);
		assert_matches!(
			tokenizer.next().unwrap(),
			Err(TokenizationError::InvalidVariable(_))
		);
	}

    #[test_case("+" ; "plus")]
    #[test_case("-" ; "subtract")]
    #[test_case("/" ; "divide")]
    #[test_case("*" ; "multiply")]
    fn handles_operator(input : &str) {
        let mut tokenizer = Tokenizer::new(input);
        assert_matches!(tokenizer.next().unwrap().unwrap(), Token::Operator(_));
    }

    #[test]
    fn handles_whitespace() {
        let input = " ";
        let mut tokenizer = Tokenizer::new(input);
        assert_matches!(tokenizer.next().unwrap().unwrap(), Token::Whitespace);
    }

    #[test]
    fn handles_left_paren() {
        let input = "(";
        let mut tokenizer = Tokenizer::new(input);
        assert_matches!(tokenizer.next().unwrap().unwrap(), Token::LeftParen);
    }

	#[test]
    fn handles_right_paren() {
        let input = ")";
        let mut tokenizer = Tokenizer::new(input);
        assert_matches!(tokenizer.next().unwrap().unwrap(), Token::RightParen);
    }

    #[test_case("&" ; "ampersand")]
    #[test_case("@" ; "at")]
    #[test_case("^" ; "chevron")]
    #[test_case("[" ; "bracket left")]
    #[test_case("]" ; "bracket right")]
    fn handles_unrecognized_character(input: &str) {
        let mut tokenizer = Tokenizer::new(input);
        assert_matches!(
            tokenizer.next().unwrap(),
            Err(TokenizationError::UnrecognizedCharacter(_))
        );
    }
}
