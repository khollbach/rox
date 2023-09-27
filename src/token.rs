use std::fmt::{self, Display};

use crate::token_type::TokenType;

pub struct Token {
    pub type_: TokenType,
    pub lexeme: String,
    pub literal: (), // todo
    pub line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.type_, self.lexeme, self.literal)
    }
}
