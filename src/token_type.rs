/// Defining TokenType Enum

pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    SemiColon,
    Minus,
    Plus,
    Slash,
    Star,
    // More than one character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterGreater,
    Less,
    LessEqual,
    // Keywords - These are reserved lexemes of language
    And,
    Class,
    Else,
    False,
    Function,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // Literals - Unreserved lexemes, lexems that represent values
    Identifier,
    String,
    Number,
    Eof
}