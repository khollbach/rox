struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: String) -> Self {
        Self { source, tokens: vec![], start: 0, current: 0, line: 1 }
    }

    fn scan_tokens(mut self) -> Vec<Token> {
        while !self.eof() {
            start = current;
            self.scan_token();
        }

        self.tokens.push(Token { type_: TokenType::Eof, (), line });
        self.tokens
    }

    fn eof(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        let type_ = match c {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,

            '!' | '=' | '<' | '>' => {
                let type_ = match (c, self.match_('=')) {
                    ('!', false) => TokenType::Bang,
                    ('!', true) => TokenType::BangEqual,
                    ('=', false) => TokenType::Equal,
                    ('=', true) => TokenType::EqualEqual,
                    ('<', false) => TokenType::Less,
                    ('<', true) => TokenType::LessEqual,
                    ('>', false) => TokenType::Greater,
                    ('>', true) => TokenType::GreaterEqual,
                };
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

            _ => Lox::error(self.line, "Unexpected character."), // todo: global state: how?
        };
        self.add_token(type_);
    }

    fn match_(&mut self, expected: char) -> bool {
        if self.eof {
            return false;
        }
        if char_at(self.source, self.current) != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn advance(&mut self) -> char {
        let c = char_at(&self.source, self.current);
        self.current += c.width; // todo: API = ?
        c
    }

    fn add_token(&mut self, type_: TokenType) {
        self.add_token_literal(type_, None);
    }

    fn add_token_literal(&mut self, type_: TokenType, literal: Option<()>) {
        let text = self.source[self.start..self.current].to_owned();
        self.tokens.push(Token { type_, text, literal, self.line })
    }
}

/// Panics if `idx` is out of bounds, or not at a char boundary.
fn char_at(s: &str, idx: usize) -> char {
    s[idx..].chars().next().unwrap()
}
