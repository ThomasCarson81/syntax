use crate::{
    functions::{FunctionId, FunctionParameter},
    tokens::TokenType,
    types::{Type, Value},
};
use serde::{Deserialize, Serialize};

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
        inferred_type: Option<Type>,
    },
    UnaryOp {
        operator: TokenType,
        operand: Box<Expression>,
        inferred_type: Option<Type>,
    },
    UnresolvedFunctionCall {
        identifier: String,
        arguments: Vec<Expression>,
    },
    ResolvedFunctionCall {
        function_id: FunctionId,
        arguments: Vec<Expression>,
    },
}
