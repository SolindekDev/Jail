module tokens

// just constants lmao
pub const (
	numbers_constants = "0123456789"
	letters_constants = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_?@"
)

/*
    Jail Programming Language Copyright (C) 2022 SolindekDev

	Contribuitors:
		https://github.com/SolindekDev/Jail/edit/main/contributors.md
*/

// enum that represents the types in language
pub enum Types {
	number = 0
    float
    string_
    identifier
    newline
    lpar
    rpar
    plus
    minus
    divide
	equals
    multiply
	eof
	null
}

// string representation of Types enum
pub const (
	token_name_number      = "Number"
	token_name_float       = "Float"
	token_name_string      = "String"
	token_name_identifier  = "Identifier"
	token_name_newline     = "Newline"
	token_name_lpar   	   = "Lpar"
	token_name_rpar        = "Rpar"
	token_name_plus        = "Plus"
	token_name_minus       = "Minus"
	token_name_divide      = "Divide"
	token_name_equals      = "Equals"
	token_name_multiply    = "Multiply"
	token_name_eof		   = "Eof"
	token_name_null 	   = "Null"
)

// Structure of token poisition
struct Position {
	pub mut:
		row int
		col int
}

// Stucture of token
struct Token {
	pub mut:
		value       string
		filename    string
		type_token  Types
		pos         Position
}

// Function for fast creating tokens
pub fn create_token(filename string, value string, type_token Types, row int, col int) Token {
	mut token := Token{}
	mut pos := Position{row, col}

	token.value         = value
	token.filename      = filename
	token.type_token    = type_token
	token.pos           = pos

	return token
}

// Function for get the string representation of index type enum.
pub fn get_string_from_token_type(type_token Types) string { 
    match type_token {
        .number		 { return token_name_number }
        .float       { return token_name_float }
        .string_     { return token_name_string }
        .identifier  { return token_name_identifier }
        .newline     { return token_name_newline }
        .lpar        { return token_name_lpar }
        .rpar        { return token_name_rpar }
        .plus        { return token_name_plus }
        .minus       { return token_name_minus }
        .divide      { return token_name_divide }
        .multiply    { return token_name_multiply }
        .equals      { return token_name_equals }
        .eof         { return token_name_eof }
        .null        { return token_name_null }
    }
}

// Function to print out token object
pub fn print_out_token(token Token) {
	type_ := get_string_from_token_type(token.type_token)
	println("$type_:$token.value")
}

// Function for get last token from array with out any errors.
pub fn get_last_token(tokens[] Token, filename string) Token {
	if tokens.len == 0 {
		return create_token(
			filename,
			"NIL",
			Types.null,
			0, 
			0
		)
	} else {
		return tokens[tokens.len - 1]
	}
}

// Function that search that the value is equals to any of tokens \ I don't know how to say it lmao
pub fn get_one_char(c1 string) Types {
	match c1 {
		"\n" { return Types.newline }
		"("  { return Types.lpar }
        ")"  { return Types.rpar }
        "+"  { return Types.plus }
        "-"  { return Types.minus }
        "/"  { return Types.divide }
        "*"  { return Types.multiply }
        "="  { return Types.equals }
		else {
			return Types.null
		}
	}
}

// Function that print out all tokens from lexer.
pub fn print_out_all_tokens(tokens[] Token) {
	for i := 0; i < tokens.len; i++ {
		print_out_token(tokens[i])
	}
}