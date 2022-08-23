use ansi_term::Colour::Red;
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

/*
    Multiline comment

    10 + 200 * 3000 / 40000 % 500000
    1.9 + 2. * 3.000 / 4.310 % 5.10
    import "jail/std"
    println("Hello, World!")
    fn main() {

    }
    // Comment
*/