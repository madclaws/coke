/// Module contains traits and structs that encompasses all productions of the syntax grammar

use crate::token::*;
use crate::token_type::*;

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Binary(Box<&'a Expr<'a>>, &'a Token , Box<&'a Expr<'a>>),
    Grouping(Box<Expr<'a>>),
    Lit(Option<Literal>),
    Unary(Token, Box<Expr<'a>>)
}

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T;
}






