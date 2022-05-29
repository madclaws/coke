/// Module contains traits and structs that encompasses all productions of the syntax grammar
use crate::token::*;
use crate::token_type::*;

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Binary(Box<Expr<'a>>, &'a Token, Box<Expr<'a>>),
    // Grouping(Box<Expr>),
    Lit(Option<&'a Literal>),
    Unary(&'a Token, Box<Expr<'a>>),
}

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T;
}
