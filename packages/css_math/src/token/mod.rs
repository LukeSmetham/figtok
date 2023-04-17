#[derive(Debug, PartialEq)]
pub enum Token {
	Number(String),
	Unit(String),
	Variable(String),
	Operator(String),
	LeftParen,
	RightParen,
}