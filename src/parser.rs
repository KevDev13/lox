// parser

use crate::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize;
}

impl Parser {
    pub fn new(t: &Vec<Token>) -> Parser {
        Parser {
            tokens: t,
            current: 0
        }
    }
