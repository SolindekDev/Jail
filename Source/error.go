package main

import "fmt"

func error_print(token Token, errorBody string, errorTitle string, filename string) {
	if token.filename == "Input" {
		token.filename = "Shell"
	}
	fmt.Printf("%s:%d:%d: %s: %s\n", filename, token.pos.row, token.pos.col, errorTitle, errorBody)
}

func error_print_lexer(row int, col int, filename string, errorBody string, errorTitle string) {
	if filename == "Input" {
		filename = "Shell"
	}
	fmt.Printf("%s:%d:%d: %s: %s\n", filename, row, col, errorTitle, errorBody)
}
