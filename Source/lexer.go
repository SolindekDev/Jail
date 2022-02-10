package main

import (
	"os"
	"strings"
)

func get_last_token(tokens []Token, filename string) Token {
	if len(tokens) == 0 {
		return create_token(filename, "NIL", NIL, 0, 0)
	} else {
		return tokens[len(tokens)-1]
	}
}

func lexer_init(value string, filename string) Lexer {
	var lexer Lexer

	var lexer_row int = 0
	var lexer_col int = 0

	var error_ bool = false

	var lexer_comments bool = false
	var lexer_space bool = false
	var lexer_string bool = false

	for i := 0; i < len(value); i++ {
		last := get_last_token(lexer.tokens, filename)
		ac := string(value[i])
		if ac == "\n" {
			lexer_row++
			lexer_col = 0
			lexer_comments = false
		} else if ac == "#" {
			if lexer_string == true {
				lexer.tokens[len(lexer.tokens)-1].value += ac
			} else {
				if lexer_comments == false {
					lexer_comments = true
				}
			}
		} else if lexer_comments == true {
			continue
		} else if ac == "\"" {
			if lexer_string == true {
				lexer_string = false
			} else {
				lexer.tokens = append(lexer.tokens, create_token(filename, "", STRING, lexer_row, lexer_col))
				lexer_string = true
			}
		} else if lexer_string == true {
			lexer.tokens[len(lexer.tokens)-1].value += ac
		} else if ac == " " {
			lexer_space = true
		} else if strings.ContainsAny(ac, LETTERS) {
			if last.type_token == NIL {
				lexer.tokens = append(lexer.tokens, create_token(filename, ac, IDENTIFIER, lexer_row, lexer_col))
				lexer_space = false
			} else {
				if lexer_space == false {
					if last.type_token == IDENTIFIER {
						lexer.tokens[len(lexer.tokens)-1].value += ac
						lexer_space = false
					} else {
						lexer.tokens = append(lexer.tokens, create_token(filename, ac, IDENTIFIER, lexer_row, lexer_col))
						lexer_space = false
					}
				} else {
					lexer.tokens = append(lexer.tokens, create_token(filename, ac, IDENTIFIER, lexer_row, lexer_col))
					lexer_space = false
				}
			}
		} else if strings.ContainsAny(ac, NUMBERS) {
			if last.type_token == NIL {
				lexer.tokens = append(lexer.tokens, create_token(filename, ac, INT, lexer_row, lexer_col))
				lexer_space = false
			} else {
				if lexer_space == false {
					if last.type_token == INT {
						lexer.tokens[len(lexer.tokens)-1].value += ac
					} else if last.type_token == FLOAT {
						lexer.tokens[len(lexer.tokens)-1].value += ac
						lexer.tokens[len(lexer.tokens)-1].type_token = FLOAT
					} else if last.type_token == IDENTIFIER && lexer_space == false {
						lexer.tokens[len(lexer.tokens)-1].value += ac
						lexer.tokens[len(lexer.tokens)-1].type_token = IDENTIFIER
					} else {
						lexer.tokens = append(lexer.tokens, create_token(filename, ac, INT, lexer_row, lexer_col))
						lexer_space = false
					}
				} else {
					lexer.tokens = append(lexer.tokens, create_token(filename, ac, INT, lexer_row, lexer_col))
					lexer_space = false
				}
			}
		} else if ac == PLUS_TOKEN {
			lexer.tokens = append(lexer.tokens, create_token(filename, PLUS_TOKEN, PLUS, lexer_row, lexer_col))
		} else if ac == MINUS_TOKEN {
			lexer.tokens = append(lexer.tokens, create_token(filename, MINUS_TOKEN, MINUS, lexer_row, lexer_col))
		} else if ac == DIVIDE_TOKEN {
			lexer.tokens = append(lexer.tokens, create_token(filename, DIVIDE_TOKEN, DIVIDE, lexer_row, lexer_col))
		} else if ac == MULTIPLY_TOKEN {
			lexer.tokens = append(lexer.tokens, create_token(filename, MULTIPLY_TOKEN, MULTIPLY, lexer_row, lexer_col))
		} else if ac == MODULUS_TOKEN {
			lexer.tokens = append(lexer.tokens, create_token(filename, MODULUS_TOKEN, MODULUS, lexer_row, lexer_col))
		} else if ac == BLOCK_BRACKETS_OPEN_TOKEN {
			lexer.tokens = append(lexer.tokens, create_token(filename, BLOCK_BRACKETS_OPEN_TOKEN, BLOCK_BRACKETS_OPEN, lexer_row, lexer_col))
		} else if ac == BLOCK_BRACKETS_CLOSE_TOKEN {
			lexer.tokens = append(lexer.tokens, create_token(filename, BLOCK_BRACKETS_CLOSE_TOKEN, BLOCK_BRACKETS_CLOSE, lexer_row, lexer_col))
		} else if value[i] == 13 {
			continue
		} else if ac == "." {
			if last.type_token == NIL {
				lexer.tokens = append(lexer.tokens, create_token(filename, "0"+ac, FLOAT, lexer_row, lexer_col))
				lexer_space = false
			} else {
				if last.type_token == INT || last.type_token == FLOAT {
					if last.type_token == INT {
						lexer.tokens[len(lexer.tokens)-1].type_token = FLOAT
						lexer.tokens[len(lexer.tokens)-1].value += ac
					} else {
						if strings.ContainsAny(last.value, ".") == true {
							error_print_lexer(lexer_row, lexer_col, filename, "This float already contains \".\"", "SyntaxError")
						} else {
							lexer.tokens[len(lexer.tokens)-1].type_token = FLOAT
							lexer.tokens[len(lexer.tokens)-1].value += ac
						}
					}
				} else {
					lexer.tokens = append(lexer.tokens, create_token(filename, "0"+ac, FLOAT, lexer_row, lexer_col))
					lexer_space = false
				}
			}
		} else {
			error_print_lexer(lexer_row, lexer_col, filename, "Unsupported character: \""+ac+"\"", "SyntaxError")
			error_ = true
		}

		UNUSED(error_)

		lexer_col++
	}

	UNUSED(lexer_row, lexer_col, lexer_comments, lexer_space, lexer_string)

	if error_ == true {
		os.Exit(FailedCode)
	}

	// print_out_lexer(lexer)

	// error_print(lexer.tokens[i], "there is an error bro", "SyntaxError")

	return lexer
}

func print_out_lexer(lexer Lexer) {
	print("[ \n")
	for i := 0; i < len(lexer.tokens); i++ {
		print("	\"" + lexer.tokens[i].value + "\":" + get_raw_token_type(lexer.tokens[i].type_token) + "\n")
	}
	print("]\n")
}
