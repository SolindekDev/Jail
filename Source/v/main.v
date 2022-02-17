import os
import lexer
import tokens
import io

fn main() {
	if os.args.len < 2 {
		println("TODO: Shell")
	} else {
		mut f := os.open(os.args[1]) or {
			println("ShellError: File not found or it not exists..")
			exit(1)
		}

		if f.is_opened == true {
			mut buf := []string
			buf = os.read_lines(os.args[1]) ?
			content := buf.join("\n")

			lexer.lexer(content, os.args[1])

			// TODO: Lexer
		} else {
			println("ShellError: Not enough permissions to open this file!")
		}
	}
} 
