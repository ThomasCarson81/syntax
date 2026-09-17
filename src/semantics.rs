use crate::{functions::FunctionParameter, types::Type};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub mutable: bool,
    pub var_type: Type,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSymbol {
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Type,
}
