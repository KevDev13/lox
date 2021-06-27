use crate::Token;
use crate::TokenType;

pub enum Expr {
    Assign {
        name: Token,
        value: Expr,
    },
    Binary {
        left: Expr,
        oper: Token,
        right: Expr,
    },
    Call {
        callee: Expr,
        paren: Token,
        args: Vec<Expr>,
    },
    Get {
        obj: Expr,
        name: Token,
    },
    Grouping {
        expression: Expr,
    },
    Literal {
        value: Literal,
    },
    Logical {
        left: Expr,
        oper: Token,
        right: Expr,
    },
    Set {
        obj: Expr,
        name: Token,
        value: Expr,
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
        right: Expr,
    },
    Variable {
        name: Token,
    },
}
