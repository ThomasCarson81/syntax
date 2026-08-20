use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}
#[derive(PartialEq, Debug, Clone, Copy, Serialize)]
pub enum TokenType {
    // Values
    Identifier,
    Number,

    // Binary Operators
    Equals,
    Plus,
    Minus,
    Asterisk,
    Slash,

    // Shorthand Binary Operators
    PlusEquals,
    MinusEquals,
    AsteriskEquals,
    SlashEquals,

    // Comparison Operators
    GreaterThan,
    LessThan,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
    EqualTo,
    NotEqualTo,

    // Unary Operators
    Increment,
    Decrement,
    Not,

    // Brackets ()
    LeftBracket,
    RightBracket,

    // Squigglies {}
    LeftSquiggly,
    RightSquiggly,

    // Keywords
    LetTok,
    IfTok,
    ElseTok,
    ForTok,
    WhileTok,

    // Ending Tokens
    Semicolon,
    EndOfFile,
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::Number => write!(f, "Number"),

            TokenType::Equals => write!(f, "Equals"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Asterisk => write!(f, "Asterisk"),
            TokenType::Slash => write!(f, "Slash"),

            TokenType::PlusEquals => write!(f, "PlusEquals"),
            TokenType::MinusEquals => write!(f, "MinusEquals"),
            TokenType::AsteriskEquals => write!(f, "AsteriskEquals"),
            TokenType::SlashEquals => write!(f, "SlashEquals"),

            TokenType::GreaterThan => write!(f, "GreaterThan"),
            TokenType::LessThan => write!(f, "LessThan"),
            TokenType::GreaterThanOrEqualTo => write!(f, "GreaterThanOrEqualTo"),
            TokenType::LessThanOrEqualTo => write!(f, "LessThanOrEqualTo"),
            TokenType::EqualTo => write!(f, "EqualTo"),
            TokenType::NotEqualTo => write!(f, "NotEqualTo"),

            TokenType::Increment => write!(f, "Increment"),
            TokenType::Decrement => write!(f, "Decrement"),
            TokenType::Not => write!(f, "Not"),

            TokenType::LeftBracket => write!(f, "LeftBracket"),
            TokenType::RightBracket => write!(f, "RightBracket"),

            TokenType::LeftSquiggly => write!(f, "LeftSquiggly"),
            TokenType::RightSquiggly => write!(f, "RightSquiggly"),

            TokenType::LetTok => write!(f, "LetTok"),
            TokenType::IfTok => write!(f, "IfTok"),
            TokenType::ElseTok => write!(f, "ElseTok"),
            TokenType::ForTok => write!(f, "ForTok"),
            TokenType::WhileTok => write!(f, "WhileTok"),

            TokenType::Semicolon => write!(f, "Semicolon"),

            TokenType::EndOfFile => write!(f, "EndOfFile"),
        }
    }
}
impl TokenType {
    pub fn from_str(s: &str) -> Option<TokenType> {
        match s {
            "Identifier" => Some(TokenType::Identifier),
            "Number" => Some(TokenType::Number),

            "Equals" => Some(TokenType::Equals),
            "Plus" => Some(TokenType::Plus),
            "Minus" => Some(TokenType::Minus),
            "Asterisk" => Some(TokenType::Asterisk),
            "Slash" => Some(TokenType::Slash),

            "PlusEquals" => Some(TokenType::PlusEquals),
            "MinusEquals" => Some(TokenType::MinusEquals),
            "AsteriskEquals" => Some(TokenType::AsteriskEquals),
            "SlashEquals" => Some(TokenType::SlashEquals),

            "GreaterThan" => Some(TokenType::GreaterThan),
            "LessThan" => Some(TokenType::LessThan),
            "GreaterThanOrEqualTo" => Some(TokenType::GreaterThanOrEqualTo),
            "LessThanOrEqualTo" => Some(TokenType::LessThanOrEqualTo),
            "EqualTo" => Some(TokenType::EqualTo),
            "NotEqualTo" => Some(TokenType::NotEqualTo),

            "Increment" => Some(TokenType::Increment),
            "Decrement" => Some(TokenType::Decrement),
            "Not" => Some(TokenType::Not),

            "LeftBracket" => Some(TokenType::LeftBracket),
            "RightBracket" => Some(TokenType::RightBracket),

            "LeftSquiggly" => Some(TokenType::LeftSquiggly),
            "RightSquiggly" => Some(TokenType::RightSquiggly),

            "LetTok" => Some(TokenType::LetTok),
            "IfTok" => Some(TokenType::IfTok),
            "ElseTok" => Some(TokenType::ElseTok),
            "ForTok" => Some(TokenType::ForTok),
            "WhileTok" => Some(TokenType::WhileTok),

            "Semicolon" => Some(TokenType::Semicolon),

            "EndOfFile" => Some(TokenType::EndOfFile),

            _ => None,
        }
    }
}
