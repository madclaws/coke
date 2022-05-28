// /// Parser module

use crate::expr::*;
use crate::token::*;
use crate::token_type::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: u32,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    // Are we at the end of token stream?
    fn is_eof(&self) -> bool {
        self.current >= self.tokens.len().try_into().unwrap()
    }

    // Returns the current token
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current as usize)
    }

    fn advance(&mut self) {
        self.current += 1
    }

    fn is_match(&self, token_type: TokenType) -> bool {
        !self.is_eof() && self.peek().unwrap().token_type == token_type
    }

    fn parse_expression(&self) -> Expr {
        // expression -> equality
        self.parse_equality()
    }

    fn parse_equality(&self) -> Expr {
        // equality -> comparison (("==" | "!=") comparison)*
        self.parse_comparison();

        unimplemented!()
    }

    fn parse_comparison(&self) -> Expr {
        // comparison -> term ((">" | "<") term)*
        self.parse_term();
        unimplemented!();
    }

    fn parse_term(&self) -> Expr {
        // term -> factor (("+", "-") factor)*
        self.parse_factor();
        unimplemented!();
    }

    fn parse_factor(&self) -> Expr {
        // factor -> unary (("/" | "*") unary)*
        self.parse_unary();
        unimplemented!()
    }

    fn parse_unary(&self) -> Expr {
        // unary -> ("-" | "!") unary | primary
        self.parse_primary();
        unimplemented!()
    }

    fn parse_primary(&self) -> Expr {
        // if self.is_match(TokenType::Number) {
            // let token = self.peek().unwrap();
            // Expr::Lit(token.literal)
        // }
        unimplemented!()
    }

}
