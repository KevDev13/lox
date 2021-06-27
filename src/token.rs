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
    And, Class, Else, False, Fun, For, If, Nil,
    Or, Print, Return, Super, This, True, Var, While,

    Eof,
}

// Token and functions

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(tt: TokenType, lex: String, lit: Option<Literal>, ln: usize) -> Token {
        Token {
            token_type: tt,
            lexeme: lex,
            literal: lit,
            line: ln
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, fm: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fm, "{:?} {} {:?}", self.token_type, self.lexeme, self.lexeme)
    }
}

#[derive(Clone, Debug)]
pub enum Literal {
    Str(String),
    Number(f64),
    Boolean(bool),
}
