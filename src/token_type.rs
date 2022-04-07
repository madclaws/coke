/// Defining TokenType Enum
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
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
    GreaterEqual,
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
    Let,
    While,
    // Literals - Unreserved lexemes, lexems that represent values
    Identifier,
    String,
    Number,
    Eof,
    // Special token for comment
    Comment,
    // For Ignored characters
    Ignored,
}
