import os
import time
import lexer
import ast
import compiler

fn main() {
	if os.args.len < 2 {
		println("TODO: Shell")
	} else {
		mut f := os.open(os.args[1]) or {
			println("ShellError: File not found or it not exists..")
			exit(1)
		}

		if f.is_opened == true {
			mut buf := []string{}
			buf = os.read_lines(os.args[1]) ?
			content := buf.join("\n")

			if os.args.contains("-time") == true {
				println("Start Time: " + time.now().format_ss_milli())
			}
			tokens   := lexer.lexer(content, os.args[1])
			main_ast := ast.ast_init(tokens)
			compiler.compiler_init(
				main_ast,
				os.args.contains("-o")
			)
			if os.args.contains("-time") == true {
				println("End Time:   " + time.now().format_ss_milli())
			}
		} else {
			println("ShellError: Not enough permissions to open this file!")
		}
	}
} 
