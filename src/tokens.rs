use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}
#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TokenType {
    // Values
    Identifier,
    Number,
    String,
    True,
    False,

    // Binary Operators
    Equals,
    Walrus,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Modulo,

    // Shorthand Binary Operators
    PlusEquals,
    MinusEquals,
    AsteriskEquals,
    SlashEquals,
    ModuloEquals,

    // Comparison Operators
    GreaterThan,
    LessThan,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
    EqualTo,
    NotEqualTo,
    LogicalAnd,
    LogicalOr,

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
    Let,
    If,
    Else,
    For,
    While,
    Fn,
    Return,
    Break,
    Continue,
    Scope,
    Mut,
    Struct,
    Enum,

    // Ending Tokens
    Semicolon,
    Colon,
    Comma,
    EndOfFile,
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::Number => write!(f, "Number"),
            TokenType::String => write!(f, "String"),
            TokenType::True => write!(f, "True"),
            TokenType::False => write!(f, "False"),

            TokenType::Equals => write!(f, "Equals"),
            TokenType::Walrus => write!(f, "Walrus"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Asterisk => write!(f, "Asterisk"),
            TokenType::Slash => write!(f, "Slash"),
            TokenType::Modulo => write!(f, "Modulo"),

            TokenType::PlusEquals => write!(f, "PlusEquals"),
            TokenType::MinusEquals => write!(f, "MinusEquals"),
            TokenType::AsteriskEquals => write!(f, "AsteriskEquals"),
            TokenType::SlashEquals => write!(f, "SlashEquals"),
            TokenType::ModuloEquals => write!(f, "ModuloEquals"),

            TokenType::GreaterThan => write!(f, "GreaterThan"),
            TokenType::LessThan => write!(f, "LessThan"),
            TokenType::GreaterThanOrEqualTo => write!(f, "GreaterThanOrEqualTo"),
            TokenType::LessThanOrEqualTo => write!(f, "LessThanOrEqualTo"),
            TokenType::EqualTo => write!(f, "EqualTo"),
            TokenType::NotEqualTo => write!(f, "NotEqualTo"),
            TokenType::LogicalAnd => write!(f, "LogicalAnd"),
            TokenType::LogicalOr => write!(f, "LogicalOr"),

            TokenType::Increment => write!(f, "Increment"),
            TokenType::Decrement => write!(f, "Decrement"),
            TokenType::Not => write!(f, "Not"),

            TokenType::LeftBracket => write!(f, "LeftBracket"),
            TokenType::RightBracket => write!(f, "RightBracket"),

            TokenType::LeftSquiggly => write!(f, "LeftSquiggly"),
            TokenType::RightSquiggly => write!(f, "RightSquiggly"),

            TokenType::Let => write!(f, "Let"),
            TokenType::If => write!(f, "If"),
            TokenType::Else => write!(f, "Else"),
            TokenType::For => write!(f, "For"),
            TokenType::While => write!(f, "While"),
            TokenType::Fn => write!(f, "Fn"),
            TokenType::Return => write!(f, "Return"),
            TokenType::Break => write!(f, "Break"),
            TokenType::Continue => write!(f, "Continue"),
            TokenType::Scope => write!(f, "Scope"),
            TokenType::Mut => write!(f, "Mut"),
            TokenType::Struct => write!(f, "Struct"),
            TokenType::Enum => write!(f, "Enum"),

            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Colon => write!(f, "Colon"),
            TokenType::Comma => write!(f, "Comma"),

            TokenType::EndOfFile => write!(f, "EndOfFile"),
        }
    }
}
impl TokenType {
    pub fn from_str(s: &str) -> Option<TokenType> {
        match s {
            "Identifier" => Some(TokenType::Identifier),
            "Number" => Some(TokenType::Number),
            "String" => Some(TokenType::String),
            "True" => Some(TokenType::True),
            "False" => Some(TokenType::False),

            "Equals" => Some(TokenType::Equals),
            "Walrus" => Some(TokenType::Walrus),
            "Plus" => Some(TokenType::Plus),
            "Minus" => Some(TokenType::Minus),
            "Asterisk" => Some(TokenType::Asterisk),
            "Slash" => Some(TokenType::Slash),
            "Modulo" => Some(TokenType::Modulo),

            "PlusEquals" => Some(TokenType::PlusEquals),
            "MinusEquals" => Some(TokenType::MinusEquals),
            "AsteriskEquals" => Some(TokenType::AsteriskEquals),
            "SlashEquals" => Some(TokenType::SlashEquals),
            "ModuloEquals" => Some(TokenType::ModuloEquals),

            "GreaterThan" => Some(TokenType::GreaterThan),
            "LessThan" => Some(TokenType::LessThan),
            "GreaterThanOrEqualTo" => Some(TokenType::GreaterThanOrEqualTo),
            "LessThanOrEqualTo" => Some(TokenType::LessThanOrEqualTo),
            "EqualTo" => Some(TokenType::EqualTo),
            "NotEqualTo" => Some(TokenType::NotEqualTo),
            "LogicalAnd" => Some(TokenType::LogicalAnd),
            "LogicalOr" => Some(TokenType::LogicalOr),

            "Increment" => Some(TokenType::Increment),
            "Decrement" => Some(TokenType::Decrement),
            "Not" => Some(TokenType::Not),

            "LeftBracket" => Some(TokenType::LeftBracket),
            "RightBracket" => Some(TokenType::RightBracket),

            "LeftSquiggly" => Some(TokenType::LeftSquiggly),
            "RightSquiggly" => Some(TokenType::RightSquiggly),

            "Let" => Some(TokenType::Let),
            "If" => Some(TokenType::If),
            "Else" => Some(TokenType::Else),
            "For" => Some(TokenType::For),
            "While" => Some(TokenType::While),
            "Fn" => Some(TokenType::Fn),
            "Return" => Some(TokenType::Return),
            "Break" => Some(TokenType::Break),
            "Continue" => Some(TokenType::Continue),
            "Scope" => Some(TokenType::Scope),
            "Mut" => Some(TokenType::Mut),
            "Struct" => Some(TokenType::Struct),
            "Enum" => Some(TokenType::Enum),

            "Semicolon" => Some(TokenType::Semicolon),
            "Colon" => Some(TokenType::Colon),
            "Comma" => Some(TokenType::Comma),

            "EndOfFile" => Some(TokenType::EndOfFile),

            _ => None,
        }
    }
}
