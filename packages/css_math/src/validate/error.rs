#[derive(Debug, PartialEq)]
pub enum ValidationError {
	InvalidToken,
	InvalidWhitespace,
	InvalidVariable(String),
	InvalidNumber(String),
	NoOperators,
	MismatchedParentheses
}