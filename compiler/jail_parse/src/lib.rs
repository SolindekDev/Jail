/*
    Jail programming language
    Copyright (C) 2022 SolindekDev <ytsolindekttv@gmail.com>
*/

use jail_token::*;
use jail_error::*;
use jail_lex::*;
use jail_ast::*;

pub struct Parser {
    lexer: Lexer,
    index: usize
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer: lexer,
            index: 0,
            last_token: Option<Token>,
            next_token: Option<Token>,
        }
    }

    pub fn get_last_token(&self) {

    }

    pub fn get_next_token(&self) {

    }

    pub fn advnace(&self) {
        self.last_token = self.get_last_token();
        self.next_token = self.get_next_token();
    }

    pub fn start(&self) {

    }
}