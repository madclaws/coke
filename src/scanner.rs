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
        let mut current: u32 = 0; // current index of lexeme
        let line: u32 = 1;

        while current < self.source.len() as u32{
            // Scan a single token
            // start and current is used for taking the lexeme out of source (substring from the source with start and length maybe) 
            let start = current;
            let source_char = self.source.get(current as usize .. current as usize).unwrap();
            if let Some(token) = scan_token(source_char) {
                self.add_token(token, String::from(""), start, current, line);
            }
            current += current;
        }

        // Its null for literal in below EOF token in book.
        self.tokens.push(Token::new(TokenType::Eof, String::from(""), String::from(""), line));
        self.tokens
    }

    fn add_token(&mut self, token_type: TokenType, literal: String, start: u32, current: u32, line: u32) {
        let lexeme = self.source.get(start as usize .. current as usize).unwrap();
        self.tokens.push(Token::new(token_type, lexeme.to_string(), literal, line));
    }
    
}
        
fn scan_token(ch: &str) -> Option<TokenType>  {
    match ch {
        "(" => Some(TokenType::LeftParen),
        ")" => Some(TokenType::RightParen),
        "{" => Some(TokenType::LeftBrace),
        "}" => Some(TokenType::RightBrace),
        "," => Some(TokenType::Comma),
        "." => Some(TokenType::Dot),
        ";" => Some(TokenType::SemiColon),
        "-" => Some(TokenType::Minus),
        "+" => Some(TokenType::Plus),
        "*" => Some(TokenType::Star),
        _ => None
    }
}

