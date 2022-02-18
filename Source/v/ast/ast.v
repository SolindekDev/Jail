module ast

import tokens
import error

pub fn ast_init(tokens_[] tokens.Token) MainAST {
	mut main_ast := create_main_ast()

	mut freeze    := 0
	mut errors    := false
	mut number    := false

	for i := 0; i < tokens_.len; i++ {
		if freeze != 0 {
			freeze--
			continue
		}

		if tokens_[i].type_token == tokens.Types.float     || 
		   tokens_[i].type_token == tokens.Types.number    ||
		   tokens_[i].type_token == tokens.Types.lpar      ||
		   tokens_[i].type_token == tokens.Types.rpar      ||
		   tokens_[i].type_token == tokens.Types.plus      ||
		   tokens_[i].type_token == tokens.Types.minus     ||
		   tokens_[i].type_token == tokens.Types.divide    ||
		   tokens_[i].type_token == tokens.Types.multiply { 
			if number == true {
				if main_ast.body[main_ast.body.len-1].body_tokens[main_ast.body[main_ast.body.len-1].body_tokens.len-1].type_token == tokens.Types.number ||
				   main_ast.body[main_ast.body.len-1].body_tokens[main_ast.body[main_ast.body.len-1].body_tokens.len-1].type_token == tokens.Types.float { 
						if tokens_[i].type_token == tokens.Types.number || tokens_[i].type_token == tokens.Types.float { 
							error.error_print_lexer(
									tokens_[i].pos.row,
									tokens_[i].pos.col,
									tokens_[i].filename,
									"Expected an operator after number no another number: " + tokens_[i].value,
									"SyntaxError"
								)
								errors=true
						} else {
							main_ast.body[main_ast.body.len-1].body_tokens << tokens_[i]
						}
				   } else {
					   if tokens_[i].type_token == tokens.Types.plus      ||
						  tokens_[i].type_token == tokens.Types.minus     ||
						  tokens_[i].type_token == tokens.Types.divide    ||
						  tokens_[i].type_token == tokens.Types.multiply { 
								error.error_print_lexer(
									tokens_[i].pos.row,
									tokens_[i].pos.col,
									tokens_[i].filename,
									"Expected an number after an operator: " + tokens_[i].value,
									"SyntaxError"
								)
								errors=true
						} else if tokens_[i].type_token == tokens.Types.lpar ||
						          tokens_[i].type_token == tokens.Types.rpar {
							
						} else
							main_ast.body[main_ast.body.len-1].body_tokens << tokens_[i]
						}
				   }
				// main_ast.body[main_ast.body.len-1].body_tokens << tokens_[i]
			} else {
				if tokens_[i].type_token == tokens.Types.plus      ||
				   tokens_[i].type_token == tokens.Types.minus     ||
				   tokens_[i].type_token == tokens.Types.divide    ||
				   tokens_[i].type_token == tokens.Types.multiply { 
				    error.error_print_lexer(
						tokens_[i].pos.row,
						tokens_[i].pos.col,
						tokens_[i].filename,
						"Expected an number before operator",
						"SyntaxError"
					)
					errors=true
				} else if tokens_[i].type_token == tokens.Types.lpar ||
						  tokens_[i].type_token == tokens.Types.rpar {
							
				} else {
					body_tokens := [tokens_[i]]
					main_ast.body << create_node_ast(
						TypeExpressionAST.ast_math_operation,
						[],
						[],
						body_tokens
					)
					number = true
				}		
			}
		} else if tokens_[i].type_token == tokens.Types.newline { 
			number = false
		}
	}

	// println(main_ast)

	if errors == true {
		exit(1) // Failed code
	}

	return main_ast
}