use serde::{Deserialize, Serialize};
use std::fmt;

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
