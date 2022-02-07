package main

import (
	"fmt"
	"os"
	"strconv"
)

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
