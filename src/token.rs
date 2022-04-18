/// Token struct contains all about the Tokens
use crate::token_type::*;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    line: u32,
}

impl Token {
    #[allow(dead_code)]
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: u32,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    /// Token information in a source file in String
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }

    #[allow(dead_code)]
    pub fn get_meta(&self) -> (&TokenType, String, &Option<Literal>, u32) {
        (&self.token_type, self.lexeme.to_string(), &self.literal, self.line)
    }
}
