use crate::token::Token;

#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenizationError {
	UnrecognizedToken(Token),
	UnrecognizedCharacter(char),
	InvalidVariable(String)
}