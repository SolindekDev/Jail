use jail_args_parser::*;
use jail_lex::*;
use std::env::*;
use std::env::consts::*;
use std::process::*;
use ansi_term::Colour::{ Yellow };

const VERSION_OF_COMPILER: &str = "1.0v";

fn help() {
    println!("{}:\n   {}:\n        -h, --help - Write down this message\n        -v, --version - Show version of jail compiler\n\n{}: jail [--version] [--help] [options] [main.ja]", Yellow.paint("Help").to_string(), Yellow.paint("Flags").to_string(), Yellow.paint("Usage").to_string());
    exit(0x00);
}

fn version() {
    println!("{} ({}) {}", Yellow.paint("jail").to_string(), ARCH, VERSION_OF_COMPILER);
    exit(0x00);
}

fn main() {
    let args_parser = ArgsParser::new(args());
    
    if args_parser.is_there_flag("version") {
        version();
    } else if args_parser.is_there_flag("help") {
        help();
    }

    if args_parser.is_filename == true {
        // Start compilation process        
    } else {
        help();
    }
}
