module lexer

import tokens

pub fn lexer(value string, filename string) {
	mut tokens_arr := []tokens.Token{}

	error_ := false

	mut lexer_row 			 := 0
	mut lexer_col 			 := 0
	mut lexer_comments       := false
	mut lexer_comments_multi := false
	mut lexer_space 		 := false
	mut lexer_string 		 := false
	
	// tokens_arr << create_token(
	// 	filename,
	// 	"",
	// 	Types.string_,
	// 	lexer_row,
	// 	lexer_col
	// )	

	for i := 0; i < value.len; i++ {
		ac := value[i].ascii_str()

		if ac == '#' {
			if lexer_string == true {
				tokens_arr[tokens_arr.len - 1].value += ac
			} else {
				if lexer_comments == false {
					lexer_comments = true
				}
			}
		} else if ac == '$' {
			if lexer_string == true {
				tokens_arr[tokens_arr.len - 1].value += ac
			} else {
				if lexer_comments_multi == false {
					lexer_comments_multi = true
				} else {
					lexer_comments_multi = false
				}
			}
		} else if lexer_comments == true || lexer_comments_multi == true {
			continue
		} else if ac == '"' {
			if lexer_string == true {
				lexer_string = false
			} else {
				tokens_arr << tokens.create_token(
					filename,
					"",
					tokens.Types.string_,
					lexer_row,
					lexer_col
				)	
				lexer_string = true
			}
		} else if lexer_string == true {
			tokens_arr[tokens_arr.len - 1].value += ac
		} else if ac == ' ' {
			lexer_space = true
		} else if tokens.get_one_char(ac) != tokens.Types.null {
			println("Symbole")
		} else if tokens.letters_constants.contains_any(ac) == true {
			// TODO: letters
		} else if tokens.numbers_constants.contains_any(ac) == true {
			// TODO: numbers
		}

		println(tokens.get_one_char(ac))
	} 	 
}