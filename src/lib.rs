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
    True,
    False,

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

    // Ending Tokens
    Semicolon,
    Comma,
    EndOfFile,
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::Number => write!(f, "Number"),
            TokenType::True => write!(f, "True"),
            TokenType::False => write!(f, "False"),

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

            TokenType::Semicolon => write!(f, "Semicolon"),
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
            "True" => Some(TokenType::True),
            "False" => Some(TokenType::False),

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

            "Semicolon" => Some(TokenType::Semicolon),
            "Comma" => Some(TokenType::Comma),

            "EndOfFile" => Some(TokenType::EndOfFile),

            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Let {
        identifier: String,
        expression: Expression,
    },
    Assignment {
        identifier: String,
        expression: Expression,
    },
    If {
        condition: Expression,
        body: Vec<Statement>,
        else_block: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        initialiser: Option<Box<Statement>>,
        condition: Option<Expression>,
        increment: Option<Box<Statement>>,
        body: Vec<Statement>,
    },
    FunctionDefinition {
        identifier: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    ExpressionStatement {
        expression: Expression,
    },
    Return {
        value: Option<Expression>,
    },
    Break,
    Continue,
    Pass,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    Literal(Value),
    Identifier(String),
    BinaryOp {
        left: Box<Expression>,
        operator: TokenType,
        right: Box<Expression>,
    },
    UnaryOp {
        operator: TokenType,
        operand: Box<Expression>,
    },
    FunctionCall {
        identifier: String,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Unit,
}
impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Number(n) => *n > 0.0,
            Value::Bool(b) => *b,
            Value::Unit => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Unit => write!(f, "()"),
            Value::Number(n) => write!(f, "{n}"),
            Value::Bool(b) => write!(f, "{b}"),
        }
    }
}
