use std::iter::Peekable;
use std::str::Chars;

use crate::token::Token;

use super::error::TokenizationError;

pub struct Tokenizer<'a> {
	chars: Peekable<Chars<'a>>
}

impl <'a> Tokenizer<'a> {
	pub(crate) fn new(input: &'a str) -> Self {
		Tokenizer {
			chars: input.chars().peekable(),
		}
	}

	fn process_number(&mut self, init: Option<String>) -> Result<Token, TokenizationError> {
		let mut num = String::from(init.unwrap_or(String::from("")));

		if let Some('-') = self.chars.peek() {
			num.push(self.chars.next().unwrap());
		}

		if num == '-'.to_string() && matches!(self.chars.peek(), Some(' ') | None) {
			Ok(Token::Operator(num))
		} else {
			while let Some('0'..='9') | Some('.') = self.chars.peek() {
				num.push(self.chars.next().unwrap());
			}

			// Push our number token
			Ok(Token::Number(num))
		}
	}

	fn process_operator(&mut self) -> Result<Token, TokenizationError> {
		let mut op = String::new();
	
		op.push(self.chars.next().unwrap());

		if op == String::from("-") && !matches!(self.chars.peek(), Some(' ')) {
			return self.process_number(Some(op.clone()));
		}

		Ok(Token::Operator(op))
	}
	
	fn process_unit(&mut self) -> Result<Token, TokenizationError> {
		let mut unit = String::new();
		
		// This first while loop allows us to capture every character that could
		// potentially be a unit (px|vh|vw|%|rem|...) but will also match "var".
		while let Some('%') | Some('a'..='z') = self.chars.peek() {
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
		if let Some('(') = &self.chars.peek() {
			variable.push(self.chars.next().unwrap());

			// Checks for "--"
			while let Some('-') = &self.chars.peek() {
				variable.push(self.chars.next().unwrap());

				// Next value should be a-z or "0-9" or the closing parenthesis
				while let Some('a'..='z') | Some('0'..='9') | Some(')') = &self.chars.peek() {
					variable.push(self.chars.next().unwrap());
				}
			}
		}

		Ok(Token::Variable(variable))
	}
}

impl<'a> Iterator for Tokenizer<'a> {
	type Item = Result<Token, TokenizationError>;

	fn next(&mut self) -> Option<Self::Item> {
		println!("{:?}", self.chars.peek());
		// Implement the logic for tokenizing the input string
		match self.chars.peek() {
			Some(&c) => match c {
				'0'..='9' => Some(self.process_number(None)),
				'%' | 'a'..='z' => Some(self.process_unit()),
				'+' | '-' | '*' | '/' => Some(self.process_operator()),
				' ' => {
					self.chars.next();
					Some(Ok(Token::Whitespace))
				},
				'(' => {
					self.chars.next();
					Some(Ok(Token::LeftParen))
				},
				')' => {
					self.chars.next();
					Some(Ok(Token::RightParen))
				},
				_ => Some(Err(TokenizationError::UnrecognizedCharacter(c)))
			}
			None => None
		}
	}
}