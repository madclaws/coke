/// Money parser, an exercise
/// Grammar 
/// money  = currency amount
/// currency = '$' | '£' | '€'
/// amount = number

use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MTokenType {
    Currency,
    Number
}

pub struct MToken<'a> {
    token_type: MTokenType,
    lexeme: &'a str
}

impl <'a>MToken<'a> {
    pub fn new(token_type: MTokenType, lexeme: &'a str) -> Self {
        MToken{token_type, lexeme}
    }
}

pub enum Currency {
    USD,
    GBP,
    EUR
}

pub struct MoneyNode {
    currency: Currency,
    amount: i32
}

pub enum ParseError {
    UnExpectedToken(MTokenType, MTokenType),
    InvalidAmount
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnExpectedToken(expected, found) => 
                write!(f, "Unexpected token expected {:?}, found {:?}", expected, found),
            Self::InvalidAmount => write!(f, "Invalid amount")
        }
    }
}

type ParseResult<T> = Result<T, ParseError>;

pub struct MParser<'a> {
    tokens: Vec<MToken<'a>>,
    current: u32
}

impl <'a>MParser<'a> {
    fn new(tokens: Vec<MToken<'a>>) -> Self {
        MParser{tokens, current: 0}
    }

    // Check if we are end of token stream
    fn is_eof(&self) -> bool {
        self.current >= self.tokens.len().try_into().unwrap()
    }

    // Get a token
    fn peek(&self) -> Option<&MToken<'a>> {
        self.tokens.get(self.current as usize)
    }

    // check if current token is same as the token_type
    fn is_match(&self, token_type: MTokenType) -> bool {
        !self.is_eof() && self.peek().unwrap().token_type == token_type
    }

    // consume and inc the current pointer
    fn advance(&mut self) {
        self.current += 1;
    }

    fn parse_amount(&mut self) -> ParseResult<i32> {
        // check if current token is a amount 
        let token = self.peek().unwrap();
        if self.is_match(MTokenType::Number) {
            let result = token.lexeme.parse::<i32>().map_err(|_| ParseError::InvalidAmount);
            self.advance();
            return result;
        }
        return Err(
            ParseError::UnExpectedToken(MTokenType::Number, token.token_type)
        )
    }
}