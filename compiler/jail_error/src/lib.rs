/*
    Jail programming language Copyright (C) 2022-2023 
        - SolindekDev <ytsolindekttv@gmail.com>
*/

use ansi_term::Colour::Red;
use jail_token::*;
use std::process::*;

pub enum ErrorKind {
    UnsupportedCharError,
    CompilerError,
    CodegenError,
    SyntaxError,
    ParserError,
    FileError,
    ArgsError,
}

impl ErrorKind {
    pub fn to_string(&self) -> String {
        match self {
            ErrorKind::UnsupportedCharError => "UnsupportedCharError".to_string(),
            ErrorKind::CompilerError => "CompilerError".to_string(),
            ErrorKind::CodegenError => "CodegenError".to_string(),
            ErrorKind::SyntaxError => "SyntaxError".to_string(),
            ErrorKind::ParserError => "ParserError".to_string(),
            ErrorKind::FileError => "FileError".to_string(),
            ErrorKind::ArgsError => "ArgsError".to_string(),
        }
    }
}

pub fn print_error(error_kind: ErrorKind, message: String, exit_: bool) {
    println!("{}: {}", 
        Red.paint(error_kind.to_string()).to_string(), 
        message);
    if exit_ == true {
        exit(0x00);
    }
}

pub fn print_error_with_pos(error_kind: ErrorKind, message: String, pos: TokenPos, filename: String, exit_: bool) {
    println!("{}: {}:{}:{}: {}", 
        Red.paint(error_kind.to_string()).to_string(), 
        filename,
        pos.row,
        pos.col,
        message);
    if exit_ == true {
        exit(0x00);
    }
}

pub fn print_error_with_line_and_pos(error_kind: ErrorKind, message: String, pos: TokenPos, filename: String, line: String, exit_: bool) {
    println!("{}: {}:{}:{}: {}", 
        Red.paint(error_kind.to_string()).to_string(), 
        filename,
        pos.row,
        pos.col,
        message);
    println!("{} | ...", pos.row - 1);
    println!("{} | {}", pos.row, line);
    println!("{} | ...", pos.row + 1);
    if exit_ == true {
        exit(0x00);
    }
}