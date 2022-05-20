/// Money parser, an exercise
/// Grammar 
/// money  = currency amount
/// currency = '$' | '£' | '€'
/// amount = number

use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
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