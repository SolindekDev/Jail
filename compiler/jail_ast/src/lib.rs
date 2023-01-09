/*
    Jail programming language Copyright (C) 2022-2023 
        - SolindekDev <ytsolindekttv@gmail.com>
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

    // Unsigneds
    U8,
    U16,
    U32,
    U64,

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
            TypesAST::I8   => "i8",
            TypesAST::I16  => "i16",
            TypesAST::I32  => "int",
            TypesAST::I64  => "i64",

            // Floats
            TypesAST::F16  => "f16",
            TypesAST::F32  => "float",
            TypesAST::F64  => "f64",

            // Unsigneds
            TypesAST::U8   => "u8",
            TypesAST::U16  => "u16",
            TypesAST::U32  => "u32",
            TypesAST::U64  => "u64",

            // Other
            TypesAST::STR  => "str",
            TypesAST::BOOL => "bool",
            TypesAST::NONE => "none",
        }.to_string()
    } 
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum NodeKindAST {
    FunctionDeclaration,

    #[default]
    NoneKind,
}

#[derive(Debug, Clone)]
pub struct FunctionArgs {
    pub name: String,
    pub argument_type: TypesAST,
}

#[derive(Debug, Default, Clone)]
pub struct NodeAST {
    // Informations about NodeAST
    pub kind: NodeKindAST,

    // Function declaration kind
    pub func_name: String,
    pub func_args: Vec<FunctionArgs>,
    pub func_body: Vec<NodeAST>,
    pub func_return: TypesAST,
}

impl NodeAST {
    pub fn new(kind: NodeKindAST) -> Self {
        let mut node = NodeAST::default();
        node.kind = kind;
        return node
    }

    // pub fn new_mut(kind: NodeKindAST) -> &'static mut Self {
    //     let mut node: &mut NodeAST = &mut NodeAST::default();
    //     node.kind = kind;
    //     return node.clone()
    // }

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