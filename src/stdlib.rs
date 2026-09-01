use crate::{
    functions::{
        BuiltinId, Function, FunctionId, FunctionImplementation, FunctionParameter,
        FunctionSignature,
    },
    types::Type,
};

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
    functions
}
