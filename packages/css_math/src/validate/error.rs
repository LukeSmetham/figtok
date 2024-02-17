#[derive(Debug, PartialEq)]
pub enum ValidationError {
    DivisionByZero,
    IncompleteExpression,
    InvalidDivisionRHS,
	InvalidToken,
	InvalidWhitespace,
	InvalidVariable(String),
	InvalidNumber(String),
	InvalidDivMulOperation,
    InvalidSyntax,
	NoOperators,
	MismatchedParentheses,
    MixedUnitArithmetic,
    MultiplicationWithUnits
}