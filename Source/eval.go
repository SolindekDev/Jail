package main

import (
	"fmt"

	"github.com/apaxa-go/eval"
)

func math_calc(math string) interface{} {
	src := "int64(" + math + ")"
	expr, err := eval.ParseString(src, "")

	if err != nil {
		fmt.Println(err)
	}

	r, err := expr.EvalToInterface(nil)

	if err != nil {
		fmt.Println(err)
	}

	return r
}

func eval_init(parser Parser) {
	var last_calc string = ""

	var variable []Variable

	for i := 0; i < len(parser.opcodes); i++ {
		switch parser.opcodes[i].opcode {
		case OPCODE_VARIABLE_DECLARE_MATH:
			r := math_calc(parser.opcodes[i].args[1])
			variable = append(variable, create_variable(parser.opcodes[i].args[0], fmt.Sprint(r)))
			break
		case OPCODE_VARIABLE_DECLARE_STRING:
			variable = append(variable, create_variable(parser.opcodes[i].args[0], parser.opcodes[i].args[1]))
			break
		case OPCODE_PUTS_VARIABLE:
			fmt.Println(found_variable(parser.opcodes[i].args[0], variable).variableValue)
		case OPCODE_MATH:
			math_calc(parser.opcodes[i].args[0])
			break
		case OPCODE_PUTS_STRING:
			fmt.Println(parser.opcodes[i].args[0])
			break
		case OPCODE_PUTS_IDENTIFIER:
			fmt.Printf("Identifier<%s>\n", parser.opcodes[i].args[0])
			break
		case OPCODE_PUTS_MATH:
			r := math_calc(parser.opcodes[i].args[0])

			fmt.Printf("%v\n", r)
		case OPCODE_RETURN_NUMBER:
			last_calc = fmt.Sprint(parser.opcodes[i].args[0])
		case OPCODE_CHANGE_VALUE_VARIABLE_STRING:
			index_var := get_variable_index(parser.opcodes[i].args[0], variable)
			variable[index_var-1].variableValue = fmt.Sprint(parser.opcodes[i].args[1])
		case OPCODE_CHANGE_VALUE_VARIABLE_MATH:
			r := math_calc(parser.opcodes[i].args[1])
			index_var := get_variable_index(parser.opcodes[i].args[0], variable)
			variable[index_var-1].variableValue = fmt.Sprint(r)
		}
	}

	UNUSED(last_calc)

	if last_calc != "" {
		fmt.Printf("%s\n", last_calc)
	}
}
