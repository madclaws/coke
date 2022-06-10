// /// Parser module

use crate::expr::*;
use crate::token::*;
use crate::token_type::*;
use std::cell::Cell;
use std::fmt;
// use std::fmt::Display;
use std::fmt::Debug;

pub struct Parser {
    tokens: Vec<Token>,
    current:  Cell<u32> ,
}

#[allow(dead_code)]
type ParseResult<T> = Result<T, ParseError>;

#[allow(dead_code)]
pub enum ParseError {
    UnExpectedToken,
    ExpectExpression
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unexpected token")
    }
}

#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
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


    pub fn parse_expression(&self) -> ParseResult<Expr>  {
        // expression -> equality
        self.parse_equality()
    }

    fn parse_equality(&self) -> ParseResult<Expr>  {
        // equality -> comparison (("==" | "!=") comparison)*
        let mut expr = self.parse_comparison()?;
        while self.is_match(vec![TokenType::Equal, TokenType::BangEqual]) {
            let operator: &Token = self.previous().unwrap();
            let right: Expr = self.parse_comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        Ok(expr)
    }

    fn parse_comparison(&self) -> ParseResult<Expr> {
        // comparison -> term ((">" | "<") term)*
        let mut expr = self.parse_term()?;
        while self.is_match(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator: &Token = self.previous().unwrap();
            let right: Expr = self.parse_term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        Ok(expr)
    }

    fn parse_term(&self) -> ParseResult<Expr> {
        // term -> factor (("+", "-") factor)*
        let mut expr = self.parse_factor()?;
        while self.is_match(vec![TokenType::Plus, TokenType::Minus]) {
            let operator: &Token = self.previous().unwrap();
            let right: Expr = self.parse_factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        Ok(expr)
    }

    fn parse_factor(&self) -> ParseResult<Expr> {
        // factor -> unary (("/" | "*") unary)*
        let mut expr = self.parse_unary()?;
        while self.is_match(vec![TokenType::Slash, TokenType::Star]) {
            let operator: &Token = self.previous().unwrap();
            let right: Expr = self.parse_unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        Ok(expr)
    }

    fn parse_unary(&self) -> ParseResult<Expr>  {
        // unary -> ("-" | "!") unary | primary
        if self.is_match(vec![TokenType::Minus, TokenType::Bang]) {
            let operator: &Token = self.previous().unwrap();
            let right: Expr = self.parse_unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)))
        }
        self.parse_primary()
    }

    fn parse_primary(&self) -> ParseResult<Expr>  {
        if self.is_match(vec![TokenType::False]) {
            return Ok(Expr::Lit(Some(&Literal::Booleans(false))))
        }
        if self.is_match(vec![TokenType::True]) {
            return Ok(Expr::Lit(Some(&Literal::Booleans(true))))
        }
        if self.is_match(vec![TokenType::Nil]) {
            return Ok(Expr::Lit(None))
        }
        
        if self.is_match(vec![TokenType::String, TokenType::Number]) {
            let token: &Token = self.previous().unwrap();
            return Ok(Expr::Lit(token.literal.as_ref()))
        }

        if self.is_match(vec![TokenType::LeftParen]) {
            let expr = self.parse_expression()?;
            match self.consume(TokenType::RightParen, "Expect ')' after expression.".to_owned()) {
                Ok(_token) => return Ok(Expr::Grouping(Box::new(expr))),
                Err(err) => return Err(err)
            }
        }

        crate::errorv2(self.peek().unwrap(), "Expect Expression");
        Err(ParseError::ExpectExpression)
    }

    fn consume(&self, token_type: TokenType, message: String) -> Result<Option<&Token> , ParseError> {
        if self.check(token_type) {
            self.advance();
            return Ok(self.previous())
        }
        crate::errorv2(self.previous().unwrap(), &message);
        Err(ParseError::UnExpectedToken)
    }

    fn synchronize(&self) {
        self.advance();
        while !self.is_eof() {
            if self.previous().unwrap().token_type == TokenType::SemiColon {
                return
            }

            match self.peek().unwrap().token_type {
                TokenType::Class => return,
                TokenType::SubRoutine => return,
                TokenType::Let => return,
                TokenType::For => return,
                TokenType::If => return,
                TokenType::While => return,
                TokenType::Print => return,
                TokenType::Return => return,
                _ => self.advance()
            }
        }
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

    #[test]
    fn test_parse_simple_math() {
        let num_token_1 = Token::new(TokenType::Number, String::from("2"), Some(Literal::Numbers(2.0)), 1);
        let num_token_2 = Token::new(TokenType::Number, String::from("3"), Some(Literal::Numbers(3.0)), 1);
        let op_token = Token::new(TokenType::Plus, String::from("+"), None, 1);
        let eof = Token::new(TokenType::Eof, String::from(""), None, 1);
        let parser = Parser::new(vec![num_token_1, op_token, num_token_2, eof]);
        let ast = parser.parse_expression();
        println!("{ast:?}");
    }
}
