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
    Scope,
    TypeName,
    Mut,

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
            TokenType::Scope => write!(f, "Scope"),
            TokenType::TypeName => write!(f, "TypeName"),
            TokenType::Mut => write!(f, "Mut"),

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
            "Scope" => Some(TokenType::Scope),
            "TypeName" => Some(TokenType::TypeName),
            "Mut" => Some(TokenType::Mut),

            "Semicolon" => Some(TokenType::Semicolon),
            "Colon" => Some(TokenType::Colon),
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
        mutable: bool,
        var_type: Option<Type>,
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
    Scope {
        body: Vec<Statement>,
    },
    FunctionDefinition {
        identifier: String,
        parameters: Vec<FunctionParameter>,
        body: Vec<Statement>,
        return_type: Type,
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

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum Number {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    F32(f32),
    F64(f64),

    IntegerLiteral(i64),
    FloatLiteral(f64),
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::I8(n) => write!(f, "{n}"),
            Number::I16(n) => write!(f, "{n}"),
            Number::I32(n) => write!(f, "{n}"),
            Number::I64(n) => write!(f, "{n}"),

            Number::U8(n) => write!(f, "{n}"),
            Number::U16(n) => write!(f, "{n}"),
            Number::U32(n) => write!(f, "{n}"),
            Number::U64(n) => write!(f, "{n}"),

            Number::F32(n) => write!(f, "{n}"),
            Number::F64(n) => write!(f, "{n}"),

            Number::IntegerLiteral(n) => write!(f, "{n}"),
            Number::FloatLiteral(n) => write!(f, "{n}"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Number(Number),
    String(String),
    Bool(bool),
    Unit,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Unit => write!(f, "()"),
            Value::Number(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Bool(b) => write!(f, "{b}"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Type {
    // signed ints
    I8,
    I16,
    I32,
    I64,
    // unsigned ints
    U8,
    U16,
    U32,
    U64,
    // floats
    F32,
    F64,
    // others
    String,
    Bool,
    Unit,
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::I8 => write!(f, "i8"),
            Type::I16 => write!(f, "i16"),
            Type::I32 => write!(f, "i32"),
            Type::I64 => write!(f, "i64"),
            Type::U8 => write!(f, "u8"),
            Type::U16 => write!(f, "u16"),
            Type::U32 => write!(f, "u32"),
            Type::U64 => write!(f, "u64"),
            Type::F32 => write!(f, "f32"),
            Type::F64 => write!(f, "f64"),
            Type::String => write!(f, "string"),
            Type::Bool => write!(f, "bool"),
            Type::Unit => write!(f, "unit"),
        }
    }
}
impl Type {
    pub fn from_str(s: &str) -> Option<Type> {
        match s {
            "i8" => Some(Type::I8),
            "i16" => Some(Type::I16),
            "i32" => Some(Type::I32),
            "i64" => Some(Type::I64),
            "u8" => Some(Type::U8),
            "u16" => Some(Type::U16),
            "u32" => Some(Type::U32),
            "u64" => Some(Type::U64),
            "f32" => Some(Type::F32),
            "f64" => Some(Type::F64),
            "string" => Some(Type::String),
            "bool" => Some(Type::Bool),
            "unit" => Some(Type::Unit),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Symbol {
    pub mutable: bool,
    pub var_type: Type,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSymbol {
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Type,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionParameter {
    pub name: String,
    pub param_type: Type,
}
