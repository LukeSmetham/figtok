#[derive(Debug, PartialEq)]
pub enum ValidationError {
    DivisionByZero,
    InvalidDivisionRHS,
	InvalidToken,
	InvalidWhitespace,
	InvalidVariable(String),
	InvalidNumber(String),
	InvalidDivMulOperation,
	NoOperators,
	MismatchedParentheses,
    MixedUnitArithmetic,
    MultiplicationWithUnits
}