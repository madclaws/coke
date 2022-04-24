/// Module contains traits and structs that encompasses all productions of the syntax grammar

use crate::token::*;
use crate::token_type::*;

#[allow(dead_code)]
pub struct Expr {

}

pub struct Binary {
    left: Expr,
    operator: Token,
    right: Expr 
}

impl Binary {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Binary {
        Binary{left, operator, right}
    }
}

pub struct Grouping {
    expression: Expr
}

impl Grouping {
    pub fn new(expression: Expr) -> Grouping {
        Grouping{expression}
    }
}

pub struct Unary {
    operator: Token,
    right: Expr
}

impl Unary {
    pub fn new(operator: Token, right: Expr) -> Unary {
        Unary{operator, right}
    }
}

pub struct Literals {
    value: Literal
}

impl Literals {
    pub fn new(value: Literal) -> Literals {
        Literals{value}
    }
}


