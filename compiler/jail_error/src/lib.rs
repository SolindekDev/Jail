use ansi_term::Colour::Red;
use std::process::*;

pub enum ErrorKind {
    SyntaxError,
    FileError,
    ArgsError,
    ParserError,
    CodegenError,
    CompilerError,
}

impl ErrorKind {
    pub fn to_string(&self) -> String {
        match self {
            ErrorKind::SyntaxError => "SyntaxError".to_string(),
            ErrorKind::FileError => "FileError".to_string(),
            ErrorKind::ArgsError => "ArgsError".to_string(),
            ErrorKind::ParserError => "ParserError".to_string(),
            ErrorKind::CodegenError => "CodegenError".to_string(),
            ErrorKind::CompilerError => "CompilerError".to_string(),
        }
    }
}

pub fn print_error(error_kind: ErrorKind, message: String, exit: bool) {
    println!("{}: {}", 
        Red.paint(error_kind.to_string()).to_string(), 
        message);
    if (exit == true) {
        process::exit(0x00);
    }
}