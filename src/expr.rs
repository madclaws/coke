/// Module contains traits and structs that encompasses all productions of the syntax grammar

use crate::token::*;
use crate::token_type::*;

pub trait Visitor<T> {
    fn visit_binary(&mut self, binary: &Binary) -> T;
    fn visit_grouping(&mut self, grouping: &Grouping) -> T;
    fn visit_unary(&mut self, unary: &Unary) -> T;
    fn visit_literal(&mut self, literal: &Literals) -> T;
}

#[allow(dead_code)]


pub trait Expr{

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
}

impl Expr for Binary{}

pub struct Grouping {
    expr: Box<dyn Expr> 
}

impl Grouping {
    pub fn new(expr: Box<dyn Expr>) -> Grouping {
        Grouping{expr}
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
}

impl Expr for Unary{}

pub struct Literals {
    pub value: Option<Literal> 
}

impl Literals {
    pub fn new(value: Option<Literal>) -> Literals {
        Literals{value}
    }
}

impl Expr for Literals {}


