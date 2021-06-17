// Token Types

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
    Identifier, String, Number,

    // keywords
    And, Class, Else, Flase, Fun, For, If, Nil,
    Or, Print, Return, Super, This, True, Var, While,

    Eof,
}
