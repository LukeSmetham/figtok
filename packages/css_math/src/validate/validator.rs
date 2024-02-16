use super::error::ValidationError;
use crate::token::Token;

pub(crate) fn validator(t: &[Token]) -> Result<(), ValidationError> {
    // convert our token slice into a vec of tuples, containing an optional previous token, and the current token in the iterator.
    let tokens: Vec<(Option<&Token>, &Token)> = std::iter::once(None)
        .chain(t.iter().map(Some))
        .zip(t.iter())
        .collect();

    // Use a fold on the tokens iterator to count parentheses. Parens should always be closed out so if the value
    // is non-zero then we have an invalid string.
    let parentheses = tokens.iter().fold(0, |count, (_, current)| match current {
        Token::LeftParen => count + 1,
        Token::RightParen => count - 1,
        _ => count,
    });

    if parentheses != 0 {
        return Err(ValidationError::MismatchedParentheses);
    }

    // If there are no operators, then we don't have a math statement.
    let operators: bool = t.iter().any(|c| matches!(c, Token::Operator(_)));

    if !operators {
        return Err(ValidationError::NoOperators);
    }

    let mut unit_count = 0;
    let mut has_div_or_mul = false;
    let mut last_operator: Option<&str> = None;
    let mut last_unit: Option<&str> = None;

    for token in t {
        match token {
            Token::Unit(unit) => {
                // If the last operator was a "/" and there is already a unit in the statement
                // then this is invalid, the right-hand side of a division statement should never
                // have a unit.
                if matches!(last_operator, Some("/")) {
                    return Err(ValidationError::InvalidDivisionRHS)
                }
                
                // If the last operator was a "*" and there is already a unit in the statement
                // then this is invalid, one side of a multiplication statement should be a 
                // unitless number.
                if matches!(last_operator, Some("*")) && unit_count > 0 {
                    return Err(ValidationError::MultiplicationWithUnits)
                }

                if last_unit.is_some() {
                    if !matches!(last_unit, Some(unit)) {
                        return Err(ValidationError::MixedUnitArithmetic)
                    }
                    last_unit = None;
                } else {
                    last_unit = Some(unit);
                }

                unit_count += 1
            },
            Token::Operator(op) => {
                if op == "/" || op == "*" {
                    has_div_or_mul = true
                }

                last_operator = Some(op);
            }
            _ => {}
        }
    }

    if has_div_or_mul && unit_count > 1 {
        return Err(ValidationError::InvalidDivMulOperation)
    }

    for (prev, current) in tokens.iter() {
        // validate based on the previous token
        // Most useful for check a Token follows another kind of Token etc.
        match prev {
            Some(Token::Operator(op)) => {
                if op == "/" && matches!(current, Token::Number(num) if num == "0") {
                    return Err(ValidationError::DivisionByZero)
                }
            }
            Some(Token::Number(_)) => {
                // Check that only a unit or whitespace follow a number.
                if !matches!(current, Token::Unit(_) | Token::Operator(_)) {
                    return Err(ValidationError::InvalidWhitespace);
                }
            }
            Some(Token::Unit(_)) => {
                if matches!(current, Token::Unit(_)) {
                    return Err(ValidationError::InvalidToken);
                }
            }
            Some(Token::Variable(_)) => {}
            Some(Token::LeftParen) => {}
            Some(Token::RightParen) => {}
            None => {}
        }

        // Validate based on the current token.
        match current {
            Token::Number(value) => {
                // Floats must include the trailing digits in css.
                if value.ends_with(".") {
                    return Err(ValidationError::InvalidNumber(value.to_string()));
                }
            }
            Token::Variable(value) => {
                if !value.starts_with("var(--") || !value.ends_with(")") {
                    return Err(ValidationError::InvalidVariable(value.to_string()));
                }
            }
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::Token;

    use matches::assert_matches;

    mod parentheses {
        use super::*;
        use test_case::test_case;

        #[test_case(vec![Token::LeftParen])]
        #[test_case(vec![Token::RightParen])]
        #[test_case(vec![Token::LeftParen, Token::Number(String::from("5")), Token::Unit(String::from("px")), Token::Operator(String::from("*")), Token::Number(String::from("10"))])]
        fn invalid(tokens: Vec<Token>) {
            let res = validator(&tokens);
            assert!(res.is_err());
            assert_matches!(res.err().unwrap(), ValidationError::MismatchedParentheses)
        }
    }

    mod variables {
        use super::*;
        use test_case::test_case;

        #[test_case(vec![Token::Variable(String::from("var(--typescale)")), Token::Operator(String::from("+")), Token::Number(String::from("1"))])]
        fn valid(tokens: Vec<Token>) {
            let res = validator(&tokens);
            assert!(res.is_ok());
        }

        #[test_case(vec![Token::Variable(String::from("var(-color)")), Token::Operator(String::from("+")), Token::Number(String::from("2"))] ; "missing hyphen")]
        #[test_case(vec![Token::Variable(String::from("var(--color")), Token::Operator(String::from("+")), Token::Number(String::from("2"))] ; "missing paren")]
        fn invalid(tokens: Vec<Token>) {
            let res = validator(&tokens);
            assert!(res.is_err());
            assert_matches!(res.err().unwrap(), ValidationError::InvalidVariable(_));
        }
    }

    mod unit {
        use super::*;

        use test_case::test_case;

        #[test_case(vec![Token::Unit(String::from("px")), Token::Unit(String::from("rem"))] ; "Disallow consecutive units")]
        fn invalid(tokens: Vec<Token>) {
            let res = validator(&tokens);
            assert!(res.is_err());
        }
    }

    mod operators {
        use super::*;

        use test_case::test_case;

        #[test_case(vec![Token::Number(String::from("12")), Token::Operator(String::from("+")), Token::Number(String::from("1"))] ; "Numbers only without units")]
        #[test_case(vec![Token::Number(String::from("12")), Token::Unit(String::from("px")), Token::Operator(String::from("*")), Token::Number(String::from("1"))])]
        #[test_case(vec![Token::Number(String::from("100")), Token::Unit(String::from("%")), Token::Operator(String::from("*")), Token::Number(String::from("2"))] ; "Multiplication where one operand has a unit")]
        #[test_case(vec![Token::Number(String::from("100")), Token::Unit(String::from("px")), Token::Operator(String::from("/")), Token::Number(String::from("10"))] ; "Division where one operand has a unit")]
        #[test_case(vec![Token::Number(String::from("100")), Token::Operator(String::from("*")), Token::Number(String::from("2")), Token::Unit(String::from("rem"))] ; "Multiplication where one operand has a unit #2")]
        #[test_case(vec![Token::Number(String::from("42")), Token::Unit(String::from("px")), Token::Operator(String::from("*")), Token::Variable(String::from("var(--base)"))] ; "Multiplication with a variable on one side")]
        #[test_case(vec![Token::Number(String::from("10")), Token::Unit(String::from("%")), Token::Operator(String::from("/")), Token::Variable(String::from("var(--base)"))] ; "Division with a variable on one side")]
        fn valid(tokens: Vec<Token>) {
            let res = validator(&tokens);
            assert!(res.is_ok())
        }

        #[test_case(vec![Token::Number(String::from("100")), Token::Operator(String::from("/")), Token::Number(String::from("0"))] ; "Division by Zero")]
        #[test_case(vec![Token::Number(String::from("100")), Token::Unit(String::from("%")), Token::Number(String::from("50")), Token::Unit(String::from("px"))] ; "Missing Operator")]
        #[test_case(vec![Token::Number(String::from("12")), Token::Unit(String::from("px")), Token::Operator(String::from("*")), Token::Number(String::from("1")), Token::Unit(String::from("px"))] ; "Multiplication where both operands have units")]
        #[test_case(vec![Token::Number(String::from("12")), Token::Unit(String::from("%")), Token::Operator(String::from("/")), Token::Number(String::from("1")), Token::Unit(String::from("rem"))] ; "Division where both operands have units")]
        #[test_case(vec![Token::Number(String::from("100")), Token::Operator(String::from("/")), Token::Number(String::from("10")), Token::Unit(String::from("%"))] ; "Division where RHS operand has a unit")]
        #[test_case(vec![Token::Variable(String::from("var(--base)")), Token::Operator(String::from("/")), Token::Number(String::from("10")), Token::Unit(String::from("%"))] ; "Division where RHS operand has a unit #2")]
        fn invalid(tokens: Vec<Token>) {
            let res = validator(&tokens);
            assert!(res.is_err());
        }
    }
}
