use super::error::ValidationError;
use crate::token::Token;

pub(crate) fn validator(t: &[Token]) -> Result<(), ValidationError> {
    // convert our token slice into a vec of tuples, containing an optional previous token, and the current token in the iterator.
    let tokens: Vec<(Option<&Token>, &Token)> = std::iter::once(None)
        .chain(t.iter().map(Some))
        .zip(t.iter())
        .collect();

    // Use a fold on the tokens iterator to count parentheses. Parens should alwyas be closed out so if the value
    // is non-zero then we have an invalid string.
    let parentheses = tokens.iter().fold(0, |count, (_, current)| match current {
        Token::LeftParen => count + 1,
        Token::RightParen => count - 1,
        _ => count,
    });

    if parentheses != 0 {
        return Err(ValidationError::MismatchedParentheses);
    }

    let operators = t.iter().any(|c| matches!(c, Token::Operator(_)));

    if !operators {
        return Err(ValidationError::NoOperators);
    }

    for (prev, current) in tokens.iter() {
        // validate based on the previous token
        // Most useful for check a Token follows another kind of Token etc.
        match prev {
            Some(Token::Operator(_)) => {}
            Some(Token::Number(_)) => {
                // Check that only a unit or whitespace follow a number.
                if !matches!(current, Token::Unit(_)) {
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
}
