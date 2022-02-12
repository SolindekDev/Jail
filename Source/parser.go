package main

import (
	"fmt"
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

type Variable struct {
	variableName  string
	variableValue string
}

func variable_exists(name string, variables []Variable) bool {
	for _, v := range variables {
		if v.variableName == name {
			return true
		}
	}

	return false
}

func create_variable(variableName string, variableValue string) Variable {
	var var_ Variable

	var_.variableName = variableName
	var_.variableValue = variableValue

	return var_
}

func found_variable(variableName string, variables []Variable) Variable {
	for _, v := range variables {
		if v.variableName == variableName {
			return v
		}
	}

	return create_variable("variable_not_found111111111", "1083921nsa")
}

func get_variable_index(variableName string, variables []Variable) int {
	i := 0
	for _, v := range variables {
		i++
		if v.variableName == variableName {
			return i
		}
	}

	return -1
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

	var variables []Variable

	for i := 0; i < len(lexer.tokens); i++ {
		if freeze != 0 {
			freeze--
			continue
		}

		if lexer.tokens[i].type_token == IDENTIFIER {
			if lexer.tokens[i].value == KEYWORD_PUTS {
				/*
					Code for puts keyword - printing the value
				*/
				if len(lexer.tokens) > i+1 {
					if lexer.tokens[i+1].type_token == STRING {
						parser.opcodes = append(parser.opcodes, OpCode_Puts(lexer.tokens[i+1].value))
						freeze += 1
					} else if lexer.tokens[i+1].type_token == MATH {
						parser.opcodes = append(parser.opcodes, OpCode_Puts_Math(lexer.tokens[i+1].value))
						freeze += 1
					} else if lexer.tokens[i+1].type_token == IDENTIFIER {
						if variable_exists(lexer.tokens[i+1].value, variables) == false {
							parser.opcodes = append(parser.opcodes, OpCode_Puts_Identifier(lexer.tokens[i+1].value))
							freeze += 1
						} else {
							parser.opcodes = append(parser.opcodes, OpCode_Puts_Variable(lexer.tokens[i+1].value))
							freeze += 1
						}
					} else {
						error_print(lexer.tokens[i], "Expected an String after "+KEYWORD_PUTS+" keyword no "+get_raw_token_type(lexer.tokens[i+1].type_token), "SyntaxError", filename)
						error = true
					}
				} else {
					error_print(lexer.tokens[i], "Expected an String after "+KEYWORD_PUTS+" keyword", "MemoryError", filename)
					error = true
				}
			} else if lexer.tokens[i].value == KEYWORD_VARIABLE {
				/*
					Code for declaring variable


					Ugh this code is sooo ugly... But it works 🦄🦄

				*/
				if len(lexer.tokens) > i+1 {
					if lexer.tokens[i+1].type_token == IDENTIFIER {
						if variable_exists(lexer.tokens[i+1].value, variables) == false {
							if len(lexer.tokens) > i+2 {
								if lexer.tokens[i+2].type_token == EQUALS {
									if len(lexer.tokens) > i+3 {
										if lexer.tokens[i+3].type_token == MATH {
											parser.opcodes = append(parser.opcodes, OpCode_Variable_Declare_Math(lexer.tokens[i+1].value, lexer.tokens[i+3].value))
											variables = append(variables, create_variable(lexer.tokens[i+1].value, lexer.tokens[i+3].value))
											freeze += 3
										} else if lexer.tokens[i+3].type_token == STRING {
											parser.opcodes = append(parser.opcodes, OpCode_Variable_Declare_String(lexer.tokens[i+1].value, lexer.tokens[i+3].value))
											variables = append(variables, create_variable(lexer.tokens[i+1].value, lexer.tokens[i+3].value))
											freeze += 3
										} else {
											error_print(lexer.tokens[i], "Unexpected type of token to put into variable", "TypeError", filename)
											error = true
										}
									} else {
										error_print(lexer.tokens[i], "Expected an = tokens", "SyntaxError", filename)
										error = true
									}
								} else {
									error_print(lexer.tokens[i], "Expected an = tokens", "SyntaxError", filename)
									error = true
								}
							} else {
								error_print(lexer.tokens[i], "Expected an = tokens", "SyntaxError", filename)
								error = true
							}
						} else {
							error_print(lexer.tokens[i], "Variable with this name already exists", "SyntaxError", filename)
							error = true
						}
					} else {
						error_print(lexer.tokens[i], "Type of variable need to be an Identifier", "SyntaxError", filename)
						error = true
					}
				} else {
					error_print(lexer.tokens[i], "Expected an name of variable", "SyntaxError", filename)
					error = true
				}
			} else {
				/*
					Code for changing the variable value
				*/
				if variable_exists(lexer.tokens[i].value, variables) == false {
					error_print(lexer.tokens[i], lexer.tokens[i].value+" is not defined", "ReferenceError", filename)
					error = true
				} else {
					if len(lexer.tokens) > i+1 {
						if lexer.tokens[i+1].type_token == EQUALS {
							if len(lexer.tokens) > i+2 {
								if lexer.tokens[i+2].type_token == MATH {
									index_var := get_variable_index(lexer.tokens[i].value, variables)

									variables[index_var].variableValue = fmt.Sprint(lexer.tokens[i+2].type_token)

									parser.opcodes = append(parser.opcodes, OpCode_Change_Value_Varaiable_Math(lexer.tokens[i].value, lexer.tokens[i+2].value))
									freeze += 2
								} else if lexer.tokens[i+2].type_token == STRING {
									index_var := get_variable_index(lexer.tokens[i].value, variables)
									variables[index_var-1].variableValue = fmt.Sprint(lexer.tokens[i+2].type_token)

									parser.opcodes = append(parser.opcodes, OpCode_Change_Value_Varaiable_String(lexer.tokens[i].value, lexer.tokens[i+2].value))
									freeze += 2
								} else {
									error_print(lexer.tokens[i], "Unexpected type of token to put into variable", "TypeError", filename)
									error = true
								}
							}
						}
					}
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
