#[derive(Debug, PartialEq)]
pub enum ValidationError {
    DivisionByZero(String),
    IncompleteExpression(String),
    InvalidDivisionRHS(String),
	InvalidToken(String),
	InvalidWhitespace(String),
	InvalidVariable(String),
	InvalidNumber(String),
	InvalidDivMulOperation(String),
    InvalidSyntax(String),
	NoOperators(String),
	MismatchedParentheses(String),
    MixedUnitArithmetic(String),
    MultiplicationWithUnits(String)
}