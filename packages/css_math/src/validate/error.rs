#[derive(Debug, PartialEq)]
pub enum ValidationError {
    DivisionByZero(String),
    IncompleteExpression(String),
    InvalidDivisionRHS(String),
	InvalidVariable(String),
	InvalidNumber(String),
    InvalidSyntax(String),
	NoOperators(String),
	MismatchedParentheses(String),
    MultiplicationWithUnits(String)
}