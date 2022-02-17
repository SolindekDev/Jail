module tokens

pub const (
	numbers_constants = "0123456789"
	letters_constants = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_?@"
)

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
    multiply
	eof
	null
}

pub const (
	token_name_number      = "number"
	token_name_float       = "float"
	token_name_string      = "string"
	token_name_identifier  = "identifier"
	token_name_newline     = "newline"
	token_name_lpar   	   = "lpar"
	token_name_rpar        = "rpar"
	token_name_plus        = "plus"
	token_name_minus       = "minus"
	token_name_divide      = "divide"
	token_name_multiply    = "multiply"
	token_name_eof		   = "eof"
	token_name_null 	   = "null"
)

struct Position {
	pub mut:
		row int
		col int
}

struct Token {
	pub mut:
		value       string
		filename    string
		type_token  Types
		pos         Position
}

pub fn create_token(filename string, value string, type_token Types, row int, col int) Token {
	mut token := Token{}
	mut pos := Position{row, col}

	token.value         = value
	token.filename      = filename
	token.type_token    = type_token
	token.pos           = pos

	return token
}

pub fn get_string_from_token_type(type_token Types) string { 
    match type_token {
        .number      { return token_name_number }
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
        .eof         { return token_name_eof }
        .null        { return token_name_null }
    }
}

pub fn get_one_char(c1 string) Types {
	match c1 {
		"\n" { return Types.newline }
		"("  { return Types.lpar }
        ")"  { return Types.rpar }
        "+"  { return Types.plus }
        "-"  { return Types.minus }
        "/"  { return Types.divide }
        "*"  { return Types.multiply }
		else {
			return Types.null
		}
	}
}

pub fn print_out_token(token Token) {
	type_ := get_string_from_token_type(token.type_token)
	println("$type_:$token.value")
}