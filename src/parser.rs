// /// Parser module

use crate::expr::*;
use crate::token::*;
use crate::token_type::*;
use std::cell::Cell;

pub struct Parser {
    tokens: Vec<Token>,
    current:  Cell<u32> ,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: Cell::new(0) }
    }

    // Are we at the end of token stream?
    fn is_eof(&self) -> bool {
        self.peek().unwrap().token_type == TokenType::Eof
    }

    // Returns the current token, that is yet to consume
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current.get() as usize)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get((self.current.get() - 1) as usize)
    }

    fn advance(&self) {
        self.current.set(self.current.get() + 1)
    }

    /// Managing the token matching check and consuming of token (if matches)
    fn is_match(&self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true
            }
        }
        false
    }

    /// Checks if the given TokenType matches the current
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_eof() {
            return false
        }
        self.peek().unwrap().token_type == token_type
    }


    fn parse_expression(&self) -> Expr {
        // expression -> equality
        self.parse_equality()
    }

    fn parse_equality(&self) -> Expr {
        // equality -> comparison (("==" | "!=") comparison)*
        let mut expr = self.parse_comparison();
        while self.is_match(vec![TokenType::Equal, TokenType::BangEqual]) {
            let operator: &Token = self.previous().unwrap();
            let right: Expr = self.parse_comparison();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        expr
    }

    fn parse_comparison(&self) -> Expr {
        // comparison -> term ((">" | "<") term)*
        let mut expr = self.parse_term();
        while self.is_match(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator: &Token = self.previous().unwrap();
            let right: Expr = self.parse_term();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        expr
    }

    fn parse_term(&self) -> Expr {
        // term -> factor (("+", "-") factor)*
        let mut expr = self.parse_factor();
        while self.is_match(vec![TokenType::Plus, TokenType::Minus]) {
            let operator: &Token = self.previous().unwrap();
            let right: Expr = self.parse_factor();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        expr
    }

    fn parse_factor(&self) -> Expr {
        // factor -> unary (("/" | "*") unary)*
        let mut expr = self.parse_unary();
        while self.is_match(vec![TokenType::Slash, TokenType::Star]) {
            let operator: &Token = self.previous().unwrap();
            let right: Expr = self.parse_unary();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        expr
    }

    fn parse_unary(&self) -> Expr {
        // unary -> ("-" | "!") unary | primary
        if self.is_match(vec![TokenType::Minus, TokenType::Bang]) {
            let operator: &Token = self.previous().unwrap();
            let right: Expr = self.parse_unary();
            return Expr::Unary(operator, Box::new(right))
        }
        self.parse_primary()
    }

    fn parse_primary(&self) -> Expr {
      if self.is_match(vec![TokenType::False]) {
          return Expr::Lit(Some(&Literal::Booleans(false)))
      }
      if self.is_match(vec![TokenType::True]) {
          return Expr::Lit(Some(&Literal::Booleans(true)))
      }
      if self.is_match(vec![TokenType::Nil]) {
          return Expr::Lit(None)
      }
      if self.is_match(vec![TokenType::String, TokenType::Number]) {
          let token: &Token = self.previous().unwrap();
          return Expr::Lit(token.literal.as_ref())
      }
      unimplemented!()
    }

}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::token::*;
    use crate::token_type::*;

    #[test]
    fn test_parse_primary_1() {
        let number_token = Token::new(TokenType::Number, String::from("2"),
            Some(Literal::Numbers(2.0)), 1);
        let tokens = vec![number_token];
        let parser = Parser::new(tokens);
        let final_expr = parser.parse_expression();
        println!("{final_expr:?}");
    }
}
