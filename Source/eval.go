package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
)

func eval_init(parser Parser) {
	var last_calc string = ""

	for i := 0; i < len(parser.opcodes); i++ {
		switch parser.opcodes[i].opcode {
		case OPCODE_ADD_NUMBERS:
			first_arg, _ := strconv.ParseFloat(parser.opcodes[i].args[0], 64)
			if first_arg == 0x0f3019fb {
				calc2, _ := strconv.ParseFloat(last_calc, 64)
				i1, _ := strconv.ParseFloat(parser.opcodes[i].args[1], 64)

				calc := calc2 + i1

				last_calc = fmt.Sprint(calc)
			} else {
				i1, _ := strconv.ParseFloat(parser.opcodes[i].args[0], 64)
				i2, _ := strconv.ParseFloat(parser.opcodes[i].args[1], 64)

				calc := i1 + i2

				last_calc = fmt.Sprint(calc)
			}
			break
		case OPCODE_MINUS_NUMBERS:
			first_arg, _ := strconv.ParseFloat(parser.opcodes[i].args[0], 64)
			if first_arg == 0x0f3019fb {
				calc2, _ := strconv.ParseFloat(last_calc, 64)
				i1, _ := strconv.ParseFloat(parser.opcodes[i].args[1], 64)

				calc := calc2 - i1

				last_calc = fmt.Sprint(calc)
			} else {
				i1, _ := strconv.ParseFloat(parser.opcodes[i].args[0], 64)
				i2, _ := strconv.ParseFloat(parser.opcodes[i].args[1], 64)

				calc := i1 - i2

				last_calc = fmt.Sprint(calc)
			}
			break
		case OPCODE_DIVIDE_NUMBERS:
			first_arg, _ := strconv.ParseFloat(parser.opcodes[i].args[0], 64)
			if first_arg == 0x0f3019fb {
				calc2, _ := strconv.ParseFloat(last_calc, 64)
				i1, _ := strconv.ParseFloat(parser.opcodes[i].args[1], 64)

				if i1 == 0 || calc2 == 0 {
					fmt.Println("MathError: number divide by zero")
					os.Exit(FailedCode)
				}

				calc := calc2 / i1

				last_calc = fmt.Sprint(calc)
			} else {
				i1, _ := strconv.ParseFloat(parser.opcodes[i].args[0], 64)
				i2, _ := strconv.ParseFloat(parser.opcodes[i].args[1], 64)

				if i1 == 0 || i2 == 0 {
					fmt.Println("MathError: number divide by zero")
					os.Exit(FailedCode)
				}

				calc := i1 / i2

				last_calc = fmt.Sprint(calc)
			}
			break
		case OPCODE_MULTIPLY_NUMBERS:
			first_arg, _ := strconv.ParseFloat(parser.opcodes[i].args[0], 64)
			if first_arg == 0x0f3019fb {
				calc2, _ := strconv.ParseFloat(last_calc, 64)
				i1, _ := strconv.ParseFloat(parser.opcodes[i].args[1], 64)

				calc := calc2 * i1

				last_calc = fmt.Sprint(calc)
			} else {
				i1, _ := strconv.ParseFloat(parser.opcodes[i].args[0], 64)
				i2, _ := strconv.ParseFloat(parser.opcodes[i].args[1], 64)

				calc := i1 * i2

				last_calc = fmt.Sprint(calc)
			}
			break
		case OPCODE_MODULUS_NUMBERS:
			first_arg, _ := strconv.ParseFloat(parser.opcodes[i].args[0], 64)
			if first_arg == 0x0f3019fb {
				calc2, _ := strconv.ParseFloat(last_calc, 64)
				i1, _ := strconv.ParseFloat(parser.opcodes[i].args[1], 64)

				calc := math.Remainder(calc2, i1)

				last_calc = fmt.Sprint(calc)
			} else {
				i1, _ := strconv.ParseFloat(parser.opcodes[i].args[0], 64)
				i2, _ := strconv.ParseFloat(parser.opcodes[i].args[1], 64)

				calc := math.Remainder(i1, i2)

				last_calc = fmt.Sprint(calc)
			}
			break
		case OPCODE_RETURN_NUMBER:
			last_calc = fmt.Sprint(parser.opcodes[i].args[0])
		}
	}

	UNUSED(last_calc)

	fmt.Println(last_calc)
}
