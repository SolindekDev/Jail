/*
    Jail programming language
    Copyright (C) 2022-2023 SolindekDev <ytsolindekttv@gmail.com>
*/

use ansi_term::Colour::{ Yellow };

use std::env::*;
use std::env::consts::*;
use std::process::*;
use std::path::*;
use std::fs;

use jail_args_parser::*;
use jail_lex::*;
use jail_token::*;
use jail_error::*;

const VERSION_OF_COMPILER: &str = "1.0v";

fn help() {
    println!("{}:\n   {}:\n        -h, --help - Write down this message\n        -v, --version - Show version of jail compiler\n        -l, --lexer - Show all tokens that have been generated from lexer\n\n{}: jail [--version] [--help] [options] [main.ja]", Yellow.paint("Help").to_string(), Yellow.paint("Flags").to_string(), Yellow.paint("Usage").to_string());
    exit(0x00);
}

fn version() {
    println!("{} ({}) {}", Yellow.paint("jail").to_string(), ARCH, VERSION_OF_COMPILER);
    exit(0x00);
}

fn main() {
    let args_parser = ArgsParser::new(args());
    
    if args_parser.is_there_flag("help") {
        help();
    } else if args_parser.is_there_flag("version") {
        version();
    }

    if args_parser.is_filename == true {
        let path = Path::new(&args_parser.filename);
        if path.exists() {
            if path.is_dir() == false {
                let value = fs::read_to_string(&args_parser.filename)
                    .expect("file not found!");
                
                // Lexer
                let lexer_flag = if args_parser.is_there_flag("lexer") == true { 
                    args_parser.is_there_flag("lexer") 
                } else {
                    args_parser.is_there_flag("l")
                };

                let mut lexer = Lexer::new(value, args_parser.filename.clone());
                lexer.start(lexer_flag);

                // print_error_with_line_and_pos(
                //     ErrorKind::SyntaxError, "This token can't be used in this expression".to_string(), 
                //     TokenPos::new(1402, 102), args_parser.filename.clone(), 
                //     "       println(\"x + y = {}\", z)".to_string(), true)

                // Parser
                let mut parser = Parser::new(lexer);
                parser.start();
            } else {
                print_error(ErrorKind::FileError, format!("\"{}\" is a directory", &args_parser.filename), true);
            }
        } else {
            print_error(ErrorKind::FileError, format!("\"{}\" do not exists", &args_parser.filename), true);
        }
    } else {
        help();
    }
}
