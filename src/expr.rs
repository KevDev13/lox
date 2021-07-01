use crate::Token;
use crate::TokenType;
use crate::Literal;

pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        oper: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        args: Vec<Box<Expr>>,
    },
    Get {
        obj: Box<Expr>,
        name: Token,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Logical {
        left: Box<Expr>,
        oper: Token,
        right: Box<Expr>,
    },
    Set {
        obj: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
    Super {
        keyword: Token,
        method: Token,
    },
    This {
        keyword: Token,
    },
    Unary {
        oper: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

pub trait Visitor<T> {
    fn visit_assign(&mut self, name: &Token, value: &Expr) -> T;
    fn visit_binary(&mut self, left: &Expr, oper: &Token, right: &Expr) -> T;
    fn visit_call(&mut self, callee: &Expr, paren: &Token, args: &Vec<Expr>) -> T;
    fn visit_get(&mut self, obj: &Expr, name: &Token) -> T;
    fn visit_grouping(&mut self, expression: &Expr) -> T;
    fn visit_literal(&mut self, value: &Literal) -> T;
    fn visit_logical(&mut self, left: &Expr, oper: &Token, right: &Expr) -> T;
    fn visit_set(&mut self, obj: &Expr, name: &Token, value: &Expr) -> T;
    fn visit_super(&mut self, keyword: &Token, method: &Token) -> T;
    fn visit_this(&mut self, keyword: &Token) -> T;
    fn visit_unary(&mut self, oper: &Token, right: &Expr) -> T;
    fn visit_variable(&mut self, name: &Token) -> T;
}

pub trait Accept<T> {
    fn accept(&self, v: &mut dyn Visitor<T>) -> T;
}
