use crate::{ast::Statement, stdlib::BuiltinId, types::Type};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionParameter {
    pub name: String,
    pub param_type: Type,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionSignature {
    pub name: String,
    pub parameters: Vec<Type>,
    pub return_type: Type,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FunctionId(pub usize);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FunctionImplementation {
    Oxide(FunctionBody),
    Builtin(BuiltinId),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub id: FunctionId,
    pub signature: FunctionSignature,
    pub parameters: Vec<FunctionParameter>,
    pub implementation: FunctionImplementation,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionTable {
    pub functions: Vec<Function>,
}
impl FunctionTable {
    pub fn get(&self, id: &FunctionId) -> Option<&Function> {
        self.functions.iter().find(|f| f.id == *id)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Function> {
        self.functions
            .iter()
            .find(|function| function.signature.name == name)
    }

    pub fn functions_named(&self, name: &str) -> impl Iterator<Item = &Function> {
        self.functions
            .iter()
            .filter(move |function| function.signature.name == name)
    }

    pub fn has_function_with_parameters(&self, name: &str, parameters: &[Type]) -> bool {
        self.functions.iter().any(|function| {
            function.signature.name == name && function.signature.parameters == parameters
        })
    }

    pub fn has_function(&self, signature: FunctionSignature) -> bool {
        self.functions
            .iter()
            .any(|function| function.signature == signature)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionBody {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckedProgram {
    pub function_table: FunctionTable,
    pub globals: Vec<Statement>,
    pub main_function_id: FunctionId,
}
