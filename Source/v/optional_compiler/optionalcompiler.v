module optional_compiler

import ast
import os
import json

struct RustDefinitions {
	puts_string string
	puts_math string
	rust string
	math string
}

fn generate_keyword(Def string, values string) string{
	return Def.replace("&", values)
}

fn generate_math_string(node ast.NodeAST) string{
	mut math_string := ""
	for token in node.body_tokens {
		math_string+=token.value
	}
	return math_string
}

pub fn optional_compile(AST ast.MainAST){
	mut definitions_json := os.read_lines("optional_compiler/rust_definitions.json") or {		
		exit(1)
	}	
	mut definitions_json_data := ""
	for i in definitions_json{
		definitions_json_data+=i
	}
	rust_definitions_data := json.decode(RustDefinitions, definitions_json_data) or {
		exit(1)
	}

	mut rust_code := []string{}
	for node in AST.body{
		rust_code << match node.type_ast {
			.ast_puts_build_in_function{generate_keyword(rust_definitions_data.puts_string, node.body_tokens[0].value)}
			.ast_math_puts{
				math_string := generate_math_string(node)
				generate_keyword(rust_definitions_data.puts_math, math_string)
			}
			.ast_math_operation{
				math_string := generate_math_string(node)
				generate_keyword(rust_definitions_data.math, math_string)
			}
			.ast___rust_input_code{generate_keyword(rust_definitions_data.rust, node.body_tokens[0].value)}
			//else{"nothing"}
		}		
	}
	println(rust_code)
}