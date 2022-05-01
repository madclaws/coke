/// Module contains traits and structs that encompasses all productions of the syntax grammar

use crate::token::*;
use crate::token_type::*;

pub trait Visitor<T> {
    fn visit_binary(&mut self, binary: Binary) -> T;
    fn visit_grouping(&mut self, grouping: Grouping) -> T;
    fn visit_unary(&mut self, unary: Unary) -> T;
    fn visit_literal(&mut self, literal: Literals) -> T;
}

#[allow(dead_code)]


pub trait Expr {
    // fn accept(&self, visitor: T) -> T;
}
pub struct Binary {
    left: Box<dyn Expr> ,
    operator: Token,
    right: Box<dyn Expr> 
}

impl Binary {
    pub fn new(left: Box<dyn Expr> , operator: Token, right: Box<dyn Expr>) -> Binary {
        Binary{left, operator, right}
    }

    pub fn accept<T: Visitor<T>>(self, mut visitor: T) -> T {
        visitor.visit_binary(self)
    }

}

impl Expr for Binary{}

pub struct Grouping {
    expression: Box<dyn Expr> 
}

impl Grouping {
    pub fn new(expression: Box<dyn Expr>) -> Grouping {
        Grouping{expression}
    }

    pub fn accept<T: Visitor<T>>(self, mut visitor: T) -> T {
        visitor.visit_grouping(self)
    }
}

impl Expr for Grouping{}

pub struct Unary {
    operator: Token,
    right: Box<dyn Expr>
}

impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expr>) -> Unary {
        Unary{operator, right}
    }

    pub fn accept<T: Visitor<T>>(self, mut visitor: T) -> T {
        visitor.visit_unary(self)
    }
}

impl Expr for Unary{}

pub struct Literals {
    value: Literal
}

impl Literals {
    pub fn new(value: Literal) -> Literals {
        Literals{value}
    }

    pub fn accept<T: Visitor<T>>(self, mut visitor: T) -> T {
        visitor.visit_literal(self)
    }
}

impl Expr for Literals {}


