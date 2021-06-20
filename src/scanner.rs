
use lazy_static;
use crate::Token;
use crate::TokenType;
use crate::Literal;
use std::collections::HashMap;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}

impl Scanner {
    pub fn new(s: String) -> Scanner {
        Scanner {
            source: s,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), None, self.line));

        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let ch: char = self.advance();
        match ch {
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
            '!' => {
                if self.match_next('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.match_next('/') {
                    // there is a commented line, so go to the next line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {} // ignore whitespaces
            '\n' => {
                // newline character
                self.line += 1;
            }
            '"' => {
                self.string()
            }
            _ => {
                if self.is_digit(ch) {
                    self.number();
                } else if self.is_alpha(ch) {
                    self.identifier();
                } else {
                    crate::error(self.line, "Unexpected token".to_string())
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.source.chars().nth(self.current).expect("Error in advance");
        self.current += 1;
        ch
    }

    fn add_token(&mut self, t: TokenType) {
        self.add_token_literal(t, None);
    }

    fn add_token_literal(&mut self, ty: TokenType, lit: Option<Literal>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(ty, text.to_string(), lit, self.line));
    }

    fn match_next(&mut self, expect: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).expect("Error in match_next") != expect {
            return false;
        }

        self.current += 1;

        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).expect("Error in peek()")
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            crate::error(self.line, "Unterminated string".to_string());
            return;
        }

        self.advance();

        let val = self.source[self.start + 1..self.current - 1].to_string();
        let lit = Literal {
            string: val,
            number: 0.0,
            boolean: false,
            token: TokenType::Str,
        };
        self.add_token_literal(TokenType::Str, Some(lit));
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // consume the '.'
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let val = self.source[self.start..self.current].to_string();
        let val = val.parse::<f64>().expect("Error converting string to f64 in number()");
        let lit = Literal {
            string: "".to_string(),
            number: val,
            boolean: false,
            token: TokenType::Number,
        };

        self.add_token_literal(TokenType::Number, Some(lit));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).expect("error in peek_next")
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..=self.current];
        if let Some(tok_type) = KEYWORDS.get(text) {
            self.add_token(*tok_type);
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

}

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_owned(),       TokenType::And);
        keywords.insert("class".to_owned(),     TokenType::Class);
        keywords.insert("else".to_owned(),      TokenType::Else);
        keywords.insert("false".to_owned(),     TokenType::False);
        keywords.insert("for".to_owned(),       TokenType::For);
        keywords.insert("fun".to_owned(),       TokenType::Fun);
        keywords.insert("if".to_owned(),        TokenType::If);
        keywords.insert("nil".to_owned(),       TokenType::Nil);
        keywords.insert("or".to_owned(),        TokenType::Or);
        keywords.insert("print".to_owned(),     TokenType::Print);
        keywords.insert("return".to_owned(),    TokenType::Return);
        keywords.insert("super".to_owned(),     TokenType::Super);
        keywords.insert("this".to_owned(),      TokenType::This);
        keywords.insert("true".to_owned(),      TokenType::True);
        keywords.insert("var".to_owned(),       TokenType::Var);
        keywords.insert("while".to_owned(),     TokenType::While);
        keywords
    };
}
