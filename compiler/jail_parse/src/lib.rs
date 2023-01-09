/*
    Jail programming language Copyright (C) 2022-2023 
        - SolindekDev <ytsolindekttv@gmail.com>
*/

use jail_token::*;
use jail_error::*;
use jail_lex::*;
use jail_ast::*;

use std::process::*;
use std::ptr::null;

const KEYWORD_FUNCTION:  &str = "proc";
// const KEYWORD_IMPORT:    &str = "import";
// const KEYWORD_RETURN:    &str = "return";

pub struct Parser{
    /* Essential fields */
    lexer: Lexer,
    index: usize,
    current_token: Token,
    is_error: bool,
    lines: Vec<String>,
    
    /* AST Nodes */
    nodes: Vec<NodeAST>,

    /* Function fields */
    is_function_opened: bool,
    curr_function: *const NodeAST,
    functions_names: Vec<String>
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            /* Essential fields */
            current_token: Token::new(TokenKind::None, "".to_string(), lexer.position.filename.clone(), 0, 0, NumberBase::None),
            lexer: lexer.clone(),
            index: 0,
            is_error: false,
            lines: lexer.data.lines().map(|s| s.to_string()).collect(),
                
            /* AST Nodes */
            nodes: vec![],

            /* Function fields */
            is_function_opened: false,
            curr_function: null(),
            functions_names: vec![]
        }
    }

    pub fn advance(&mut self, add_index: usize) {
        self.index += add_index;
        self.current_token = self.lexer.tokens[self.index].clone();
    }

    pub fn function_is_opened(&mut self) {
        if self.is_function_opened == true {
            self.is_error = true; print_error_with_line_and_pos(
                ErrorKind::SyntaxError, 
                "function need to be close to use this expression".to_string(),
                TokenPos {
                    row: self.current_token.pos.row,
                    col: self.current_token.pos.col,
                }, self.lexer.position.filename.clone(), 
                self.lines[(self.current_token.pos.row - 1) as usize].to_string(), true)
        }
    }

    pub fn function_is_not_opened(&mut self) {
        if self.is_function_opened == false {
            self.is_error = true; print_error_with_line_and_pos(
                ErrorKind::SyntaxError, 
                "function need to be opened to use this expression".to_string(),
                TokenPos {
                    row: self.current_token.pos.row,
                    col: self.current_token.pos.col,
                }, self.lexer.position.filename.clone(), 
                self.lines[(self.current_token.pos.row - 1) as usize].to_string(), true)
        }
    }

    pub fn convert_token_to_typeast(&mut self, val: &str) -> TypesAST {
        match val {
            /* Ints */
            "i8"     => TypesAST::I8,
            "i16"    => TypesAST::I16,
            "i32"    => TypesAST::I32,
            "i64"    => TypesAST::I64,
            "int"    => TypesAST::I32,

            /* Floats */
            "f16"    => TypesAST::F16,
            "f32"    => TypesAST::F32,
            "f64"    => TypesAST::F64,
            "float"  => TypesAST::F32,

            /* Other */
            "str"    => TypesAST::STR,
            "bool"   => TypesAST::BOOL,

            /* None */
            _        => TypesAST::NONE
        }
    }

    pub fn parse_arguments_function(&mut self) -> Vec<FunctionArgs> {
        let mut to_ret: Vec<FunctionArgs> = vec![];

        /* 
            State:
                0 - expecting name
                1 - expecting type
                2 - expecting comma
        */
        let mut state: i32 = 0;

        loop {
            match state {
                0 => {
                    // Get next token that will be the argument name
                    self.is_next_token(TokenKind::Identifier);
                    let argument: FunctionArgs = FunctionArgs::new(
                        self.current_token.value.clone(), TypesAST::NONE);

                    // Push argument to array of arguments and set the state to `expecting type`
                    // that is equalation of 1
                    to_ret.push(argument);
                    state = 1;
                }, 
                1 => {
                    // Get the last element of `to_ret` variable and convert token to 
                    // `TypesAST` enumerator 
                    self.advance(1);
                    let mut last_argument: &mut FunctionArgs = to_ret.last_mut().unwrap();
                    let type_jail: TypesAST = self.convert_token_to_typeast(self.current_token.value.clone().as_str());

                    if type_jail == TypesAST::NONE {
                        self.is_error = true; print_error_with_line_and_pos(
                            ErrorKind::SyntaxError, 
                            format!("unexpected type that is not defined {}", self.current_token.value.clone()),
                            TokenPos { row: self.current_token.pos.row, col: self.current_token.pos.col, }, 
                            self.lexer.position.filename.clone(), self.lines[(self.current_token.pos.row - 1) as usize].to_string(), true)
                    }

                    // Set type_jail to last item of `to_ret` array and set state to 2, so
                    // we expect comma
                    last_argument.argument_type = type_jail;
                    state = 2;
                },
                2 => {
                    // Is next token an comma `,` then set state to `expecting name` that is 
                    // equalation of 0 if it's not an comma `,` then break loop
                    self.advance(1); 

                    if self.current_token.kind != TokenKind::Comma { break }
                    else { state = 0 }
                },
                _ => unimplemented!(),
            }
        }   

        return to_ret;
    }

    pub fn is_next_token(&mut self, token_kind: TokenKind) {
        self.advance(1); 
        if self.current_token.kind != token_kind {
            self.is_error = true; print_error_with_line_and_pos(
                ErrorKind::SyntaxError, 
                format!("expected `{}` not an `{}` type",
                    token_kind.get_pretty(),
                    self.current_token.kind.get_pretty()),
                TokenPos {
                    row: self.current_token.pos.row,
                    col: self.current_token.pos.col,
                }, self.lexer.position.filename.clone(), 
                self.lines[(self.current_token.pos.row - 1) as usize].to_string(), true)
        }
    }

    pub fn parse_function_declaration(&mut self) {
        // Check is function close to use parser function declaration expression
        self.function_is_opened();
        self.is_next_token(TokenKind::Identifier);

        // Save this name here so we can access it in the future
        let func_name: String = self.current_token.value.clone();
        self.is_next_token(TokenKind::LeftParen);
        let is_func_exist = self.functions_names
                                .iter()
                                .enumerate()
                                .find(|&r| r.1.to_string() == func_name)
                                .is_none();
        if is_func_exist == false {
            self.is_error = true; print_error_with_line_and_pos(
                ErrorKind::SyntaxError, 
                format!("function with name `{}` already exists", func_name),
                TokenPos {
                    row: self.current_token.pos.row,
                    col: self.current_token.pos.col,
                }, self.lexer.position.filename.clone(), 
                self.lines[(self.current_token.pos.row - 1) as usize].to_string(), true)
        }

        // Call function `parse_arguments_function` which will parse
        // arguments and return it by Vec<FunctionArgs>
        let func_args: Vec<FunctionArgs> = self.parse_arguments_function();

        // Get the function return type if there isn't any type to return
        // just skip it and set the variable func_return to TypesAST::NONE
        self.advance(1);
        let func_return: TypesAST = if self.current_token.kind == TokenKind::Arrow {
            self.is_next_token(TokenKind::Identifier);
            self.convert_token_to_typeast(self.current_token.value.clone().as_str())
        } else { TypesAST::NONE };
        
        // Create and push node
        let mut node: NodeAST = NodeAST::new(NodeKindAST::FunctionDeclaration);
        node.func_name   = func_name;
        node.func_args   = func_args;
        node.func_body   = vec![];
        node.func_return = func_return;
        self.nodes.push(node);

        // Set important values
        self.is_function_opened = true;
        self.curr_function = self.nodes.last().unwrap();

        self.is_next_token(TokenKind::LeftCurlyBrackets);

        self.advance(1);
    }

    pub fn parse_identifier(&mut self) {
        match self.current_token.value.as_str() {
            KEYWORD_FUNCTION => self.parse_function_declaration(),
            _ => unimplemented!()
        }
    }

    pub fn is_error_exit(&mut self) {
        if self.is_error {
            exit(0x01);
        }
    }

    pub fn start(&mut self) {
        while self.index < self.lexer.tokens.len() {
            self.advance(0);
            match self.current_token.kind {
                TokenKind::Identifier => self.parse_identifier(),
                TokenKind::Newline => {},
                TokenKind::RightCurlyBrackets => {
                    if self.is_function_opened != true { 
                        self.is_error = true; print_error_with_line_and_pos(
                            ErrorKind::SyntaxError, 
                            format!("unexpected use of `{}` in this expression", 
                                self.current_token.kind.get_pretty()),
                            TokenPos {
                                row: self.current_token.pos.row,
                                col: self.current_token.pos.col,
                            }, self.lexer.position.filename.clone(), 
                            self.lines[(self.current_token.pos.row - 1) as usize].to_string(), true)
                    }
                    self.is_function_opened = false; 
                    self.advance(1);
                }
                TokenKind::Eof => { 
                    break;
                },
                _ => {
                    self.is_error = true; print_error_with_line_and_pos(
                        ErrorKind::ParserError,
                        format!("unexpected use of `{}` in this expression", 
                            self.current_token.kind.get_pretty()),
                            TokenPos {
                                row: self.current_token.pos.row,
                                col: self.current_token.pos.col,
                            }, self.lexer.position.filename.clone(), 
                            self.lines[(self.current_token.pos.row - 1) as usize].to_string(), true)
                }
            }
        }

        self.is_error_exit();
    }
}