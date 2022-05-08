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
        // println!("SOURCE\n{}", self.source);
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
                    while self.peek_next(0).is_some() && self.peek_next(0).unwrap() != '\n' {
                        self.current += 1;
                    }
                } else if self.is_next('*') {
                    loop {
                        if self.peek_next(0).is_some()
                            && self.peek_next(0).unwrap() == '*'
                            && self.peek_next(1).is_some()
                            && self.peek_next(1).unwrap() == '/'
                        {
                            self.current += 2;
                            break;
                        } else if let Some(ch) = self.peek_next(0) {
                            if ch == '\n' {
                                self.line += 1;
                            }
                            self.current += 1;
                        }

                        if self.is_at_end() {
                            println!("block comment end of line");
                            break;
                        }
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
                while self.peek_next(0).is_some() && self.peek_next(0).unwrap() != '"' {
                    if self.peek_next(0).unwrap() == '\n' {
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
                self.add_token(
                    TokenType::String,
                    Some(Literal::Strings(self.get_last_string_char())),
                )
            }
            src_char => {
                if self.is_digit(src_char) {
                    self.number();
                } else if self.is_alpha(src_char) {
                    self.identifier();
                } else {
                    crate::error(
                        self.line as i32,
                        &format!("Unexpected character {}", src_char),
                    );
                }
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
        false
    }

    /// peek the next character, without consuming.
    /// step - which charater to peek from the current one.
    /// Default should be 0, so it peeks the next
    fn peek_next(&self, step: u32) -> Option<char> {
        if self.current + step < self.source.len() as u32 {
            Some(
                self.source
                    .get((self.current + step) as usize..((self.current + step) + 1) as usize)
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap(),
            )
        } else {
            None
        }
    }

    fn get_last_string_char(&self) -> String {
        self.source
            .get((self.start + 1) as usize..(self.current - 1) as usize)
            .unwrap()
            .to_string()
    }

    fn is_digit(&self, digit: char) -> bool {
        ('0'..='9').contains(&digit)
    }

    fn number(&mut self) {
        // consume all the adjacent numbers
        while self.peek_next(0).is_some() && self.is_digit(self.peek_next(0).unwrap()) {
            self.current += 1;
        }
        if self.peek_next(0).is_some()
            && self.peek_next(0).unwrap() == '.'
            && self.peek_next(1).is_some()
            && self.is_digit(self.peek_next(1).unwrap())
        {
            self.current += 1;
            while self.peek_next(0).is_some() && self.is_digit(self.peek_next(0).unwrap()) {
                self.current += 1;
            }
        }
        let number_in_string = self
            .source
            .get((self.start) as usize..self.current as usize)
            .unwrap();
        // println!("number in string {}", number_in_string);
        let number_in_f64 = number_in_string.parse::<f64>().unwrap();
        self.add_token(TokenType::Number, Some(Literal::Numbers(number_in_f64)));
    }

    fn is_alpha(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
    }

    fn identifier(&mut self) {
        while self.peek_next(0).is_some() && self.is_alphanumeric(self.peek_next(0).unwrap()) {
            self.current += 1;
        }
        let lexeme = self
            .source
            .get(self.start as usize..self.current as usize)
            .unwrap();
        if let Some(token_type) = self.get_keyword(lexeme) {
            self.add_token(token_type, None)
        } else {
            self.add_token(TokenType::Identifier, None)
        }
    }

    fn is_alphanumeric(&self, ch: char) -> bool {
        self.is_alpha(ch) || self.is_digit(ch)
    }

    fn get_keyword(&self, lexeme: &str) -> Option<TokenType> {
        match lexeme {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "sub" => Some(TokenType::SubRoutine),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "let" => Some(TokenType::Let),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Scanner;
    use crate::token::*;
    use crate::token_type::*;

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

    #[test]
    fn test_number_literals() {
        let mut scanner = Scanner::new("2".to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens[0].lexeme, "2".to_string());
        assert_eq!(tokens[0].literal, Some(Literal::Numbers(2.0)));
        assert_eq!(tokens.len(), 2);

        let mut scanner = Scanner::new("2.56".to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens[0].lexeme, "2.56".to_string());
        assert_eq!(tokens[0].literal, Some(Literal::Numbers(2.56)));
        assert_eq!(tokens.len(), 2);
    }

    #[test]
    fn test_invalid_numbers() {
        let mut scanner = Scanner::new("2.56.2".to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens.len(), 4);
    }

    #[test]
    fn test_identifiers_and_keywords() {
        let mut scanner = Scanner::new("let order = 3 \n if (3 or 5) {print(\"yo\")}".to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens[0].token_type, TokenType::Let);
        assert_eq!(tokens.len(), 17);
    }

    #[test]
    fn test_block_comments() {
        let mut scanner = Scanner::new("/* let a = 3 */".to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens.len(), 1);

        let mut scanner = Scanner::new("/* let a = 3 \n let b = 5 */".to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();
        assert_eq!(tokens[0].get_meta().3, 2);
        assert_eq!(tokens.len(), 1);

        let mut scanner = Scanner::new("/* let a = 3 \n let b = 5 /*/".to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();
        // println!("{tokens:?}");
        // assert_eq!(tokens[0].get_meta().3, 2);
        assert_eq!(tokens.len(), 1);

        // Handling infinite loop
        let mut scanner = Scanner::new("/* let a = 3 \n let b = 5 /".to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();
        // println!("{tokens:?}");
        // assert_eq!(tokens[0].get_meta().3, 2);
        assert_eq!(tokens.len(), 1);

        // Handling infinite loop
        let mut scanner = Scanner::new("/*".to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();
        println!("{tokens:?}");
        // assert_eq!(tokens[0].get_meta().3, 2);
        assert_eq!(tokens.len(), 1);
    }

    // cargo test test_block_comments -- --nocapture
}
