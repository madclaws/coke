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
    Number,
}

pub struct MToken<'a> {
    token_type: MTokenType,
    lexeme: &'a str,
}

#[allow(dead_code)]
impl<'a> MToken<'a> {
    pub fn new(token_type: MTokenType, lexeme: &'a str) -> Self {
        MToken { token_type, lexeme }
    }
}

#[derive(Debug, PartialEq)]
pub enum Currency {
    USD,
    GBP,
    EUR,
}

#[derive(Debug, PartialEq)]
pub struct MoneyNode {
    currency: Currency,
    amount: i32,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnExpectedToken(MTokenType, MTokenType),
    InvalidAmount,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnExpectedToken(expected, found) => write!(
                f,
                "Unexpected token expected {:?}, found {:?}",
                expected, found
            ),
            Self::InvalidAmount => write!(f, "Invalid amount"),
        }
    }
}

#[allow(dead_code)]
type ParseResult<T> = Result<T, ParseError>;

pub struct MParser<'a> {
    tokens: Vec<MToken<'a>>,
    current: u32,
}

#[allow(dead_code)]
impl<'a> MParser<'a> {
    fn new(tokens: Vec<MToken<'a>>) -> Self {
        MParser { tokens, current: 0 }
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
            let result = token
                .lexeme
                .parse::<i32>()
                .map_err(|_| ParseError::InvalidAmount);
            self.advance();
            return result;
        }
        return Err(ParseError::UnExpectedToken(
            MTokenType::Number,
            token.token_type,
        ));
    }

    fn parse_currency(&mut self) -> ParseResult<Currency> {
        let token = self.peek().unwrap();
        if self.is_match(MTokenType::Currency) {
            let currency = match token.lexeme {
                "$" => Currency::USD,
                "€" => Currency::EUR,
                _ => Currency::GBP,
            };
            self.advance();
            return Ok(currency);
        }
        Err(ParseError::UnExpectedToken(
            MTokenType::Currency,
            token.token_type,
        ))
    }

    fn parse_money(&mut self) -> ParseResult<MoneyNode> {
        let currency = self.parse_currency()?;
        let amount = self.parse_amount()?;
        Ok(MoneyNode { currency, amount })
    }
}

#[cfg(test)]
mod tests {
    use super::Currency;
    use super::MParser;
    use super::MToken;
    use super::MTokenType;
    use super::MoneyNode;
    use super::ParseError;
    #[test]
    fn test_parse_usd() {
        let tokens = vec![
            MToken::new(MTokenType::Currency, "$"),
            MToken::new(MTokenType::Number, "512"),
        ];
        let mut parser = MParser::new(tokens);
        assert_eq!(
            parser.parse_money(),
            Ok(MoneyNode {
                currency: Currency::USD,
                amount: 512
            })
        )
    }

    #[test]
    fn test_parse_eur() {
        let tokens = vec![
            MToken::new(MTokenType::Currency, "€"),
            MToken::new(MTokenType::Number, "512"),
        ];
        let mut parser = MParser::new(tokens);
        assert_eq!(
            parser.parse_money(),
            Ok(MoneyNode {
                currency: Currency::EUR,
                amount: 512
            })
        )
    }

    #[test]
    fn test_parse_invalid_amount() {
        let tokens = vec![
            MToken::new(MTokenType::Currency, "$"),
            MToken::new(MTokenType::Number, "512rr"),
        ];
        let mut parser = MParser::new(tokens);
        assert_eq!(parser.parse_money(), Err(ParseError::InvalidAmount))
    }

    #[test]
    fn test_parse_unexpected_token() {
        let tokens = vec![
            MToken::new(MTokenType::Number, "512"),
            MToken::new(MTokenType::Currency, "512"),
        ];
        let mut parser = MParser::new(tokens);
        assert_eq!(
            parser.parse_money(),
            Err(ParseError::UnExpectedToken(
                MTokenType::Currency,
                MTokenType::Number
            ))
        )
    }
}
