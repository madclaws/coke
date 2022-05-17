/// Parser module

use crate::token::*;
use crate::token_type::*;
use crate::expr::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: u32
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser{tokens, current: 0}
    }

    // expression -> equality;
    fn expression(&mut self) -> &Expr {
        self.equality()
    }

    // equality -> comparison (("!=" | "==") comparison)
    fn equality(&mut self) -> &Expr {
        // unimplemented!()
        let mut expr = self.comparison();
        while self.match_token(&vec![TokenType::Bang, TokenType::BangEqual]) {
            if !self.is_at_end() {
                self.current += 1;
                let operator = self.tokens.get((self.current) as usize).unwrap();
                {
                let right = self.comparison();
                expr = &Expr::Binary(Box::new(expr) , operator, Box::new(right))
                }
            }   
        }
        expr 
    }

    fn comparison(&self) -> &Expr {
        unimplemented!();
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current as usize)
    }

    fn is_at_end(&self) -> bool {
        if let Some(token_ref) = self.peek() {
            token_ref.token_type == TokenType::Eof
        } else {
            true
        }
    }

    fn match_token(&self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                return true
            }        
        }

        return false;
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if let Some(token_ref) = self.peek() {
            if token_ref.token_type == *token_type {
                true
            } else {
                false
            }
        } else {
            false
        }
    }


}