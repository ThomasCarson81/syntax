use crate::{
    functions::{
        Function, FunctionId, FunctionImplementation, FunctionParameter, FunctionSignature,
    },
    types::Type,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuiltinId {
    Print,
    Println,
    ToString,
    Input,
    ToNum(Type),
    Time,
}

pub fn builtin_functions() -> Vec<Function> {
    let mut functions = vec![
        Function {
            id: FunctionId(0),
            signature: FunctionSignature {
                name: "print".into(),
                parameters: vec![Type::String],
                return_type: Type::Unit,
            },
            parameters: vec![FunctionParameter {
                name: "value".into(),
                param_type: Type::String,
            }],
            implementation: FunctionImplementation::Builtin(BuiltinId::Print),
        },
        Function {
            id: FunctionId(1),
            signature: FunctionSignature {
                name: "println".into(),
                parameters: vec![Type::String],
                return_type: Type::Unit,
            },
            parameters: vec![FunctionParameter {
                name: "value".into(),
                param_type: Type::String,
            }],
            implementation: FunctionImplementation::Builtin(BuiltinId::Println),
        },
    ];
    let numeric_types = [
        Type::I8,
        Type::I16,
        Type::I32,
        Type::I64,
        Type::U8,
        Type::U16,
        Type::U32,
        Type::U64,
        Type::F32,
        Type::F64,
    ];
    functions.extend(
        numeric_types
            .into_iter()
            .enumerate()
            .map(|(index, param_type)| Function {
                id: FunctionId(index + 2),
                signature: FunctionSignature {
                    name: "to_string".into(),
                    parameters: vec![param_type],
                    return_type: Type::String,
                },
                parameters: vec![FunctionParameter {
                    name: "value".into(),
                    param_type,
                }],
                implementation: FunctionImplementation::Builtin(BuiltinId::ToString),
            }),
    );
    let input_id = functions.len();
    functions.push(Function {
        id: FunctionId(input_id),
        signature: FunctionSignature {
            name: "input".into(),
            parameters: vec![],
            return_type: Type::String,
        },
        parameters: vec![],
        implementation: FunctionImplementation::Builtin(BuiltinId::Input),
    });
    functions.push(Function {
        id: FunctionId(input_id + 1),
        signature: FunctionSignature {
            name: "input".into(),
            parameters: vec![Type::String],
            return_type: Type::String,
        },
        parameters: vec![FunctionParameter {
            name: "message".into(),
            param_type: Type::String,
        }],
        implementation: FunctionImplementation::Builtin(BuiltinId::Input),
    });

    let num_to_num_id = functions.len();

    functions.extend(
        numeric_types
            .iter()
            .copied()
            .flat_map(|return_type| {
                numeric_types
                    .iter()
                    .copied()
                    .map(move |param_type| (param_type, return_type))
            })
            .enumerate()
            .map(|(index, (param_type, return_type))| Function {
                id: FunctionId(num_to_num_id + index),
                signature: FunctionSignature {
                    name: format!("to_{}", return_type),
                    parameters: vec![param_type],
                    return_type,
                },
                parameters: vec![FunctionParameter {
                    name: "value".into(),
                    param_type,
                }],
                implementation: FunctionImplementation::Builtin(BuiltinId::ToNum(return_type)),
            }),
    );

    let string_to_num_id = functions.len();

    functions.extend(
        numeric_types
            .iter()
            .copied()
            .enumerate()
            .map(|(index, return_type)| Function {
                id: FunctionId(string_to_num_id + index),
                signature: FunctionSignature {
                    name: format!("to_{}", return_type),
                    parameters: vec![Type::String],
                    return_type,
                },
                parameters: vec![FunctionParameter {
                    name: "value".into(),
                    param_type: Type::String,
                }],
                implementation: FunctionImplementation::Builtin(BuiltinId::ToNum(return_type)),
            }),
    );

    functions.push(Function {
        id: FunctionId(functions.len()),
        signature: FunctionSignature {
            name: "time".into(),
            parameters: vec![],
            return_type: Type::F64,
        },
        parameters: vec![],
        implementation: FunctionImplementation::Builtin(BuiltinId::Time),
    });

    functions
}
