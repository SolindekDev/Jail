/*
    Jail programming language
    Copyright (C) 2022-2023 SolindekDev <ytsolindekttv@gmail.com>
*/

use std::vec::Vec;

#[derive(Clone, PartialEq, Debug, Default)]
pub enum TypesAST {
    // Ints
    I8,
    I16,
    I32,
    I64,

    // Floats
    F16,
    F32,
    F64,

    // Other
    STR,
    BOOL,

    // None
    #[default]
    NONE,
}

impl TypesAST {
    pub fn get_pretty(&self) -> String {
        match self {
            // Ints
            TypesAST::I8 => "i8".to_string(),
            TypesAST::I16 => "i16".to_string(),
            TypesAST::I32 => "int".to_string(),
            TypesAST::I64 => "i64".to_string(),

            // Floats
            TypesAST::F16 => "f16".to_string(),
            TypesAST::F32 => "float".to_string(),
            TypesAST::F64 => "f64".to_string(),

            // Other
            TypesAST::STR => "str".to_string(),
            TypesAST::BOOL => "bool".to_string(),
            TypesAST::NONE => "none".to_string(),
        }
    } 
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum NodeKindAST {
    FunctionDeclaration,

    #[default]
    NoneKind,
}

#[derive(Debug)]
pub struct FunctionArgs {
    pub name: String,
    pub argument_type: TypesAST,
}

#[derive(Debug, Default)]
pub struct NodeAST {
    // Informations about NodeAST
    pub kind: NodeKindAST,

    // Function declaration kind
    pub func_name: String,
    pub func_args: Vec<FunctionArgs>,
    pub func_body: Vec<NodeKindAST>,
    pub func_return: TypesAST,
}

impl NodeAST {
    pub fn new(kind: NodeKindAST) -> Self {
        let mut node = NodeAST::default();
        node.kind = kind;
        return node
    }

    pub fn copy(self) -> Self {
        return self;
    }
}

impl FunctionArgs {
    pub fn new(name: String, argument_type: TypesAST) -> Self {
        FunctionArgs{
            name,
            argument_type
        }
    }

    pub fn copy(self) -> Self {
        return self;
    }
}