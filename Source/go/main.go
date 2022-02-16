package main

import (
	"bufio"
	"errors"
	"fmt"
	"io/ioutil"
	"os"
)

var SuccesCode int = 0
var FailedCode int = 1

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

func input() {
	fmt.Printf(">> ")
	var reader = bufio.NewReader(os.Stdin)
	_input, _ := reader.ReadString('\n')

	// now1 := time.Now()

	var lexer Lexer = lexer_init(_input, "Input")
	lexer = refactor_tokens_math(lexer)
	var parser Parser = parser_init(lexer, "Input")
	eval_init(parser)

	// now2 := time.Now()

	// fmt.Printf("%d nanoseconds\n\n", now2.Nanosecond()-now1.Nanosecond())
	// fmt.Printf("%d seconds\n\n", now2.Second()-now1.Second())

	UNUSED(parser)

	input()
}

// main function
func main() {
	argv := os.Args

	if len(argv) < 2 {
		input()
	} else {
		if argv[1] == "--help" || argv[1] == "--h" || argv[1] == "-h" || argv[1] == "-help" {
			fmt.Println("Jail - Interpreter programming language created in GoLang.\n\t- --help - Show this\n\t- --version - Show version\n\t- <filename.j> - Interpreter a file")
			os.Exit(SuccesCode)
		}

		var fileexst bool = file_exists(argv[1])

		if fileexst == false {
			fmt.Println(argv[1] + " <--- This file not exists")
			os.Exit(FailedCode)
		}

		var file_content string = get_file_content(argv[1])

		var lexer Lexer = lexer_init(file_content, argv[1])
		lexer = refactor_tokens_math(lexer)
		var parser Parser = parser_init(lexer, argv[1])
		eval_init(parser)
	}
}
