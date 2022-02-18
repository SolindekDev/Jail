/*
    Jail Programming Language Copyright (C) 2022 SolindekDev

	Contribuitors:
		https://github.com/SolindekDev/Jail/edit/main/contributors.md
*/

module main

/* Import some packages */
import os
import time
import lexer
import ast
import compiler

fn main() {
	if os.args.len < 2 {
		/*
			TODO: Shell
		*/
		println("TODO: Shell")
	} else {
		/* Open file */
		mut f := os.open(os.args[1]) or {
			println("ShellError: File not found or it not exists..")
			exit(1)
		}

		if f.is_opened == true { // If file is opened
			mut buf := []string{} // Buffer of the file
			buf = os.read_lines(os.args[1]) ? // Read lines from file
			content := buf.join("\n") // Join buffer \n

			/* If flag -time exists */
			if os.args.contains("-time") == true {
				println("Start Time: " + time.now().format_ss_milli())
			}
			tokens   := lexer.lexer(content, os.args[1]) // Init lexer
			main_ast := ast.ast_init(tokens) // Init AST
			compiler.compiler_init(
				main_ast,
				os.args.contains("-o") 
			) // Init compiler

			/* If flag -time exists */
			if os.args.contains("-time") == true {
				println("End Time:   " + time.now().format_ss_milli())
			}
		} else {
			println("ShellError: Not enough permissions to open this file!") // Some error
		}
	}
} 
