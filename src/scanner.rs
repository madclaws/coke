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

    pub fn scan_tokens(mut self) -> Vec<Token> {
        let mut current: u32 = 0; // current index of lexeme
        let line: u32 = 1;
        println!("SOURCE\n{}", self.source);
        while current < self.source.len() as u32{
            // Scan a single token
            // start and current is used for taking the lexeme out of source (substring from the source with start and length maybe) 
            let start = current;
            let source_char = self.source.get(current as usize .. (current+1) as usize).unwrap();
            current += 1;
            println!("source_char {}", source_char);
            if let Some(token) = scan_token(source_char) {
                self.add_token(token, String::from(""), start, current, line);
            } else {
                crate::error(line as i32, "Unexpected error");
                break;
            }
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

#[cfg(test)]
mod tests {
    use super::Scanner;
    use crate::token::*;
    
    #[test]
    fn create_new_scanner() {
        let scanner = Scanner::new("()".to_string());
        assert_eq!(scanner.source, "()".to_string());
        assert_eq!(scanner.tokens.len(), 0);
    }

    #[test]
    fn test_scan_tokens() {
        let scanner = Scanner::new("()".to_string());
        let mut tokens: Vec<Token> = scanner.scan_tokens(); 
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens.pop().unwrap().lexeme, "".to_string());
        assert_eq!(tokens.pop().unwrap().lexeme, ")".to_string());
        assert_eq!(tokens.pop().unwrap().lexeme, "(".to_string());
        assert_eq!(tokens.pop(), None);
    }

    #[test]
    fn test_scan_invalid_tokens() {
        let scanner = Scanner::new("#)".to_string());
        let tokens: Vec<Token> = scanner.scan_tokens(); 
        assert_eq!(tokens.len(), 1);
    }
}

