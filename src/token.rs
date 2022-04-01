/// Token struct contains all about the Tokens
use crate::token_type::*;

#[allow(dead_code)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String, // We will change the datatype to different latet
    line: u32,
}

impl Token {
    #[allow(dead_code)]
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: u32) -> Token {
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
        format!("{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}
