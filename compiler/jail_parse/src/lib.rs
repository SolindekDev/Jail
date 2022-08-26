/*
    Jail programming language
    Copyright (C) 2022 SolindekDev <ytsolindekttv@gmail.com>
*/

use jail_token::*;
use jail_error::*;
use jail_lex::*;
use jail_ast::*;
use std::process::*;
use ansi_term::Colour::Blue;

const KEYWORD_IMPORT: &str = "import";

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

    pub fn parse_import(&mut self) {
        self.advance(1);
        if self.current_token.kind == TokenKind::StringLiteral {
            let package_name: String = self.current_token.value.clone();
            if package_name.starts_with("jail/") {
                let mut package_built_in: Vec<&str> = package_name
                    .split("jail/")
                    .collect();
                package_built_in.remove(0);

                if package_built_in.len() == 1 {
                    println!("{:?}, {}", package_built_in, package_built_in.len());
                } else {
                    print_error_with_pos(
                        ErrorKind::SyntaxError,
                        format!("unknown package name `{}`",
                            self.current_token.value.clone()),
                        self.current_token.pos.clone(),
                        self.current_token.filename.clone(),
                        false
                    );
                    self.is_error = true;
                    self.advance(1);
                }
            } else {
                // Import local file
            }
            self.advance(1);
        } else {
            print_error_with_pos(
                ErrorKind::SyntaxError,
                format!("unexpected use of `{}` after `import` keyword. {}",
                    self.current_token.kind.get_pretty(),
                    Blue.paint("help: expected string that have value of file to import").to_string()),
                self.current_token.pos.clone(),
                self.current_token.filename.clone(),
                false
            );
            self.is_error = true;
            self.advance(1);
        }
    }

    pub fn parse_identifier(&mut self) {
        match self.current_token.value.as_str() {
            KEYWORD_IMPORT => self.parse_import(),
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