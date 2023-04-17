mod error;
mod validator;
use validator::validator;

use crate::token::Token;

pub(crate) fn validate(tokens: &[Token]) -> bool {
    match validator(tokens) {
        Ok(_) => true,
        Err(_) => false,
    }
}
