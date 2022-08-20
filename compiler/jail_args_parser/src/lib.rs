use jail_error::*;
use std::env::{args, Args};
use std::vec;

pub struct ArgsParser {
    pub is_filename: bool,
    pub filename: String,
    pub flags: Vec<String>,
}

impl ArgsParser {
    pub fn clear_out_flag(&self, argument: String) -> String {

    }

    pub fn new(mut argv: Args) -> Self {
        // Some important variables in args parser
        let mut flags: Vec<String> = Vec::new();             // Array of flags e.g this can look like this ["--help"]
        let mut better_args: Vec<String> = argv.collect();   // We don't want Args type we want a String vector
        let mut is_filename: bool = false;                   // Do we already get the filename
        let mut filename: String = "".to_string();           // Value of the file name

        // Remove first element of array "better_args" because first 
        // element is a name of executing file i mean when we are 
        // writting into terminal "jail main.ja" in array this will
        // be ["jail", "main.ja"], so we don't want to "jail" be in
        // array. Why i'am so sure that the list have almost 1 element
        // that's easy always std::env::Args have almost 1 element
        better_args.remove(0);

        // Loop by every element of array "better_args"
        for mut arg in better_args {
            if arg.starts_with("--") || arg.starts_with("-") {
                let flag = clear_out_flag(arg);
            } else {
                if is_filename == true {
                    print_error(ErrorKind::ArgsError, format!("unknown use of {} in arguments", arg))
                } else {
                    is_filename = true;
                    filename = arg;
                }
            }
        }

        println!("{}", filename);

        // Return
        ArgsParser{
            is_filename: is_filename,
            filename: filename.to_string(),
            flags: flags,
        }
    }
}