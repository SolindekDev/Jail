use jail_args_parser::*;
use jail_lex::*;
use std::env;

fn main() {
    let _args_parser = ArgsParser::new(env::args());
}
