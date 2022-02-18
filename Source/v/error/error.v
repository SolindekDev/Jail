/*
    Jail Programming Language Copyright (C) 2022 SolindekDev

	Contribuitors:
		https://github.com/SolindekDev/Jail/edit/main/contributors.md
*/

module error

/*

	Unicode colors code for there:
	https://gist.github.com/Prakasaka/219fe5695beeb4d6311583e79933a009

*/

// Simple function to print errors in lexer
pub fn error_print_lexer(
	row int,
	col int,
	filename string,
	body string,
	title string
) {
	println("\e[1;33m$filename:$row:$col: \e[0m\e[1;91m$title: \e[0m$body")
}