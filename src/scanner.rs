/// Scanner scans the source and generate tokens out of it.

use crate::token::*;
use crate::token_type::*;

#[allow(dead_code)]
pub struct Scanner  {
    source: String,
    tokens: Vec<Token>
}

#[allow(dead_code)]
impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner{source, tokens: Vec::new()}
    }

    pub fn scanTokens(mut self) -> Vec<Token> {
        let start: u32 = 0; // start index of a lexeme
        let current: u32 = 0; // current index of lexeme
        let line: u32 = 1;

        while current < self.source.len() as u32{
            // Scan a single token
        }
        self.tokens.push(Token::new(TokenType::Eof, String::from(""), String::from(""), line));
        self.tokens
    }
    
}