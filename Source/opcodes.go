package main

import "strconv"

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
