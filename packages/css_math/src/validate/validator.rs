use super::error::ValidationError;
use crate::token::Token;

fn tokens_to_string(tokens: &[Token]) -> String {
    let mut result = String::new();
    for token in tokens {
        match token {
            Token::Number(num) => result.push_str(num),
            Token::Unit(unit) => result.push_str(unit),
            Token::Operator(op) => {
                // Optionally add spaces around operators for readability
                result.push(' ');
                result.push_str(op);
                result.push(' ');
            },
            Token::LeftParen => result.push('('),
            Token::RightParen => result.push(')'),
            Token::Variable(var) => result.push_str(var),
        }
    }
    result
}

pub(crate) fn validator(t: &[Token]) -> Result<(), ValidationError> {
    let parentheses = t.iter().fold(0, |count, current| match current {
        Token::LeftParen => count + 1,
        Token::RightParen => count - 1,
        _ => count,
    });

    if parentheses != 0 {
        return Err(ValidationError::MismatchedParentheses(tokens_to_string(t)));
    }

    // If there are no operators, then we don't have a math statement.
    let operators: bool = t.iter().any(|c| matches!(c, Token::Operator(_)));

    if !operators {
        return Err(ValidationError::NoOperators(tokens_to_string(t)));
    }

    // Initialize context stack to keep track of operations, and nested operations,
    // Each item on the stack is a tuple of (latest_operator, latest_unit, latest_token)
    let mut ctx_stack: Vec<(Option<&str>, Option<&str>, Option<&Token>)> = Vec::new();
    ctx_stack.push((None, None, None));

    for token in t {
        match token {
            // Left parentheses create a new context in the stack
            Token::LeftParen => {
                let context = ctx_stack.last_mut().unwrap();

                // For Left paren, we push the token to the current context before creating a new one
                // This ensures that parentheses are treated as a member of the context they are written in
                // i.e. not a part of the context they delimit.
                context.2 = Some(token);

                ctx_stack.push((None, None, None))
            },
            // Right parentheses pops the latest context from the stack
            Token::RightParen => {
                ctx_stack.pop();
                let stack_len = ctx_stack.len();
                if stack_len == 0 {
                    return Err(ValidationError::MismatchedParentheses(tokens_to_string(t)))
                }

                // For Right paren, we push the token to the context after removing the previous from the stack
                // This ensures that parentheses are treated as a member of the context they are written in
                // i.e. not a part of the context they delimit.
                let context = ctx_stack.last_mut().unwrap();
                context.2 = Some(token);
            },
            Token::Operator(op) => {
                let context = ctx_stack.last_mut().unwrap();

                context.0 = Some(op);
                if op != "/" && op != "*" {
                    context.1 = None;
                }

                if matches!(context.2, None) {
                    return Err(ValidationError::InvalidSyntax(tokens_to_string(t)));
                }

                context.2 = Some(token);
            },
            Token::Unit(unit) => {
                let context = ctx_stack.last_mut().unwrap();

                // If we hit a unit, and we didn't previously have a number then error
                if !matches!(context.2, Some(Token::Number(_))) {
                    return Err(ValidationError::InvalidSyntax(tokens_to_string(t)))
                }

                // If in a division operation, there should be no units on the RHS
                if matches!(context.0, Some("/")) {
                    return Err(ValidationError::InvalidDivisionRHS(tokens_to_string(t)));
                }

                // If in a multiplication operation and there has already been a unit
                // there should be no more units in the operation
                if matches!(context.0, Some("*")) && context.1.is_some() {
                    return Err(ValidationError::MultiplicationWithUnits(tokens_to_string(t)));
                }

                context.1 = Some(unit);
                context.2 = Some(token);
            },
            Token::Number(num) => {
                let context = ctx_stack.last_mut().unwrap();

                // Number should only ever follow None or an operator.
                if !matches!(context.2, None | Some(Token::Operator(_))) {
                    return Err(ValidationError::InvalidSyntax(tokens_to_string(t)))
                }

                if matches!(context.0, Some("/")) && num == "0" {
                    return Err(ValidationError::DivisionByZero(tokens_to_string(t)))
                }

                // Floats should always include the trailing digits (Number should never end in ".")
                if num.ends_with(".") {
                    return Err(ValidationError::InvalidNumber(tokens_to_string(t)));
                }

                context.2 = Some(token);
            },
            Token::Variable(value) => {
                let context = ctx_stack.last_mut().unwrap();

                if !value.starts_with("var(--") || !value.ends_with(")") {
                    return Err(ValidationError::InvalidVariable(tokens_to_string(t)))
                }

                context.2 = Some(token);
            }
        }
    }

    if ctx_stack.len() > 1 || matches!(ctx_stack[0].2, Some(Token::Operator(_))) {
        return Err(ValidationError::IncompleteExpression(tokens_to_string(t)))
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    
    #[test_case(&[Token::Number("100".to_string()), Token::Unit("%".to_string()), Token::Operator("-".to_string()), Token::Number("50".to_string()), Token::Unit("px".to_string())]; "subtract px from percent")]
    #[test_case(&[Token::Number("100".to_string()), Token::Unit("%".to_string()), Token::Operator("*".to_string()), Token::Number("2".to_string())]; "multiply percent by number")]
    #[test_case(&[Token::Number("50".to_string()), Token::Unit("vh".to_string()), Token::Operator("/".to_string()), Token::Number("2".to_string())]; "divide vh by number")]
    #[test_case(&[Token::Number("100".to_string()), Token::Unit("px".to_string()), Token::Operator("+".to_string()), Token::Number("2".to_string()), Token::Unit("em".to_string())]; "add px to em")]
    #[test_case(&[Token::Number("100".to_string()), Token::Unit("px".to_string()), Token::Operator("-".to_string()), Token::LeftParen, Token::Number("50".to_string()), Token::Unit("px".to_string()), Token::Operator("-".to_string()), Token::Number("30".to_string()), Token::Unit("px".to_string()),Token::RightParen]; "nested operation with subtraction")]
    #[test_case(&[Token::LeftParen, Token::Number("100".to_string()), Token::Unit("px".to_string()), Token::Operator("-".to_string()), Token::Number("50".to_string()), Token::Unit("px".to_string()), Token::RightParen, Token::Operator("*".to_string()), Token::Number("3".to_string())]; "nested operation with subtraction and multiplication")]
    #[test_case(&[Token::Number("100".to_string()), Token::Unit("%".to_string()), Token::Operator("/".to_string()), Token::Number("2".to_string()), Token::Operator("-".to_string()), Token::Number("30".to_string()), Token::Unit("px".to_string())] ; "Division and subtraction with no parentheses")]
    #[test_case(&[Token::Variable("var(--width)".to_string()), Token::Operator("*".to_string()), Token::Number("2".to_string())] ; "multiply variable by number")]
    #[test_case(&[Token::Number("100".to_string()), Token::Unit("%".to_string()), Token::Operator("-".to_string()), Token::Variable("var(--padding)".to_string())] ; "subtract variable from percentage")]
    #[test_case(&[Token::Number("100".to_string()), Token::Unit("vh".to_string()), Token::Operator("-".to_string()), Token::LeftParen, Token::Number("2".to_string()), Token::Operator("*".to_string()), Token::Variable("var(--margin)".to_string()), Token::RightParen] ; "complex nested operation with variable")]
    #[test_case(&[
        Token::LeftParen, 
        Token::LeftParen, 
        Token::Number("100".to_string()), 
        Token::Unit("px".to_string()), 
        Token::Operator("+".to_string()), 
        Token::Number("50".to_string()), 
        Token::Unit("px".to_string()), 
        Token::RightParen, 
        Token::Operator("-".to_string()), 
        Token::Number("20".to_string()), 
        Token::Unit("px".to_string()), 
        Token::RightParen, 
        Token::Operator("*".to_string()), 
        Token::Number("2".to_string())
    ]; "double nested add and subtract, then multiply")]
    #[test_case(&[
        Token::Number("100".to_string()), 
        Token::Unit("vw".to_string()), 
        Token::Operator("-".to_string()), 
        Token::LeftParen, 
        Token::Number("80".to_string()), 
        Token::Unit("vw".to_string()), 
        Token::Operator("+".to_string()), 
        Token::Number("30".to_string()), 
        Token::Unit("px".to_string()), 
        Token::RightParen
    ]; "subtract with nested add")]
    #[test_case(&[
        Token::LeftParen, 
        Token::Number("200".to_string()), 
        Token::Operator("/".to_string()), 
        Token::Number("2".to_string()), 
        Token::RightParen, 
        Token::Operator("*".to_string()), 
        Token::LeftParen, 
        Token::Number("50".to_string()), 
        Token::Operator("+".to_string()), 
        Token::Number("50".to_string()), 
        Token::RightParen
    ]; "nested division and addition")]
    fn valid(input: &[Token]) {
        let result = validator(input);
        assert!(result.is_ok())
    }

    #[test_case(&[
        Token::Number("100".to_string()), 
        Token::Unit("px".to_string()), 
        Token::Operator("/".to_string()), 
        Token::Number("0".to_string())
    ]; "division by zero")]
    #[test_case(&[
        Token::Number("100".to_string()), 
        Token::Unit("px".to_string()), 
        Token::Operator("/".to_string()), 
        Token::Number("2".to_string()), 
        Token::Unit("px".to_string())
    ]; "divide px by px")]
    #[test_case(&[
        Token::Number("100".to_string()), 
        Token::Unit("%".to_string()), 
        Token::Operator("*".to_string()), 
        Token::Number("50".to_string()), 
        Token::Unit("%".to_string())
    ]; "multiply percent by percent")]
    #[test_case(&[
        Token::Number("100".to_string()), 
        Token::Operator("+".to_string())
    ]; "incomplete expression")]
    #[test_case(&[
        Token::LeftParen, 
        Token::Number("100".to_string()), 
        Token::Unit("px".to_string())
    ]; "missing closing parenthesis")]
    #[test_case(&[
        Token::Number("100".to_string()), 
        Token::Unit("em".to_string()), 
        Token::Operator("*".to_string()), 
        Token::Number("5".to_string()), 
        Token::Unit("vh".to_string())
    ]; "multiplication with two units")]
    #[test_case(&[
        Token::Variable("var(--base)".to_string()), 
        Token::Operator("/".to_string()), 
        Token::Number("2".to_string()), 
        Token::Unit("px".to_string())
    ]; "division of variable by unit")]
    #[test_case(&[
        Token::Unit("px".to_string()), 
        Token::Number("100".to_string()), 
        Token::Operator("+".to_string()), 
        Token::Number("100".to_string())
    ]; "unit before number")]
    #[test_case(&[
        Token::Operator("*".to_string()), 
        Token::Number("100".to_string()), 
        Token::Unit("px".to_string())
    ]; "operator at the start")]
    #[test_case(&[
        Token::LeftParen, 
        Token::Operator("*".to_string()), 
        Token::Number("100".to_string()), 
        Token::Unit("px".to_string()), 
        Token::Operator("-".to_string()), 
        Token::Number("50".to_string()), 
        Token::Unit("px".to_string()), 
        Token::RightParen, 
        Token::Operator("*".to_string()), 
        Token::Number("3".to_string())
    ]; "operator at the start of nested expression")]
    #[test_case(&[
        Token::LeftParen, 
        Token::Number("100".to_string()), 
        Token::Unit("px".to_string()), 
        Token::Operator("*".to_string()), 
        Token::LeftParen, 
        Token::Number("50".to_string()), 
        Token::RightParen, 
        Token::Operator("+".to_string()), 
        Token::Number("50".to_string()), 
        Token::Unit("px".to_string())
    ]; "nested multiplication without unit in inner expression")]
    #[test_case(&[
        Token::Number("100".to_string()), 
        Token::Unit("px".to_string()), 
        Token::Operator("/".to_string()), 
        Token::LeftParen, 
        Token::Number("10".to_string()),
        Token::Unit("px".to_string()),
        Token::Operator("/".to_string()),
        Token::Number("0".to_string()), 
        Token::RightParen
    ]; "division by zero in nested expression")]
    #[test_case(&[
        Token::LeftParen, 
        Token::Operator("-".to_string()), 
        Token::Number("100".to_string()), 
        Token::Unit("px".to_string()), 
        Token::RightParen, 
        Token::Operator("*".to_string()), 
        Token::Number("3".to_string())
    ]; "invalid operator at the start of nested expression")]
    #[test_case(&[
        Token::LeftParen, 
        Token::Number("100".to_string()), 
        Token::Unit("px".to_string()), 
        Token::Operator("-".to_string()), 
        Token::RightParen, 
        Token::Number("50".to_string())
    ]; "missing operator before a number")]
    #[test_case(&[
        Token::LeftParen, 
        Token::Number("100".to_string()), 
        Token::Unit("%".to_string()), 
        Token::Operator("+".to_string()), 
        Token::Number("50".to_string()), 
        Token::Unit("px".to_string()), 
        Token::RightParen, 
        Token::Operator("/".to_string()), 
        Token::Number("2".to_string()), 
        Token::Unit("px".to_string())
    ]; "unit mismatch in division after nested expression")]
    fn invalid(input: &[Token]) {
        let result = validator(input);
        assert!(result.is_err())
    }
}
