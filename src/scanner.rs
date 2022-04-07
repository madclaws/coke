/// Scanner scans the source and generate tokens out of it.
use crate::token::*;
use crate::token_type::*;
#[allow(dead_code)]
#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

#[allow(dead_code)]
impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        self.current = 0; // current index of lexeme
        self.line = 1;
        println!("SOURCE\n{}", self.source);
        while self.is_at_end() {
            // Scan a single token
            // start and current is used for taking the lexeme out of source (substring from the source with start and length maybe)
            self.start = self.current;

            if let Some(token) = self.scan_token() {
                if token == TokenType::Comment {
                    // If token is comment, then we consume till the end of line
                    while !self.is_at_end() && self.peek_next() != "\n" {
                        self.current += 1;
                    }
                } else {
                    self.add_token(token, String::from(""));
                }
            } else {
                crate::error(self.line as i32, "Unexpected error");
            }
        }

        // Its null for literal in below EOF token in book.
        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            String::from(""),
            self.line,
        ));
        self.tokens
    }

    fn add_token(&mut self, token_type: TokenType, literal: String) {
        let lexeme = self
            .source
            .get(self.start as usize..self.current as usize)
            .unwrap();
        self.tokens.push(Token::new(
            token_type,
            lexeme.to_string(),
            literal,
            self.line,
        ));
    }

    fn scan_token(&mut self) -> Option<TokenType> {
        let source_char = self
            .source
            .get(self.current as usize..(self.current + 1) as usize)
            .unwrap();
        // println!("source_char {}", source_char);
        self.current += 1;
        match source_char {
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
            "!" => {
                if self.is_next("=") {
                    Some(TokenType::BangEqual)
                } else {
                    Some(TokenType::Bang)
                }
            },
            "=" => {
                if self.is_next("=") {
                    Some(TokenType::EqualEqual)
                } else {
                    Some(TokenType::Equal)
                }
            },
            ">" => {
                if self.is_next("=") {
                    Some(TokenType::GreaterEqual)
                } else {
                    Some(TokenType::Greater)
                }
            },
            "<" => {
                if self.is_next("=") {
                    Some(TokenType::LessEqual)
                } else {
                    Some(TokenType::Less)
                }
            },
            "/" => {
                if self.is_next("/") {
                    Some(TokenType::Comment)
                } else {
                    Some(TokenType::Slash)
                }
            }
            _ => None,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current < self.source.len() as u32
    }

    fn is_next(&mut self, expected: &str) -> bool {
        if self.is_at_end() {
            if self
                .source
                .get(self.current as usize..(self.current + 1) as usize)
                .unwrap()
                == expected
            {
                self.current += 1;
                return true;
            }
            return false;
        }
        return false;
    }

    fn peek_next(&self) -> &str {
        self.source.get(self.current as usize..(self.current + 1) as usize).unwrap()
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
        assert_eq!(tokens.len(), 2);
    }

    #[test]
    fn test_operator_scan_tokens_1() {
        let scanner = Scanner::new("!".to_string());
        let mut tokens: Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.pop().unwrap().lexeme, "".to_string());
        assert_eq!(tokens.pop().unwrap().lexeme, "!".to_string());
        assert_eq!(tokens.pop(), None);
    }

    #[test]
    fn test_operator_scan_tokens_3() {
        let scanner = Scanner::new("!=".to_string());
        let mut tokens: Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.pop().unwrap().lexeme, "".to_string());
        assert_eq!(tokens.pop().unwrap().lexeme, "!=".to_string());
        assert_eq!(tokens.pop(), None);

        let scanner_2 = Scanner::new("! =".to_string());
        let tokens_2: Vec<Token> = scanner_2.scan_tokens();
        assert_eq!(tokens_2.len(), 3);
    }
}
