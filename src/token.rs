// Token and functions

use std::fmt;
use super::tokentype::*;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    //pub literal:
    pub line: u32,
}

// TODO: implement to_string for Token
