/// Module contains traits and structs that encompasses all productions of the syntax grammar

use crate::token::*;
use crate::token_type::*;

pub trait Visitor<T> {
    fn visit_binary(&mut self, binary: Binary) -> T;
    fn visit_expr(&mut self, expr: Expr) -> T;
    fn visit_grouping(&mut self, grouping: Grouping) -> T;
    fn visit_unary(&mut self, unary: Unary) -> T;
    fn visit_literal(&mut self, literal: Literals) -> T;
}

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


