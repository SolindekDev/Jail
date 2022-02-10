package main

import (
	"fmt"

	"github.com/apaxa-go/eval"
)

func eval_init(parser Parser) {
	var last_calc string = ""

	for i := 0; i < len(parser.opcodes); i++ {
		switch parser.opcodes[i].opcode {
		case OPCODE_MATH:
			src := "int8(" + parser.opcodes[i].args[0] + ")"
			expr, err := eval.ParseString(src, "")

			if err != nil {
				fmt.Println(err)
			}

			r, err := expr.EvalToInterface(nil)

			if err != nil {
				fmt.Println(err)
			}

			fmt.Printf("%v\n", r)

		case OPCODE_PUTS_STRING:
			fmt.Println(parser.opcodes[i].args[0])
			break
		case OPCODE_RETURN_NUMBER:
			last_calc = fmt.Sprint(parser.opcodes[i].args[0])
		}
	}

	UNUSED(last_calc)

	if last_calc != "" {
		fmt.Printf("%s\n", last_calc)
	}
}
