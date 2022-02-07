package main

import (
	"bufio"
	"errors"
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

var SuccesCode int = 0
var FailedCode int = 1

const (
	STRING int = iota
	INT
	FLOAT
	IDENTIFIER
	PLUS
	MINUS
	MULTIPLY
	DIVIDE
	NIL
)

const (
	PLUS_TOKEN     = "+"
	MINUS_TOKEN    = "-"
	DIVIDE_TOKEN   = "/"
	MULTIPLY_TOKEN = "*"
)

const (
	OPCODE_ADD_NUMBERS int = iota
	OPCODE_MINUS_NUMBERS
	OPCODE_DIVIDE_NUMBERS
	OPCODE_MULTIPLY_NUMBERS
)

func create_opcode(opcode int, args []string) OpCode {
	var opcode_ OpCode
	opcode_.opcode = opcode
	opcode_.args = args

	return opcode_
}

func OpCode_Divide(number1 int, number2 int) OpCode {
	args := []string{strconv.Itoa(number1), strconv.Itoa(number2)}

	return create_opcode(OPCODE_DIVIDE_NUMBERS, args)
}

func OpCode_Multiply(number1 int, number2 int) OpCode {
	args := []string{strconv.Itoa(number1), strconv.Itoa(number2)}

	return create_opcode(OPCODE_MULTIPLY_NUMBERS, args)
}

func OpCode_Minus(number1 int, number2 int) OpCode {
	args := []string{strconv.Itoa(number1), strconv.Itoa(number2)}

	return create_opcode(OPCODE_MINUS_NUMBERS, args)
}

func OpCode_Add(number1 int, number2 int) OpCode {
	args := []string{strconv.Itoa(number1), strconv.Itoa(number2)}

	return create_opcode(OPCODE_ADD_NUMBERS, args)
}

const NUMBERS string = "0123456789"
const LETTERS string = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_?@"

type Position struct {
	row int
	col int
}

type Token struct {
	value      string
	filename   string
	type_token int
	pos        Position
}

type Lexer struct {
	tokens   []Token
	filename string
	value    string
}

type OpCode struct {
	opcode int
	args   []string
}

type Parser struct {
	opcodes    []OpCode
	errorCount bool
}

func UNUSED(x ...interface{}) {}

// check if file exists
func file_exists(filename string) bool {
	_, err := os.Stat(filename)
	return !errors.Is(err, os.ErrNotExist)
}

// get file content
func get_file_content(filename string) string {
	content, err := ioutil.ReadFile(filename)

	if err != nil {
		fmt.Println("File not exists")
	}

	return string(content)
}

func create_token(filename string, value string, type_token int, row int, col int) Token {
	var token Token
	var pos Position

	pos.row = row
	pos.col = col

	token.filename = filename
	token.value = value
	token.type_token = type_token

	token.pos = pos

	return token
}

func get_raw_token_type(type_token int) string {
	to_ret := "NIL"

	switch type_token {
	case STRING:
		to_ret = "String"
		break
	case INT:
		to_ret = "Int"
		break
	case FLOAT:
		to_ret = "Float"
		break
	case IDENTIFIER:
		to_ret = "Identifier"
		break
	case PLUS:
		to_ret = "Plus"
		break
	case MINUS:
		to_ret = "Minus"
		break
	case DIVIDE:
		to_ret = "Divide"
		break
	case MULTIPLY:
		to_ret = "Multiply"
		break
	}

	return to_ret
}

func get_last_token(tokens []Token, filename string) Token {
	if len(tokens) == 0 {
		return create_token(filename, "NIL", NIL, 0, 0)
	} else {
		return tokens[len(tokens)-1]
	}
}

func error_print(token Token, errorBody string, errorTitle string) {
	if token.filename == "Input" {
		token.filename = "Shell"
	}
	fmt.Printf("%s:%d:%d: %s: %s\n", token.filename, token.pos.row, token.pos.col, errorTitle, errorBody)
}

func error_print_lexer(row int, col int, filename string, errorBody string, errorTitle string) {
	if filename == "Input" {
		filename = "Shell"
	}
	fmt.Printf("%s:%d:%d: %s: %s\n", filename, row, col, errorTitle, errorBody)
}

func parser_init(lexer Lexer) Parser {
	var parser Parser

	var freeze int = 0
	var error bool = false
	var num int = 0
	var num1 string = ""

	for i := 0; i < len(lexer.tokens); i++ {

		if freeze != 0 {
			freeze--
			continue
		}

		if lexer.tokens[i].type_token == INT {
			i1, _ := strconv.ParseInt(lexer.tokens[i].value, 10, 64)

			if i1 == 9223372036854775807 {
				error_print(lexer.tokens[i], "Number is too big", "MemoryError")
				error = true
			}

			if num == 1 {
				error_print(lexer.tokens[i], "Expected operator not another number", "SyntaxError")
				error = true
			} else {
				num1 = lexer.tokens[i].value
				num = 1
			}
		} else if lexer.tokens[i].type_token == PLUS || lexer.tokens[i].type_token == MINUS || lexer.tokens[i].type_token == MULTIPLY || lexer.tokens[i].type_token == DIVIDE {
			if lexer.tokens[i].type_token == PLUS {
				if num == 1 {
					if len(lexer.tokens) > i+1 {
						if num1 == "last_operation" {
							i2, _ := strconv.ParseInt(lexer.tokens[i+1].value, 10, 64)

							parser.opcodes = append(parser.opcodes, OpCode_Add(0x0f3019fb, int(i2)))
							freeze += 1
							num1 = "last_operation"
							num = 1
						} else {
							i1, _ := strconv.ParseInt(num1, 10, 64)
							i2, _ := strconv.ParseInt(lexer.tokens[i+1].value, 10, 64)

							parser.opcodes = append(parser.opcodes, OpCode_Add(int(i1), int(i2)))
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
							i2, _ := strconv.ParseInt(lexer.tokens[i+1].value, 10, 64)

							parser.opcodes = append(parser.opcodes, OpCode_Minus(0x0f3019fb, int(i2)))
							freeze += 1
							num1 = "last_operation"
							num = 1
						} else {
							i1, _ := strconv.ParseInt(num1, 10, 64)
							i2, _ := strconv.ParseInt(lexer.tokens[i+1].value, 10, 64)

							parser.opcodes = append(parser.opcodes, OpCode_Minus(int(i1), int(i2)))
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
			} else if lexer.tokens[i].type_token == DIVIDE {
				if num == 1 {
					if len(lexer.tokens) > i+1 {
						if num1 == "last_operation" {
							i2, _ := strconv.ParseInt(lexer.tokens[i+1].value, 10, 64)

							parser.opcodes = append(parser.opcodes, OpCode_Divide(0x0f3019fb, int(i2)))
							freeze += 1
							num1 = "last_operation"
							num = 1
						} else {
							i1, _ := strconv.ParseInt(num1, 10, 64)
							i2, _ := strconv.ParseInt(lexer.tokens[i+1].value, 10, 64)

							parser.opcodes = append(parser.opcodes, OpCode_Divide(int(i1), int(i2)))
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
							i2, _ := strconv.ParseInt(lexer.tokens[i+1].value, 10, 64)

							parser.opcodes = append(parser.opcodes, OpCode_Multiply(0x0f3019fb, int(i2)))
							freeze += 1
							num1 = "last_operation"
							num = 1
						} else {
							i1, _ := strconv.ParseInt(num1, 10, 64)
							i2, _ := strconv.ParseInt(lexer.tokens[i+1].value, 10, 64)

							parser.opcodes = append(parser.opcodes, OpCode_Multiply(int(i1), int(i2)))
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

	UNUSED(error, freeze, num)

	parser.errorCount = error

	if parser.errorCount == true {
		os.Exit(FailedCode)
	}

	// print_out_parser(parser)

	return parser
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
		} else if ac == "%" {
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
			// Soon...
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
		} else if value[i] == 13 {
			continue
		} else {
			error_print_lexer(lexer_row, lexer_col, filename, "Unsupported character: \""+ac+"\"", "SyntaxError")
			error_ = true
		}

		UNUSED(error_)

		lexer_col++
	}

	UNUSED(lexer_row, lexer_col, lexer_comments, lexer_space, lexer_string)

	// print("[ \n")
	// for i := 0; i < len(lexer.tokens); i++ {
	// 	print("	\"" + lexer.tokens[i].value + "\":" + get_raw_token_type(lexer.tokens[i].type_token) + "\n")
	// }
	// print("]\n")

	// error_print(lexer.tokens[i], "there is an error bro", "SyntaxError")

	return lexer
}

func eval_init(parser Parser) {
	var last_calc string = ""

	for i := 0; i < len(parser.opcodes); i++ {
		switch parser.opcodes[i].opcode {
		case OPCODE_ADD_NUMBERS:
			first_arg, _ := strconv.Atoi(parser.opcodes[i].args[0])
			if first_arg == 0x0f3019fb {
				calc2, _ := strconv.Atoi(last_calc)
				i1, _ := strconv.Atoi(parser.opcodes[i].args[1])

				calc := calc2 + i1

				last_calc = strconv.Itoa(calc)
			} else {
				i1, _ := strconv.Atoi(parser.opcodes[i].args[0])
				i2, _ := strconv.Atoi(parser.opcodes[i].args[1])

				calc := i1 + i2

				last_calc = strconv.Itoa(calc)
			}
			break
		case OPCODE_MINUS_NUMBERS:
			first_arg, _ := strconv.Atoi(parser.opcodes[i].args[0])
			if first_arg == 0x0f3019fb {
				calc2, _ := strconv.Atoi(last_calc)
				i1, _ := strconv.Atoi(parser.opcodes[i].args[1])

				calc := calc2 - i1

				last_calc = strconv.Itoa(calc)
			} else {
				i1, _ := strconv.Atoi(parser.opcodes[i].args[0])
				i2, _ := strconv.Atoi(parser.opcodes[i].args[1])

				calc := i1 - i2

				last_calc = strconv.Itoa(calc)
			}
			break
		case OPCODE_DIVIDE_NUMBERS:
			first_arg, _ := strconv.Atoi(parser.opcodes[i].args[0])
			if first_arg == 0x0f3019fb {
				calc2, _ := strconv.Atoi(last_calc)
				i1, _ := strconv.Atoi(parser.opcodes[i].args[1])

				if i1 == 0 || calc2 == 0 {
					fmt.Println("MathError: number divide by zero")
					os.Exit(FailedCode)
				}

				calc := calc2 / i1

				last_calc = strconv.Itoa(calc)
			} else {
				i1, _ := strconv.Atoi(parser.opcodes[i].args[0])
				i2, _ := strconv.Atoi(parser.opcodes[i].args[1])

				if i1 == 0 || i2 == 0 {
					fmt.Println("MathError: number divide by zero")
					os.Exit(FailedCode)
				}

				calc := i1 / i2

				last_calc = strconv.Itoa(calc)
			}
			break
		case OPCODE_MULTIPLY_NUMBERS:
			first_arg, _ := strconv.Atoi(parser.opcodes[i].args[0])
			if first_arg == 0x0f3019fb {
				calc2, _ := strconv.Atoi(last_calc)
				i1, _ := strconv.Atoi(parser.opcodes[i].args[1])

				calc := calc2 * i1

				last_calc = strconv.Itoa(calc)
			} else {
				i1, _ := strconv.Atoi(parser.opcodes[i].args[0])
				i2, _ := strconv.Atoi(parser.opcodes[i].args[1])

				calc := i1 * i2

				last_calc = strconv.Itoa(calc)
			}
			break
		}
	}

	UNUSED(last_calc)

	fmt.Println(last_calc)
}

// print out help
func help() {
}

func input() {
	fmt.Printf(">> ")
	var reader = bufio.NewReader(os.Stdin)
	_input, _ := reader.ReadString('\n')

	var lexer Lexer = lexer_init(_input, "Input")
	var parser Parser = parser_init(lexer)
	eval_init(parser)

	input()
}

// main function
func main() {
	argv := os.Args

	if len(argv) < 2 {
		input()
	} else {
		if argv[1] == "--help" || argv[1] == "--h" || argv[1] == "-h" || argv[1] == "-help" {
			fmt.Println("Joule - Interpreter programming language created in GoLang.\n\t- --help - Show this\n\t- --version - Show version\n\t- <filename.j> - Interpreter a file")
			os.Exit(SuccesCode)
		}

		var fileexst bool = file_exists(argv[1])

		if fileexst == false {
			fmt.Println(argv[1] + " <-- Ten plik nie istnieje")
			os.Exit(FailedCode)
		}

		var file_content string = get_file_content(argv[1])
		var lexer Lexer = lexer_init(file_content, argv[1])
		var parser Parser = parser_init(lexer)
		UNUSED(parser)
		// eval_init(parser.opcodes)
	}
}
