/*
    Jail programming language Copyright (C) 2022-2023 
        - SolindekDev <ytsolindekttv@gmail.com>
*/

use jail_token::*;
use jail_error::*;
use jail_lex::*;
use jail_ast::*;
use std::process::*;
use ansi_term::Colour::Cyan;

const KEYWORD_FUNCTION:  &str = "proc";
const KEYWORD_IMPORT:    &str = "import";
const KEYWORD_RETURN:    &str = "return";

pub struct Parser {
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

    pub fn convert_token_to_typeast(&mut self, token: Token) -> TypesAST {
        // TODO: implement this function just convert token.value to TypesAST kind
        //       for now just return i32 (int)
        return TypesAST::I32;
    }

    pub fn parse_arguments_function(&mut self) -> Vec<FunctionArgs> {
        // TODO: repaire it so it work on file `./tests/function_parse_start_02.ja`
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
                    let mut last_argument: &mut FunctionArgs = to_ret.last_mut().unwrap();
                    let type_jail: TypesAST = self.convert_token_to_typeast(self.current_token.clone());
                    self.advance(1);

                    // Set type_jail to last item of `to_ret` array and set state to 2, so
                    // we expect comma
                    last_argument.argument_type = type_jail;
                    state = 2;
                },
                2 => {
                    // Is next token an comma `,` then set state to `expecting name` that is 
                    // equalation of 0

                    // TODO: don't make it loop forever if it's not an comma break
                    self.is_next_token(TokenKind::Comma);
                    state = 0;

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

        // Skip keyword token and get the function name
        self.is_next_token(TokenKind::Identifier);

        // Save this name here so we can access it in the future
        let func_name: String = self.current_token.value.clone();

        // Check is next token an left parent `(`
        self.is_next_token(TokenKind::LeftParen);

        // Call function `parse_arguments_function` which will parse
        // arguments and return it by Vec<FunctionArgs>
        let _args: Vec<FunctionArgs> = self.parse_arguments_function();
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
                TokenKind::Eof => { 
                    break;
                },
                _ => {
                    print_error_with_pos(
                        ErrorKind::ParserError,
                        format!("unimplemented parser token kind `{}`", 
                            self.current_token.kind.get_pretty()),
                        self.current_token.pos.clone(),
                        self.current_token.filename.clone(),
                        true
                    );
                }
            }
        }

        self.is_error_exit();
    }
}