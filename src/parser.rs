// parser

use crate::Token;
use crate::TokenType;
use crate::Expr;

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

    fn expression() -> Expr {
        equality()
    }

    fn equality(&self) -> Expr {
        let mut expr = comparison();

        while match_tokens() {
            let oper = previous();
            let r = comparison();
        }
    }

    fn check(&self, ttype: TokenType) -> bool {
        if is_at_end() {
            false
        } else {
            peek().type == ttype
        }
    }

    fn advance(&self) -> Token {
        if !is_at_end() {
            self.current++;
        }

        previous()
    }

    fn is_at_end() -> bool {
        peek().type == TokenType::Eof;
    }

    fn peek(&self) -> Token {
        self.tokens.get(current)
    }

    fn previous(&self) -> Token {
        self.tokens.get(current - 1)
    }
}
