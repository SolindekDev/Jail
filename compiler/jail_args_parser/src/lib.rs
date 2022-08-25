/*
    Jail programming language
    Copyright (C) 2022 SolindekDev <ytsolindekttv@gmail.com>
*/

use jail_error::*;
use std::env::{Args};

pub struct ArgsParser {
    pub is_filename: bool,
    pub filename: String,
    pub flags: Vec<String>,
}

// This function delete -- or - from start
// of flag e.g before clear_out_flag "--help" 
// after "-help"
pub fn clear_out_flag(mut argument: String) -> String {
    if argument.starts_with("--") {
        argument.remove(0);
        argument.remove(0);
        return argument;
    } else {
        argument.remove(0);
        return argument.to_string();
    }
}

impl ArgsParser {
    // Find flag
    pub fn is_there_flag(&self, flag: &str) -> bool {
        return self.flags
                   .iter()
                   .position(|each| *each == flag.to_string())
                   .is_some();
    }

    pub fn new(argv: Args) -> Self {
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
        for arg in better_args {
            if arg.starts_with("--") || arg.starts_with("-") {
                let flag = clear_out_flag(arg.clone());
                match flag.as_str() {
                    "h" | "he" | "hel" | "help"                                  => flags.push("help".to_string()),
                    "v" | "ve" | "ver" | "vers" | "versi" | "versio" | "version" => flags.push("version".to_string()),
                    _ => { 
                        print_error(ErrorKind::ArgsError, format!("unknown flag \"{}\" in arguments", arg), true);
                    }
                }
            } else {
                if is_filename == true {
                    print_error(ErrorKind::ArgsError, format!("unknown use of \"{}\" in arguments", arg), true);
                } else {
                    is_filename = true;
                    filename = arg;
                }
            }
        }

        // Return
        Self {
            is_filename: is_filename,
            filename: filename.to_string(),
            flags: flags,
        }
    }
}