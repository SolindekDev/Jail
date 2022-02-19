/*
    Jail Programming Language Copyright (C) 2022 SolindekDev

	Contribuitors:
		https://github.com/SolindekDev/Jail/edit/main/contributors.md
*/

module compiler

/* Import some packages */
import os
import rand
import ast
import tokens

/* Compile string to byte array */
pub fn str_to_byte(str string) []byte {
	mut bytes := []byte{}

	for i in str {
		bytes << i
	}

	return bytes
}

/* This function easlly write into file */
pub fn writef(mut fs os.File, str string) {
	fs.write(
		str_to_byte(str)
	) or {
		println("cannot write to out.rs. not enough permissions.")
		exit(1)
	}
}

pub fn compiler_init(mast ast.MainAST, out_flag bool) {
	/* Open file */
	mut out := os.create("out.rs") or {
		println("out.rs already exists")
		exit(1)
	}

	/* This code generate standard code */
	writef(mut out, "/*
    Jail Programming Language Copyright (C) 2022 SolindekDev
        - Code generated by Jail Programming Language
*/\n")
	writef(mut out, "
use std::io;
use std::process;\n") // Standard library
	writef(mut out, "
#[warn(unused_variables)]
#[warn(unused_)]
#[warn(unused_imports)]
#[warn(unused_features)]
#[warn(dead_code)]
#[warn(non_snake_case)]
#[warn(non_upper_case_globals)]
#[warn(unreachable_code)]
#[warn(while_true)]
#[warn(unused_unsafe)]\n") // Some warns
	writef(mut out, "
type byte    = u8;
type sbyte   = i8;
type ushort  = u16;
type short   = i16;
type uint    = u32;
type int     = i32;
type ulong   = u64;
type long    = i64;
type float   = f32;
type double  = f64;\n") // Some standard types
	writef(mut out, "
fn main() {
") // Create function main

	/* This function generate code from ast nodes */
	for i := 0; i < mast.body.len; i++ {
		actual_node_ast := mast.body[i] // Store in variable acutal node ast
		if actual_node_ast.type_ast == ast.TypeExpressionAST.ast_math_operation { // If acutal node is a math operation
			body_tokens   := actual_node_ast.body_tokens // Store in variable acutal node body tokens
			mut operation := "" // Store in variable that what will be send into output file write

			for j := 0; j < body_tokens.len; j++ { // Loop through the body tokens
				if body_tokens[j].type_token == tokens.Types.number { // if body token is and number (int type)
					t := body_tokens[j].value // Store in variable actual node body token value
					operation += "($t) as float" // Add into operation variable an code
				} else { // if body token is else
					operation += body_tokens[j].value // Add into operation variable an code
				}
			}

			name := rand.i64_in_range(0, 2000000) // Generate the name for variable
			writef(mut out, "	let calculations_$name: float = ($operation) as float;\n") // Generate code
		} else if actual_node_ast.type_ast == ast.TypeExpressionAST.ast_puts_build_in_function { // If actual node is puts build in function
			if actual_node_ast.arguments[0] == 'int' || actual_node_ast.arguments[0] == 'float' { // If thing to print out is type int or float
				ac_token := actual_node_ast.body_tokens[0].value // Store in variable value to print out
				writef(mut out, '	println!("{}", $ac_token);\n') // Generate code
			} else { // If thing to print out is something else than float or int
				ac_token := actual_node_ast.body_tokens[0].value // Store in variable value to print out
				writef(mut out, '	println!("$ac_token");\n') // Generate code
			}
		} else if actual_node_ast.type_ast == ast.TypeExpressionAST.ast_math_puts { // If actual node is math operation puts
			body_tokens   := actual_node_ast.body_tokens // Store in variable acutal node body tokens
			mut operation := "" // Store in variable that what will be send into output file write

			for j := 0; j < body_tokens.len; j++ { // Loop through the body tokens
				if body_tokens[j].type_token == tokens.Types.number { // if body token is and number (int type)
					t := body_tokens[j].value // Store in variable actual node body token value
					operation += "($t) as float" // Add into operation variable an code
				} else { // if body token is else
					operation += body_tokens[j].value // Add into operation variable an code
				}
			}

			writef(mut out, "	println!(\"{}\", ($operation) as float);\n") // Generate code
		} else if actual_node_ast.type_ast == ast.TypeExpressionAST.ast___rust_input_code { // If actual node is __rust input code
			code := actual_node_ast.body_tokens[0].value.replace("`", '"') // Store in variable code to generate
			writef(mut out, "	/* Code from __rust keyword */\n$code\n	/* Code end. */\n") // Generate code!
		}
	}
	writef(mut out, "}") // End function main

	out.close() // Close file

	os.execute("rustc out.rs") // Execute the compiler
	if out_flag == false { // Is there -o flag?
		os.execute("rm out.rs") // if no delete the source code
	}
	os.execute("rm out.pdb") // some rubish bruh
}
