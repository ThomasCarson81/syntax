use crate::{
    functions::{BuiltinId, Function, FunctionId, FunctionImplementation, FunctionSignature},
    types::Type,
};

pub fn builtin_functions() -> Vec<Function> {
    vec![
        Function {
            id: FunctionId(0),
            signature: FunctionSignature {
                name: "print".into(),
                parameters: vec![Type::String],
                return_type: Type::Unit,
            },
            implementation: FunctionImplementation::Builtin(BuiltinId::Print),
        },
        Function {
            id: FunctionId(1),
            signature: FunctionSignature {
                name: "println".into(),
                parameters: vec![Type::String],
                return_type: Type::Unit,
            },
            implementation: FunctionImplementation::Builtin(BuiltinId::Println),
        },
    ]
}
