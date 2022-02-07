package main

import (
	"math"
	"os"
	"strconv"
)

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

func parser_init(lexer Lexer) Parser {
	var parser Parser

	var freeze int = 0
	var error bool = false
	var num int = 0
	var num1 string = ""
	var num_type int = 100

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
						error_print(lexer.tokens[i], "Expected an String after "+KEYWORD_PUTS+" keyword no "+get_raw_token_type(lexer.tokens[i+1].type_token), "MemoryError")
						error = true
					}
				} else {
					error_print(lexer.tokens[i], "Expected an String after "+KEYWORD_PUTS+" keyword", "MemoryError")
					error = true
				}
			}
		} else if lexer.tokens[i].type_token == INT || lexer.tokens[i].type_token == FLOAT {
			if lexer.tokens[i].type_token == INT {
				i1, _ := strconv.ParseInt(lexer.tokens[i].value, 10, 64)

				if i1 == 9223372036854775807 {
					error_print(lexer.tokens[i], "Int is too big", "MemoryError")
					error = true
				}

				if num == 1 {
					error_print(lexer.tokens[i], "Expected operator not another number", "SyntaxError")
					error = true
				} else {
					num1 = lexer.tokens[i].value
					num = 1
					num_type = INT
				}
			} else {
				i1, _ := strconv.ParseFloat(lexer.tokens[i].value, 64)

				if i1 == math.MaxFloat64 {
					error_print(lexer.tokens[i], "Float is too big", "MemoryError")
					error = true
				}

				if num == 1 {
					error_print(lexer.tokens[i], "Expected operator not another number", "SyntaxError")
					error = true
				} else {
					num1 = lexer.tokens[i].value
					num = 1
					num_type = FLOAT
				}
			}

		} else if lexer.tokens[i].type_token == PLUS || lexer.tokens[i].type_token == MINUS || lexer.tokens[i].type_token == MULTIPLY || lexer.tokens[i].type_token == DIVIDE || lexer.tokens[i].type_token == MODULUS {
			if lexer.tokens[i].type_token == PLUS {
				if num == 1 {
					if len(lexer.tokens) > i+1 {
						if num1 == "last_operation" {
							i2, _ := strconv.ParseFloat(lexer.tokens[i+1].value, 64)

							parser.opcodes = append(parser.opcodes, OpCode_Add(0x0f3019fb, i2))
							freeze += 1
							num1 = "last_operation"
							num = 1
						} else {
							i1, _ := strconv.ParseFloat(num1, 64)
							i2, _ := strconv.ParseFloat(lexer.tokens[i+1].value, 64)

							parser.opcodes = append(parser.opcodes, OpCode_Add(i1, i2))
							freeze += 1
							num1 = "last_operation"
							num = 1
						}
					} else {
						error_print(lexer.tokens[i], "Expected a number after a plus operator", "SyntaxError")
						error = true
						freeze += 1
					}
				} else {
					error_print(lexer.tokens[i], "Before plus operator need to be a number", "SyntaxError")
					error = true
				}
			} else if lexer.tokens[i].type_token == MINUS {
				if num == 1 {
					if len(lexer.tokens) > i+1 {
						if num1 == "last_operation" {
							i2, _ := strconv.ParseFloat(lexer.tokens[i+1].value, 64)

							parser.opcodes = append(parser.opcodes, OpCode_Minus(0x0f3019fb, i2))
							freeze += 1
							num1 = "last_operation"
							num = 1
						} else {
							i1, _ := strconv.ParseFloat(num1, 64)
							i2, _ := strconv.ParseFloat(lexer.tokens[i+1].value, 64)

							parser.opcodes = append(parser.opcodes, OpCode_Minus(i1, i2))
							freeze += 1
							num1 = "last_operation"
							num = 1
						}
					} else {
						error_print(lexer.tokens[i], "Expected a number after a minus operator", "SyntaxError")
						error = true
						freeze += 1
					}
				} else {
					error_print(lexer.tokens[i], "Before minus operator need to be a number", "SyntaxError")
					error = true
				}
			} else if lexer.tokens[i].type_token == MODULUS {
				if num == 1 {
					if len(lexer.tokens) > i+1 {
						if num1 == "last_operation" {
							i2, _ := strconv.ParseFloat(lexer.tokens[i+1].value, 64)

							parser.opcodes = append(parser.opcodes, OpCode_Modulus(0x0f3019fb, i2))
							freeze += 1
							num1 = "last_operation"
							num = 1
						} else {
							i1, _ := strconv.ParseFloat(num1, 64)
							i2, _ := strconv.ParseFloat(lexer.tokens[i+1].value, 64)

							parser.opcodes = append(parser.opcodes, OpCode_Modulus(i1, i2))
							freeze += 1
							num1 = "last_operation"
							num = 1
						}
					} else {
						error_print(lexer.tokens[i], "Expected a number after a modulus operator", "SyntaxError")
						error = true
						freeze += 1
					}
				} else {
					error_print(lexer.tokens[i], "Before modulus operator need to be a number", "SyntaxError")
					error = true
				}
			} else if lexer.tokens[i].type_token == DIVIDE {
				if num == 1 {
					if len(lexer.tokens) > i+1 {
						if num1 == "last_operation" {
							i2, _ := strconv.ParseFloat(lexer.tokens[i+1].value, 64)

							parser.opcodes = append(parser.opcodes, OpCode_Divide(0x0f3019fb, i2))
							freeze += 1
							num1 = "last_operation"
							num = 1
						} else {
							i1, _ := strconv.ParseFloat(num1, 64)
							i2, _ := strconv.ParseFloat(lexer.tokens[i+1].value, 64)

							parser.opcodes = append(parser.opcodes, OpCode_Divide(i1, i2))
							freeze += 1
							num1 = "last_operation"
							num = 1
						}
					} else {
						error_print(lexer.tokens[i], "Expected a number after a divide operator", "SyntaxError")
						error = true
						freeze += 1
					}
				} else {
					error_print(lexer.tokens[i], "Before divide operator need to be a number", "SyntaxError")
					error = true
				}
			} else if lexer.tokens[i].type_token == MULTIPLY {
				if num == 1 {
					if len(lexer.tokens) > i+1 {
						if num1 == "last_operation" {
							i2, _ := strconv.ParseFloat(lexer.tokens[i+1].value, 64)

							parser.opcodes = append(parser.opcodes, OpCode_Multiply(0x0f3019fb, i2))
							freeze += 1
							num1 = "last_operation"
							num = 1
						} else {
							i1, _ := strconv.ParseFloat(num1, 64)
							i2, _ := strconv.ParseFloat(lexer.tokens[i+1].value, 64)

							parser.opcodes = append(parser.opcodes, OpCode_Multiply(i1, i2))
							freeze += 1
							num1 = "last_operation"
							num = 1
						}
					} else {
						error_print(lexer.tokens[i], "Expected a number after a multiply operator", "SyntaxError")
						error = true
						freeze += 1
					}
				} else {
					error_print(lexer.tokens[i], "Before multiply operator need to be a number", "SyntaxError")
					error = true
				}
			}
		}
	}

	if len(parser.opcodes) == 0 && len(lexer.tokens) == 1 {
		if lexer.tokens[0].type_token == FLOAT || lexer.tokens[0].type_token == INT {
			i1, _ := strconv.ParseFloat(lexer.tokens[0].value, 64)
			parser.opcodes = append(parser.opcodes, OpCode_Return(i1))
		}
	}

	UNUSED(error, freeze, num, num_type)

	parser.errorCount = error

	if parser.errorCount == true {
		os.Exit(FailedCode)
	}

	// print_out_parser(parser)

	return parser
}
