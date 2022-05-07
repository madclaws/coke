/// Module contains traits and structs that encompasses all productions of the syntax grammar

use crate::token::*;
use crate::token_type::*;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, Token , Box<Expr>),
    Grouping(Box<Expr>),
    Lit(Option<Literal>),
    Unary(Token, Box<Expr>)
}

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T;
}






