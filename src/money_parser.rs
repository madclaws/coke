/// Money parser, an exercise

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