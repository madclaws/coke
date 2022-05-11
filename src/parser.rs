/// Parser module

use crate::token::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: u32
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser{tokens, current: 0}
    }
}