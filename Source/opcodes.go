package main

import (
	"fmt"
)

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
	OPCODE_RETURN_NUMBER
)

func create_opcode(opcode int, args []string) OpCode {
	var opcode_ OpCode
	opcode_.opcode = opcode
	opcode_.args = args

	return opcode_
}

func OpCode_Return(return_num float64) OpCode {
	args := []string{fmt.Sprint(return_num)}

	return create_opcode(OPCODE_RETURN_NUMBER, args)
}

func OpCode_Divide(number1 float64, number2 float64) OpCode {
	args := []string{fmt.Sprint(number1), fmt.Sprint(number2)}

	return create_opcode(OPCODE_DIVIDE_NUMBERS, args)
}

func OpCode_Multiply(number1 float64, number2 float64) OpCode {
	args := []string{fmt.Sprint(number1), fmt.Sprint(number2)}

	return create_opcode(OPCODE_MULTIPLY_NUMBERS, args)
}

func OpCode_Minus(number1 float64, number2 float64) OpCode {
	args := []string{fmt.Sprint(number1), fmt.Sprint(number2)}

	return create_opcode(OPCODE_MINUS_NUMBERS, args)
}

func OpCode_Add(number1 float64, number2 float64) OpCode {
	args := []string{fmt.Sprint(number1), fmt.Sprint(number2)}

	return create_opcode(OPCODE_ADD_NUMBERS, args)
}
