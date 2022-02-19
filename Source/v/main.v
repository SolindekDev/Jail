/*
    Jail Programming Language Copyright (C) 2022 SolindekDev

	Contribuitors:
		https://github.com/SolindekDev/Jail/edit/main/contributors.md
*/

module main

/* Import some packages */
import os
import time
import json
import lexer
import ast
import compiler

struct ConfigurationJSON {
	help_command     string
	version_command  string
}

fn main() {
	configuration_json_content := '{\n
    "help_command": "Help command",
    "version_command": "Version command"
}'

	if os.args.len < 2 {
		/*
			TODO: Shell
		*/
		println("TODO: Shell")
	} else {
		decode := json.decode(ConfigurationJSON, configuration_json_content)?

		if os.args[1] == "help" { // Help command
			println(decode.help_command)
			exit(0) // Success code
		} else if os.args[1] == "version" { // Version command
			println(decode.version_command)
			exit(0) // Success code
		}
		/* Open file */
		mut f := os.open(os.args[1]) or {
			println("ShellError: File not found or it not exists..")
			exit(1)
		}

		if f.is_opened == true { // If file is opened
			mut buf := []string{} // Buffer of the file
			buf = os.read_lines(os.args[1]) ? // Read lines from file
			content := buf.join("\n") // Join buffer \n

			start_time := time.now()

			/* If flag -time exists */
			if os.args.contains("-time") == true {
				println("Start Time:                      | " + start_time.format_ss_milli())
			}

			/*
				Compiling
			*/
			tokens   := lexer.lexer(content, os.args[1]) // Init lexer
			main_ast := ast.ast_init(tokens) // Init AST
			compiler.compiler_init(
				main_ast,
				os.args.contains("-o")
			) // Init compiler

			end_time := time.now()

			/* If flag -time exists */
			if os.args.contains("-time") == true {
				println("End Time:                        | " + end_time.format_ss_milli())
			}
		} else {
			println("ShellError: Not enough permissions to open this file!") // Some error
		}
	}
}
