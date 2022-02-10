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
	OPCODE_MATH int = iota
	OPCODE_PUTS_STRING
	OPCODE_RETURN_NUMBER
)

func create_opcode(opcode int, args []string) OpCode {
	var opcode_ OpCode
	opcode_.opcode = opcode
	opcode_.args = args

	return opcode_
}

func OpCode_Puts(putValue string) OpCode {
	args := []string{putValue}

	return create_opcode(OPCODE_PUTS_STRING, args)
}

func OpCode_Return(return_num float64) OpCode {
	args := []string{fmt.Sprint(return_num)}

	return create_opcode(OPCODE_RETURN_NUMBER, args)
}

func OpCode_Math(math string) OpCode {
	args := []string{math}

	return create_opcode(OPCODE_MATH, args)
}
