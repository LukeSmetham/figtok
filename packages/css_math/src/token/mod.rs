#[derive(Debug, PartialEq)]
pub enum Token {
	Number(String),
	Unit(String),
	Variable(String),
	Operator(String),
	LeftParen,
	RightParen,
	/// If we were writing a CSS Parser intended to handle full files, this would add a lot of unnecessary memory.
	/// However, seeing as we are specifically handling CSS math, and Whitespace is important, combined with the
	/// face that we are only ever testing strings such as "12px * var(--typescale-base)" we be wasting to much
	/// space for the additional benefit of being able to validate we have correct whitespace in the validator.
	Whitespace,
}