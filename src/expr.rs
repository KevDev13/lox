use crate::Token;
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
        args: Vec<Expr>,
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

impl<T> Accept<T> for Expr {
    fn accept(&self, v: &mut dyn Visitor<T>) -> T {
        use Expr::*;
        match self {
            Assign { name, value } => v.visit_assign(name, value),
            Binary { left, oper, right } => v.visit_binary(left, oper, right),
            Call { callee, paren, args } => v.visit_call(callee, paren, args),
            Get { obj, name } => v.visit_get(obj, name),
            Grouping { expression } => v.visit_grouping(expression),
            Literal { value } => v.visit_literal(value),
            Logical { left, oper, right } => v.visit_logical(left, oper, right),
            Set { obj, name, value } => v.visit_set(obj, name, value),
            Super { keyword, method } => v.visit_super(keyword, method),
            This { keyword } => v.visit_this(keyword),
            Unary { oper, right } => v.visit_unary(oper, right),
            Variable { name } => v.visit_variable(name),
        }
    }
}
