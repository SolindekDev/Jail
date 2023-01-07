/*
    Jail programming language
    Copyright (C) 2022-2023 SolindekDev <ytsolindekttv@gmail.com>
*/

use jail_token::*;
use jail_error::*;
use jail_lex::*;
use jail_ast::*;
use std::process::*;
use ansi_term::Colour::Cyan;

const KEYWORDS: &'static [&str] = &[
    "proc",
    "import",
    "return"
];

// These const represent index of keyword in array 
// above
const KEYWORD_PROC:   i32 = 0;
const KEYWORD_IMPORT: i32 = 1;
const KEYWORD_RETURN: i32 = 2;

pub struct Parser {
    lexer: Lexer,
    index: usize,
    current_token: Token,
    is_error: bool,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            current_token: Token::new(TokenKind::None, "".to_string(), lexer.position.filename.clone(), 0, 0, NumberBase::None),
            lexer: lexer,
            index: 0,
            is_error: false,
        }
    }

    pub fn advance(&mut self, add_index: usize) {
        self.index += add_index;
        self.current_token = self.lexer.tokens[self.index].clone();
    }

    pub fn parse_identifier(&mut self) {
        match self.current_token.value.as_str() {
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