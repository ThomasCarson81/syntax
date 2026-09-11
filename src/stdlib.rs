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
    Abs,
    Min,
    Max,
    Clamp,
    Sqrt,
}

const NUMERIC_TYPES: [Type; 10] = [
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
const NUMERIC_TYPES_NO_UNSIGNED: [Type; 6] = [
    Type::I8,
    Type::I16,
    Type::I32,
    Type::I64,
    Type::F32,
    Type::F64,
];

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
    functions.extend(
        NUMERIC_TYPES
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
        NUMERIC_TYPES
            .iter()
            .copied()
            .flat_map(|return_type| {
                NUMERIC_TYPES
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
        NUMERIC_TYPES
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
    functions.extend(math_functions(functions.len()));

    functions
}

fn math_functions(start_index: usize) -> Vec<Function> {
    let mut math_funcs: Vec<Function> = vec![];

    let abs_index = start_index;
    math_funcs.extend(NUMERIC_TYPES_NO_UNSIGNED.iter().copied().enumerate().map(
        |(index, num_type)| Function {
            id: FunctionId(abs_index + index),
            signature: FunctionSignature {
                name: "abs".into(),
                parameters: vec![num_type],
                return_type: num_type,
            },
            parameters: vec![FunctionParameter {
                name: "value".into(),
                param_type: num_type,
            }],
            implementation: FunctionImplementation::Builtin(BuiltinId::Abs),
        },
    ));

    let min_index = start_index + math_funcs.len();
    math_funcs.extend(
        NUMERIC_TYPES
            .iter()
            .copied()
            .enumerate()
            .map(|(index, num_type)| Function {
                id: FunctionId(min_index + index),
                signature: FunctionSignature {
                    name: "min".into(),
                    parameters: vec![num_type, num_type],
                    return_type: num_type,
                },
                parameters: vec![
                    FunctionParameter {
                        name: "x".into(),
                        param_type: num_type,
                    },
                    FunctionParameter {
                        name: "y".into(),
                        param_type: num_type,
                    },
                ],
                implementation: FunctionImplementation::Builtin(BuiltinId::Min),
            }),
    );

    let max_index = start_index + math_funcs.len();
    math_funcs.extend(
        NUMERIC_TYPES
            .iter()
            .copied()
            .enumerate()
            .map(|(index, num_type)| Function {
                id: FunctionId(max_index + index),
                signature: FunctionSignature {
                    name: "max".into(),
                    parameters: vec![num_type, num_type],
                    return_type: num_type,
                },
                parameters: vec![
                    FunctionParameter {
                        name: "x".into(),
                        param_type: num_type,
                    },
                    FunctionParameter {
                        name: "y".into(),
                        param_type: num_type,
                    },
                ],
                implementation: FunctionImplementation::Builtin(BuiltinId::Max),
            }),
    );

    let clamp_index = start_index + math_funcs.len();
    math_funcs.extend(
        NUMERIC_TYPES
            .iter()
            .copied()
            .enumerate()
            .map(|(index, num_type)| Function {
                id: FunctionId(clamp_index + index),
                signature: FunctionSignature {
                    name: "clamp".into(),
                    parameters: vec![num_type, num_type, num_type],
                    return_type: num_type,
                },
                parameters: vec![
                    FunctionParameter {
                        name: "value".into(),
                        param_type: num_type,
                    },
                    FunctionParameter {
                        name: "min".into(),
                        param_type: num_type,
                    },
                    FunctionParameter {
                        name: "max".into(),
                        param_type: num_type,
                    },
                ],
                implementation: FunctionImplementation::Builtin(BuiltinId::Clamp),
            }),
    );

    let sqrt_index = start_index + math_funcs.len();
    math_funcs.push(Function {
        id: FunctionId(sqrt_index),
        signature: FunctionSignature {
            name: "sqrt".into(),
            parameters: vec![Type::F32],
            return_type: Type::F32,
        },
        parameters: vec![FunctionParameter {
            name: "value".into(),
            param_type: Type::F32,
        }],
        implementation: FunctionImplementation::Builtin(BuiltinId::Sqrt),
    });
    math_funcs.push(Function {
        id: FunctionId(sqrt_index + 1),
        signature: FunctionSignature {
            name: "sqrt".into(),
            parameters: vec![Type::F64],
            return_type: Type::F64,
        },
        parameters: vec![FunctionParameter {
            name: "value".into(),
            param_type: Type::F64,
        }],
        implementation: FunctionImplementation::Builtin(BuiltinId::Sqrt),
    });

    math_funcs
}
