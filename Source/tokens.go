package main

const NUMBERS string = "0123456789"
const LETTERS string = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_?@"

const (
	STRING int = iota
	INT
	FLOAT
	IDENTIFIER
	PLUS
	MINUS
	MULTIPLY
	DIVIDE
	MODULUS
	BLOCK_BRACKETS_OPEN
	BLOCK_BRACKETS_CLOSE
	NIL
)

const (
	PLUS_TOKEN     = "+"
	MINUS_TOKEN    = "-"
	DIVIDE_TOKEN   = "/"
	MULTIPLY_TOKEN = "*"
	MODULUS_TOKEN  = "^"

	BLOCK_BRACKETS_OPEN_TOKEN  = "["
	BLOCK_BRACKETS_CLOSE_TOKEN = "]"
)

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
	case MODULUS:
		to_ret = "Modulus"
		break
	case BLOCK_BRACKETS_OPEN:
		to_ret = "OpenBracketsBlock"
		break
	case BLOCK_BRACKETS_CLOSE:
		to_ret = "CloseBracketsBlock"
		break
	}

	return to_ret
}
