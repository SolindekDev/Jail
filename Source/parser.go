package main

import (
	"os"
	"strconv"
	"strings"
)

func refactor_tokens_math(lexer Lexer) Lexer {
	var lexer_ Lexer

	lexer_.filename = lexer.filename
	lexer_.value = lexer.value

	var math_ bool = false
	var j int = 0

	for i := 0; i < len(lexer.tokens); i++ {
		if lexer.tokens[i].type_token == IDENTIFIER || lexer.tokens[i].type_token == STRING {
			lexer_.tokens = append(lexer_.tokens, create_token(lexer_.filename, lexer.tokens[i].value, lexer.tokens[i].type_token, lexer.tokens[i].pos.row, lexer.tokens[i].pos.col))
			j++
			math_ = false
		} else if math_ == true {
			lexer_.tokens[j-1].value += lexer.tokens[i].value
		} else if lexer.tokens[i].type_token == PLUS || lexer.tokens[i].type_token == MINUS || lexer.tokens[i].type_token == MULTIPLY || lexer.tokens[i].type_token == INT || lexer.tokens[i].type_token == DIVIDE || lexer.tokens[i].type_token == MODULUS && math_ == false {
			lexer_.tokens = append(lexer_.tokens, create_token(lexer_.filename, lexer.tokens[i].value, MATH, lexer.tokens[i].pos.row, lexer.tokens[i].pos.col))
			j++
			math_ = true
		} else {
			lexer_.tokens = append(lexer_.tokens, create_token(lexer_.filename, lexer.tokens[i].value, lexer.tokens[i].type_token, lexer.tokens[i].pos.row, lexer.tokens[i].pos.col))
			j++
		}
	}

	UNUSED(math_)

	// print_out_lexer(lexer_)

	return lexer_
}

func print_out_parser(parser Parser) {
	print("[ \n")
	for i := 0; i < len(parser.opcodes); i++ {
		print(strconv.Itoa(parser.opcodes[i].opcode))
		print(" Args: [")
		for j := 0; j < len(parser.opcodes[i].args); j++ {
			out, _ := strconv.Atoi(parser.opcodes[i].args[j])
			if out == 0x0f3019fb {
				print("\"Last Operation\" ")
			} else {
				print("\"" + parser.opcodes[i].args[j] + "\" ")
			}
		}
		print("]")
	}
	print("\n]\n")
}

func parser_init(lexer Lexer, filename string) Parser {
	var parser Parser

	var freeze int = 0
	var error bool = false

	for i := 0; i < len(lexer.tokens); i++ {
		if freeze != 0 {
			freeze--
			continue
		}

		if lexer.tokens[i].type_token == IDENTIFIER {
			if lexer.tokens[i].value == KEYWORD_PUTS {
				if len(lexer.tokens) > i+1 {
					if lexer.tokens[i+1].type_token == STRING {
						parser.opcodes = append(parser.opcodes, OpCode_Puts(lexer.tokens[i+1].value))
						freeze += 1
					} else {
						error_print(lexer.tokens[i], "Expected an String after "+KEYWORD_PUTS+" keyword no "+get_raw_token_type(lexer.tokens[i+1].type_token), "MemoryError", filename)
						error = true
					}
				} else {
					error_print(lexer.tokens[i], "Expected an String after "+KEYWORD_PUTS+" keyword", "MemoryError", filename)
					error = true
				}
			}
		} else if lexer.tokens[i].type_token == MATH {
			var num int = 0
			var operator int = 0

			for j := 0; j < len(lexer.tokens[i].value); j++ {
				if lexer.tokens[i].value[j] == '+' || lexer.tokens[i].value[j] == '-' || lexer.tokens[i].value[j] == '/' || lexer.tokens[i].value[j] == '*' || lexer.tokens[i].value[j] == '%' {
					if num == 0 {
						err := "Expected an number before operator "
						error_print(lexer.tokens[i], string(err)+string(lexer.tokens[i].value[j]), "SyntaxError", filename)
						error = true
					} else {
					}
				} else if strings.ContainsAny(string(lexer.tokens[i].value[j]), NUMBERS) {
					num = 1
				}
			}

			if error != true {
				parser.opcodes = append(parser.opcodes, OpCode_Math(lexer.tokens[i].value))
			}

			UNUSED(operator)
		}
	}

	if len(parser.opcodes) == 0 && len(lexer.tokens) == 1 {
		if lexer.tokens[0].type_token == FLOAT || lexer.tokens[0].type_token == INT {
			i1, _ := strconv.ParseFloat(lexer.tokens[0].value, 64)
			parser.opcodes = append(parser.opcodes, OpCode_Return(i1))
		}
	}

	UNUSED(error, freeze)

	parser.errorCount = error

	if parser.errorCount == true {
		os.Exit(FailedCode)
	}

	// print_out_parser(parser)

	return parser
}
