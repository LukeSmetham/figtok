use crate::token::Token;

#[derive(Debug)]
pub enum TokenizationError {
	UnrecognizedToken(Token),
	UnrecognizedCharacter(char),
	InvalidNegativeOperator(String),
	InvalidVariable(String)
}