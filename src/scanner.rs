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

    pub fn scan_tokens(&mut self) -> &mut Vec<Token> {
        self.current = 0; // current index of lexeme
        self.line = 1;
        println!("SOURCE\n{}", self.source);
        while !self.is_at_end() {
            // Scan a single token
            // start and current is used for taking the lexeme out of source (substring from the source with start and length maybe)
            self.start = self.current;

            self.scan_token();
        }

        // Its None for literal in below EOF token in book.
        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            None,
            self.line,
        ));
        &mut self.tokens
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
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

    fn get_current_char(&mut self) -> char {
        self.source
            .get(self.current as usize..(self.current + 1) as usize)
            .unwrap()
            .chars()
            .next()
            .unwrap()
    }

    fn scan_token(&mut self) {
        let source_char = self.get_current_char();
        self.current += 1;
        match source_char {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            ';' => self.add_token(TokenType::SemiColon, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                if self.is_next('=') {
                    self.add_token(TokenType::BangEqual, None)
                } else {
                    self.add_token(TokenType::Bang, None)
                }
            }
            '=' => {
                if self.is_next('=') {
                    self.add_token(TokenType::EqualEqual, None)
                } else {
                    self.add_token(TokenType::Equal, None)
                }
            }
            '>' => {
                if self.is_next('=') {
                    self.add_token(TokenType::GreaterEqual, None)
                } else {
                    self.add_token(TokenType::Greater, None)
                }
            }
            '<' => {
                if self.is_next('=') {
                    self.add_token(TokenType::LessEqual, None)
                } else {
                    self.add_token(TokenType::Less, None)
                }
            }
            '/' => {
                if self.is_next('/') {
                    while !self.is_at_end() && self.peek_next(0) != '\n' {
                        self.current += 1;
                    }
                } else {
                    self.add_token(TokenType::Slash, None)
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                while !self.is_at_end() && self.peek_next(0) != '"' {
                    if self.peek_next(0) != '\n' {
                        self.line += 1;
                    }
                    self.current += 1;
                }
                if self.is_at_end() {
                    crate::error(self.line as i32, "Unterminated string.");
                    return;
                }
                // consuming the last closing string
                self.current += 1;
                self.add_token(TokenType::String, Some(Literal::Strings(self.get_last_string_char())))
            }
            src_char => if self.is_digit(src_char) {
                self.number();
            } else {
                crate::error(self.line as i32, "Unexpcted character.")
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= (self.source.len() as u32)
    }

    fn is_next(&mut self, expected: char) -> bool {
        if !self.is_at_end() {
            if self.get_current_char() == expected {
                self.current += 1;
                return true;
            }
            return false;
        }
        return false;
    }

    /// peek the next character, without consuming.
    /// step - which charater to peek from the current one.
    /// Default should be 0, so it peeks the next
    fn peek_next(&self, step: u32) -> char {
        self.source
            .get((self.current + step) as usize..((self.current + step) + 1) as usize)
            .unwrap()
            .chars()
            .next()
            .unwrap()
    }

    fn get_last_string_char(&self) -> String {
        self.source
            .get((self.start + 1) as usize..(self.current - 1) as usize)
            .unwrap()
            .to_string()
    }

    fn is_digit(&self, digit: char) -> bool {
        if digit >= '0' && digit <= '9' {
            true
        } else {
            false
        }
    }

    fn number(&mut self) {
        // consume all the adjacent numbers
        while self.is_digit(self.peek_next(0)) {
            self.current += 1;
        }
        if self.peek_next(0) == '.' && self.is_digit(self.peek_next(1)) {
            self.current += 1;
            while self.is_digit(self.peek_next(0)) {
                self.current += 1;
            }
        }
        let number_in_string =  self.source
        .get((self.start) as usize..self.current as usize)
        .unwrap();

        let number_in_f64 = number_in_string.parse::<f64>().unwrap();
        self.add_token(TokenType::Number, Some(Literal::Numbers(number_in_f64)));
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
        let mut scanner = Scanner::new("()".to_string());
        let tokens: &mut Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens.pop().unwrap().lexeme, "".to_string());
        assert_eq!(tokens.pop().unwrap().lexeme, ")".to_string());
        assert_eq!(tokens.pop().unwrap().lexeme, "(".to_string());
        assert_eq!(tokens.pop(), None);
    }

    #[test]
    fn test_scan_invalid_tokens() {
        let mut scanner = Scanner::new("#)".to_string());
        let tokens: &mut Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens.len(), 2);
    }

    #[test]
    fn test_operator_scan_tokens_1() {
        let mut scanner = Scanner::new("!".to_string());
        let tokens: &mut Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.pop().unwrap().lexeme, "".to_string());
        assert_eq!(tokens.pop().unwrap().lexeme, "!".to_string());
        assert_eq!(tokens.pop(), None);
    }

    #[test]
    fn test_operator_scan_tokens_3() {
        let mut scanner = Scanner::new("!=".to_string());
        let tokens: &mut Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.pop().unwrap().lexeme, "".to_string());
        assert_eq!(tokens.pop().unwrap().lexeme, "!=".to_string());
        assert_eq!(tokens.pop(), None);

        let mut scanner_2 = Scanner::new("! =".to_string());
        let tokens_2: &Vec<Token> = scanner_2.scan_tokens();
        assert_eq!(tokens_2.len(), 3);
    }

    #[test]
    fn test_longer_lexemes_1() {
        let mut scanner = Scanner::new("//".to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens.len(), 1);

        let mut scanner = Scanner::new("/".to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens.len(), 2);

        let mut scanner = Scanner::new("//yoyo".to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens.len(), 1);

        let mut scanner = Scanner::new("// this is a comment".to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens.len(), 1);

        let mut scanner = Scanner::new("(( )){} // grouping stuff".to_string());
        let tokens: &mut Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens.len(), 7);

        assert_eq!(tokens.pop().unwrap().lexeme, "".to_string());
        assert_eq!(tokens.pop().unwrap().lexeme, "}".to_string());

        let mut scanner = Scanner::new("() \n {}\n".to_string());
        let mut _tokens: &Vec<Token> = scanner.scan_tokens();
        assert_eq!(scanner.line, 3);
    }

    #[test]
    fn test_string_literals() {
        let mut scanner = Scanner::new("\"afd\"".to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens.len(), 2);

        let mut scanner = Scanner::new("\"afd".to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens.len(), 1);

        let mut scanner = Scanner::new("(){}\"afd\"".to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens.len(), 6);
    }
}
