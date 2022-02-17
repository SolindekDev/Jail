module lexer

import tokens
import error

pub fn lexer(value string, filename string) {
	mut tokens_ := []tokens.Token{}

	mut error_ := false

	mut lexer_row 			 := 0
	mut lexer_col 			 := 0
	mut lexer_comments       := false
	mut lexer_comments_multi := false
	mut lexer_space 		 := false
	mut lexer_string 		 := false

	for i := 0; i < value.len; i++ {
		ac := value[i].ascii_str()
		ac_ := value[i]
		last := tokens.get_last_token(tokens_, filename)

		lexer_col ++ 

		if ac == "\n" {
			lexer_space = true
			lexer_comments = false
			lexer_row++
			lexer_col=0
		} else if ac == "#" {
			if lexer_string == true {
				tokens_[tokens_.len - 1].value += ac
			} else {
				if lexer_comments == false {
					lexer_comments = true
				}
			}
		} else if ac == "$" {
			if lexer_string == true {
				tokens_[tokens_.len - 1].value += ac
			} else {
				if lexer_comments_multi == false {
					lexer_comments_multi = true
				} else {
					lexer_comments_multi = false
				}
			}
		} else if lexer_comments == true || lexer_comments_multi == true {
			continue
		} else if ac_ == 34 {
			if lexer_string == true {
				lexer_string = false
			} else {
				tokens_ << tokens.create_token(
					filename,
					"",
					tokens.Types.string_,
					lexer_row,
					lexer_col
				)	
				lexer_string = true
			}
		} else if lexer_string == true {
			tokens_[tokens_.len - 1].value += ac
		} else if ac == " " {
			lexer_space = true
		} else if tokens.get_one_char(ac) != tokens.Types.null {
			tokens_ << tokens.create_token(
				filename, 
				ac, 
				tokens.get_one_char(ac), 
				lexer_row,
				lexer_col
			)
			lexer_space = false
		} else if tokens.letters_constants.contains_any(ac) == true {
			if last.type_token == tokens.Types.null {
				tokens_ << tokens.create_token(
					filename, 
					ac, 
					tokens.Types.identifier, 
					lexer_row,
					lexer_col
				)
				lexer_space = false
			} else {
				if lexer_space == false {
					if last.type_token == tokens.Types.identifier {
						tokens_[tokens_.len-1].value += ac
						lexer_space = false
					} else {
						tokens_ << tokens.create_token(
							filename, 
							ac, 
							tokens.Types.identifier, 
							lexer_row,
							lexer_col
						)
						lexer_space = false
					}
				} else {
					tokens_ << tokens.create_token(
						filename, 
						ac, 
						tokens.Types.identifier, 
						lexer_row,
						lexer_col
					)
					lexer_space = false
				}
			}
		} else if tokens.numbers_constants.contains_any(ac) == true {
			if last.type_token == tokens.Types.null {
				tokens_ << tokens.create_token(
					filename, 
					ac, 
					tokens.Types.int, 
					lexer_row,
					lexer_col
				)
				lexer_space = false
			} else {
				if lexer_space == false {
					if last.type_token == tokens.Types.int {
						tokens_[tokens_.len-1].value += ac
					} else if last.type_token == tokens.Types.float {
						tokens_[tokens_.len-1].value += ac
						tokens_[tokens_.len-1].type_token = tokens.Types.float
					} else if last.type_token == tokens.Types.identifier && lexer_space == false {
						tokens_[tokens_.len-1].value += ac
						tokens_[tokens_.len-1].type_token = tokens.Types.identifier
					} else {
						tokens_ << tokens.create_token(
							filename, 
							ac, 
							tokens.Types.int, 
							lexer_row,
							lexer_col
						)
						lexer_space = false
					}
				} else {
					tokens_ << tokens.create_token(
						filename, 
						ac, 
						tokens.Types.int, 
						lexer_row,
						lexer_col
					)
					lexer_space = false
				}
			}
		} else if ac == "." {
			if last.type_token == tokens.Types.null {
				tokens_ << tokens.create_token(
					filename, 
					"0.", 
					tokens.Types.float, 
					lexer_row,
					lexer_col
				)
				lexer_space = false
			} else {
				if last.type_token == tokens.Types.int || last.type_token == tokens.Types.float {
					if last.type_token == tokens.Types.int {
						tokens_[tokens_.len-1].type_token = tokens.Types.float
						tokens_[tokens_.len-1].value += ac
					} else {
						if last.value.contains_any(".") == true {
							error.error_print_lexer(lexer_row, lexer_col, filename, "This float already contains \".\"", "SyntaxError")
							error_ = true
						} else {
							tokens_[tokens_.len-1].type_token = tokens.Types.float
							tokens_[tokens_.len-1].value += ac
						}
					}
				} else {
					tokens_ << tokens.create_token(
						filename, 
						"0.", 
						tokens.Types.float, 
						lexer_row,
						lexer_col
					)
					lexer_space = false
				}
			}
		} else {
			error.error_print_lexer(lexer_row, lexer_col, filename, "Unsupported character: \""+ac+"\"", "SyntaxError")
			error_ = true
		}
	} 	 

	if error_ == true {
		exit(1) // 1 is a failed code
	}

	for i := 0; i < tokens_.len; i++ {
		tokens.print_out_token(tokens_[i])
	}
}