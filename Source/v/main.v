import os
import lexer
import tokens

fn main() {
	if os.args.len < 2 {
		println("TODO: Shell")
	} else {
		println("TODO: Compile")

		t := tokens.create_token(
			"Shell",
			"2137",
			tokens.Types.number,
			2137,
			2137
		)
		tokens.print_out_token(t)
	}
}
