use std::fmt::{Display, self};

struct Token {
    type_: TokenType,
    lexeme: String,
    literal: (), // todo
    line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.type_, self.lexeme, self.literal)
    }
}
