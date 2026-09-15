use crate::token::{Literal, Token}; // Remember, Literal is either number or a string (the only characters Lox supports)

pub enum Expr {
    Binary {        //  Have to use Box<> for anything that could thoretically recurse at any point
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Literal {
        value_type: Option<Literal>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Unary {
        prefix: Token,
        expression: Box<Expr>,
    },
}