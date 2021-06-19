// Token Types

#[derive(Clone, Debug, Copy)]
pub enum TokenType {
    // single-char tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Plus, Minus, Semicolon, Slash, Star,

    // one or two char tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // literals
    Identifier, Str, Number,

    // keywords
    And, Class, Else, Flase, Fun, For, If, Nil,
    Or, Print, Return, Super, This, True, Var, While,

    Eof,
}

// Token and functions
use std::fmt;

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    //pub literal:
    pub line: usize,
}

impl Token {
    pub fn new(tt: TokenType, lex: String, ln: usize) -> Token {
        Token {
            token_type: tt,
            lexeme: lex,
            line: ln
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, fm: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fm, "{:?} {} {}", self.token_type, self.lexeme, self.line)
    }
}
