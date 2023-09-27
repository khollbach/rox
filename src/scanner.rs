use crate::{error::ErrorReporter, token::Token, token_type::TokenType};

pub struct Scanner<'a> {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    er: &'a mut ErrorReporter,
}

impl<'a> Scanner<'a> {
    pub fn new(source: String, er: &'a mut ErrorReporter) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            er,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.eof() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            type_: TokenType::Eof,
            lexeme: "".to_owned(),
            line: self.line,
        });
        self.tokens
    }

    fn eof(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            '!' | '=' | '<' | '>' => {
                let (c, c_eq) = match c {
                    '!' => (TokenType::Bang, TokenType::BangEqual),
                    '=' => (TokenType::Equal, TokenType::EqualEqual),
                    '<' => (TokenType::Less, TokenType::LessEqual),
                    '>' => (TokenType::Greater, TokenType::GreaterEqual),
                    _ => unreachable!(),
                };
                let type_ = if self.match_('=') { c_eq } else { c };
                self.add_token(type_);
            }

            '/' => {
                if self.match_('/') {
                    while !self.eof() && self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            '"' => self.string(),

            _ if c.is_ascii_digit() => self.number(),

            _ if is_alpha(c) => self.identifier(),

            _ => self
                .er
                .error(self.line, &format!("Unexpected character: {c:?}")),
        }
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let type_ = match &self.source[self.start..self.current] {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };
        self.add_token(type_);
    }

    fn number(&mut self) {
        while !self.eof() && self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // Consume the '.'

            while !self.eof() && self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value: f64 = match self.source[self.start..self.current].parse() {
            Ok(x) => x,
            Err(e) => {
                // This could fail if the string of digits is too long, e.g.
                self.er
                    .error(self.line, &format!("Failed to parse number: {e}"));
                return;
            }
        };

        self.add_token(TokenType::Number(value));
    }

    fn string(&mut self) {
        while !self.eof() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.eof() {
            self.er.error(self.line, "Unterminated string.");
            return;
        }

        self.advance(); // Consume closing "

        let value = self.source[self.start + 1..self.current - 1].to_owned();
        self.add_token(TokenType::String(value));
    }

    fn match_(&mut self, expected: char) -> bool {
        if self.eof() {
            return false;
        }
        if char_at(&self.source, self.current) != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.eof() {
            return '\0';
        }
        char_at(&self.source, self.current)
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        char_at(&self.source, self.current + 1)
    }

    fn advance(&mut self) -> char {
        let c = char_at(&self.source, self.current);
        self.current += c.to_string().len();
        c
    }

    fn add_token(&mut self, type_: TokenType) {
        let text = self.source[self.start..self.current].to_owned();
        self.tokens.push(Token {
            type_,
            lexeme: text,
            line: self.line,
        });
    }
}

/// Panics if `idx` is out of bounds, or not at a char boundary.
fn char_at(s: &str, idx: usize) -> char {
    s[idx..].chars().next().unwrap()
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
