/// Parser module

use crate::token::*;
use crate::token_type::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: u32
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser{tokens, current: 0}
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current as usize)
    }

    fn is_at_end(&self) -> bool {
        if let Some(token_ref) = self.peek() {
            token_ref.token_type == TokenType::Eof
        } else {
            true
        }
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get((self.current - 1) as usize)
    }
}