use jail_args_parser::*;
use jail_lex::*;
use std::env;

fn main() {
    let args_parser = ArgsParser::new(env::args());
    println!("{}", args_parser.is_there_flag("help"));
}
