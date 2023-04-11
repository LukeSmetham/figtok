#[derive(Debug, PartialEq)]
pub enum ValidationError {
	InvalidToken,
	InvalidWhitespace,
	InvalidNumber(String)
}