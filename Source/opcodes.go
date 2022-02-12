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
	OPCODE_PUTS_MATH
	OPCODE_PUTS_IDENTIFIER
	OPCODE_PUTS_VARIABLE
	OPCODE_RETURN_NUMBER
	OPCODE_VARIABLE_DECLARE_MATH
	OPCODE_VARIABLE_DECLARE_STRING
	OPCODE_CHANGE_VALUE_VARIABLE_MATH
	OPCODE_CHANGE_VALUE_VARIABLE_STRING
)

func OpCode_Change_Value_Varaiable_Math(variableName string, newValue string) OpCode {
	args := []string{variableName, newValue}

	return create_opcode(OPCODE_CHANGE_VALUE_VARIABLE_MATH, args)
}

func OpCode_Change_Value_Varaiable_String(variableName string, newValue string) OpCode {
	args := []string{variableName, newValue}

	return create_opcode(OPCODE_CHANGE_VALUE_VARIABLE_STRING, args)
}

func OpCode_Puts_Variable(variableName string) OpCode {
	args := []string{variableName}

	return create_opcode(OPCODE_PUTS_VARIABLE, args)
}

func OpCode_Variable_Declare_String(variableName string, variableValue string) OpCode {
	args := []string{variableName, variableValue}

	return create_opcode(OPCODE_VARIABLE_DECLARE_STRING, args)
}

func OpCode_Variable_Declare_Math(variableName string, variableValue string) OpCode {
	args := []string{variableName, variableValue}

	return create_opcode(OPCODE_VARIABLE_DECLARE_MATH, args)
}

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

func OpCode_Puts_Math(putValue string) OpCode {
	args := []string{putValue}

	return create_opcode(OPCODE_PUTS_MATH, args)
}

func OpCode_Puts_Identifier(putValue string) OpCode {
	args := []string{putValue}

	return create_opcode(OPCODE_PUTS_IDENTIFIER, args)
}

func OpCode_Return(return_num float64) OpCode {
	args := []string{fmt.Sprint(return_num)}

	return create_opcode(OPCODE_RETURN_NUMBER, args)
}

func OpCode_Math(math string) OpCode {
	args := []string{math}

	return create_opcode(OPCODE_MATH, args)
}
