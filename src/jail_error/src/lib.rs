use colored::Colorize;

pub enum ErrorKind {
    SyntaxError,
    FileError,
    ParserError,
    CodegenError,
    CompilerError,
}

impl ErrorKind {
    pub fn to_string(&self) -> String {
        match self {
            ErrorKind::SyntaxError => "SyntaxError".to_string(),
            ErrorKind::FileError => "FileError".to_string(),
            ErrorKind::ParserError => "ParserError".to_string(),
            ErrorKind::CodegenError => "CodegenError".to_string(),
            ErrorKind::CompilerError => "CompilerError".to_string(),
        }
    }
}

pub fn print_error(error_kind: ErrorKind, message: String) {
    println!("{}: {}", error_kind.to_string(), message)
}