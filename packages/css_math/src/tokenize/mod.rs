use crate::token::Token;
use std::{iter::Peekable, str::Chars};

fn handle_variable_or_unit(chars: &mut Peekable<Chars>) -> Token {
    let mut variable = String::new();

    // This first while loop allows us to capture every character that could
    // potentially be a unit (px|vh|vw|%|rem|...) but will also match "var".
    while let Some('%') | Some('a'..='z') = chars.peek() {
        variable.push(chars.next().unwrap());
    }

    if !variable.starts_with("var") {
        // If the above ran, and we don't have `var`, we must have a unit.
        return Token::Unit(variable);
    }

    // Check that the next char is (
    if let Some('(') = chars.peek() {
        variable.push(chars.next().unwrap());

        // Checks for "--"
        while let Some('-') = chars.peek() {
            variable.push(chars.next().unwrap());

            // Next value should be a-z " 0-9"
            while let Some('a'..='z') | Some('0'..='9') = chars.peek() {
                variable.push(chars.next().unwrap());
            }

            // If the above loops are done, we should have a ')' to close the var() statement.
            if let Some(')') = chars.peek() {
                variable.push(chars.next().unwrap());
            }
        }
    }

    Token::Variable(variable)
}

/// Tokenize is responsible for an input string, and breaking it up into tokens, returning
/// a Vec<Token>. It also does some checks on correct whitespace around Operator tokens.
pub fn tokenize(input: &str) -> Option<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            // Number
            '0'..='9' | '.' => {
                let mut num = String::new();

                // Continually call chars.peek() to check if we have a number or a '.' character
                // We can then call next and push into our num string until we hit something that
                // no longer matches.
                while let Some('0'..='9') | Some('.') = chars.peek() {
                    num.push(chars.next().unwrap());
                }

                // In CSS, float values without the trailing digits are not valid syntax (i.e. `10.`)
                if num.ends_with('.') {
                    return None;
                }

                // Push our number token
                tokens.push(Token::Number(num));

                // Now check the following character, and only continue if we have a whitespace, a unit or the end of the string (None).
                if !matches!(
                    chars.peek(),
                    Some('%' | 'a'..='z') | Some(')') | Some(' ') | None
                ) {
                    return None;
                }
            }
            // Var/Unit ("%", "px", "vh", etc. technically this will match "%" or any a-z chars.)
            '%' | 'a'..='z' => {
                let token = handle_variable_or_unit(&mut chars);

                match token {
                    Token::Variable(t) => {
                        if t.starts_with("var(--") && t.ends_with(")") {
                            tokens.push(Token::Variable(t));
                        } else {
                            return None;
                        }
                    }
                    Token::Unit(t) => tokens.push(Token::Unit(t)),
                    _ => {
                        return None;
                    }
                }

                // If the following character isn't either a close parens, whitespace or the end of the string then exit.
                if !matches!(chars.peek(), Some(')') | Some(' ') | None) {
                    return None;
                }
            }
            '+' | '-' | '*' | '/' => {
                tokens.push(Token::Operator(chars.next().unwrap().to_string()));

                // Operators should always have trailing whitespace.
                if !matches!(chars.peek(), Some(' ')) {
                    return None;
                }
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            ' ' => {
                chars.next();
            }
            _ => return None,
        }
    }
    Some(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod operations {
        use super::*;

        #[test]
        fn basic_math() {
            // Single Operation
            let input = "5 + 10";
            let tokens = tokenize(input).unwrap();
            let expected_tokens = [
                Token::Number(String::from("5")),
                Token::Operator(String::from("+")),
                Token::Number(String::from("10")),
            ];

            assert_eq!(tokens, expected_tokens);
        }

        #[test]
        fn multiple_ops() {
            let input = "5vh - 10px + 100%";
            let tokens = tokenize(input).unwrap();
            let expected_tokens = [
                Token::Number(String::from("5")),
                Token::Unit(String::from("vh")),
                Token::Operator(String::from("-")),
                Token::Number(String::from("10")),
                Token::Unit(String::from("px")),
                Token::Operator(String::from("+")),
                Token::Number(String::from("100")),
                Token::Unit(String::from("%")),
            ];

            assert_eq!(tokens, expected_tokens);
        }

        #[test]
        fn parenthesis() {
            let input = "(2 * 10ch) + 4px";
            let tokens = tokenize(input).unwrap();
            let expected_tokens = [
                Token::LeftParen,
                Token::Number(String::from("2")),
                Token::Operator(String::from("*")),
                Token::Number(String::from("10")),
                Token::Unit(String::from("ch")),
                Token::RightParen,
                Token::Operator(String::from("+")),
                Token::Number(String::from("4")),
                Token::Unit(String::from("px")),
            ];

            assert_eq!(tokens, expected_tokens);
        }
    }

    mod variables {
        use super::*;

        #[test]
        fn valid() {
            let input = "var(--background)";
            let tokens = tokenize(input).unwrap();

            let expected_tokens = [Token::Variable(String::from("var(--background)"))];
            assert_eq!(tokens, expected_tokens);

            let input = "var(--typescale-base)";
            let tokens = tokenize(input).unwrap();

            let expected_tokens = [Token::Variable(String::from("var(--typescale-base)"))];
            assert_eq!(tokens, expected_tokens);

            let input = "var(--typescale-base) * var(--typescale-1)";
            let tokens = tokenize(input).unwrap();

            let expected_tokens = [
                Token::Variable(String::from("var(--typescale-base)")),
                Token::Operator(String::from("*")),
                Token::Variable(String::from("var(--typescale-1)")),
            ];

            assert_eq!(tokens, expected_tokens);
        }

        #[test]
        fn invalid() {
            let invalid_inputs = vec![
                "var(-typescale-base)",
                "var(---typescale-base)",
                "var(-(typescale-base)",
                "var(-)typescale-base)",
                "var(--*typescale)",
                "var(--0typescale)",
                "var(--typescale-1-)",
            ];

            for current in invalid_inputs {
                let tokens = tokenize(current);

                if let None = tokens {
                    assert!(true)
                }
            }
        }
    }

    mod syntax {
        use super::*;

        /// CSS calc statements should always have a space between the operators and operands.
        #[test]
        fn whitepsace() {
            let invalid_inputs = vec![
                "100*10px",
                "100 /10px",
                "100* 10px",
                "100px/10%",
                "100px /10%",
                "100px/ 10%",
                "(2*2) + 4px",
                "(2 * 2)+4px",
                "(2%- 2) + 10vh",
                "100% *(10 + 10vh)",
            ];

            for current in invalid_inputs {
                let tokens = tokenize(current);

                if let None = tokens {
                    assert!(true)
                }
            }

            let valid_inputs = vec![
                "100 * 10px",
                "100px / 10%",
                "(2 * 2) + 4px",
                "(2 * 2) + 4px",
                "(2% - 2) + 10vh",
                "100% * (10 + 10vh)",
            ];

            for current in valid_inputs {
                let tokens = tokenize(current);

                if let Some(_) = tokens {
                    assert!(true)
                }
            }
        }
    }
}
