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

    pub fn parse_arguments_function(&mut self) -> Vec<FunctionArgs> {
        let mut to_ret: Vec<FunctionArgs> = vec![];

        while true {
            break;
        }   

        return to_ret;
    }

    pub fn is_next_token(&mut self, token_kind: TokenKind) {
        self.advance(1); 
        if self.current_token.kind != TokenKind::Identifier {
            self.is_error = true; print_error_with_line_and_pos(
                ErrorKind::SyntaxError, 
                format!("expected `{}` not an `{}` type",
                    token_kind.get_pretty()
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
        let args: Vec<FunctionArgs> = self.parse_arguments_function();
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