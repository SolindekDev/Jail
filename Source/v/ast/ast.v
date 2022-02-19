/*
    Jail Programming Language Copyright (C) 2022 SolindekDev

	Contribuitors:
		https://github.com/SolindekDev/Jail/edit/main/contributors.md
*/

module ast

/* Import some packages */
import tokens
import error

pub fn ast_init(tokens_[] tokens.Token) MainAST {
	mut main_ast := create_main_ast()

	mut freeze     	:= 0   	 // Variable of ast.
	mut errors    	:= false  // Variable of ast.
	mut number    	:= false  // Variable of ast.

	for i := 0; i < tokens_.len; i++ { // Loop through all lexer tokens
		if freeze != 0 { // Freeze is freezing loop work
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
		   tokens_[i].type_token == tokens.Types.multiply { // Math!
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
							  	last_t := main_ast.body[main_ast.body.len-1].body_tokens[main_ast.body[main_ast.body.len-1].body_tokens.len-1]
							  // (1+2)*3/4
							  	// if (main_ast.body[main_ast.body.len-1].body_tokens[main_ast.body[main_ast.body.len-1].body_tokens.len-1].type_token != tokens.Types.lpar) == false ||
								//    (main_ast.body[main_ast.body.len-1].body_tokens[main_ast.body[main_ast.body.len-1].body_tokens.len-1].type_token != tokens.Types.rpar) == false {
								// 	    println((main_ast.body[main_ast.body.len-1].body_tokens[main_ast.body[main_ast.body.len-1].body_tokens.len-1].type_token != tokens.Types.rpar) == false)
								// 	    error.error_print_lexer(
								// 			tokens_[i].pos.row,
								// 			tokens_[i].pos.col,
								// 			tokens_[i].filename,
								// 			"Expected an number after an operator: " + tokens_[i].value,
								// 			"SyntaxError"
								// 		)
								// 		errors=true
								//    }
								if last_t.type_token == tokens.Types.rpar || last_t.type_token == tokens.Types.lpar {
									main_ast.body[main_ast.body.len-1].body_tokens << tokens_[i]
								} else {
									error.error_print_lexer(
										tokens_[i].pos.row,
										tokens_[i].pos.col,
										tokens_[i].filename,
										"Expected an number after an operator: " + tokens_[i].value,
										"SyntaxError"
									)
									errors=true
								}
						} else if tokens_[i].type_token == tokens.Types.lpar ||
						          tokens_[i].type_token == tokens.Types.rpar {
							main_ast.body[main_ast.body.len-1].body_tokens << tokens_[i]
						} else {
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
					if tokens_[i].type_token == tokens.Types.lpar {
						body_tokens := [tokens_[i]]
						main_ast.body << create_node_ast(
							TypeExpressionAST.ast_math_operation,
							[],
							[],
							body_tokens
						)
						number = true
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
		} else if tokens_[i].type_token == tokens.Types.identifier {
			if tokens_[i].value == keyword_puts {
				if tokens_.len > i+1 {
					if tokens_[i+1].type_token == tokens.Types.string_ {
						body_tokens := [tokens_[i+1]]
						main_ast.body << create_node_ast(
							TypeExpressionAST.ast_puts_build_in_function,
							["string"],
							[],
							body_tokens
						)
						freeze += 1
					} else if
						(
							tokens_[i+1].type_token == tokens.Types.number ||
							tokens_[i+1].type_token == tokens.Types.float  ||
							tokens_[i+1].type_token == tokens.Types.rpar   ||
							tokens_[i+1].type_token == tokens.Types.lpar
						) && (
							tokens_.len > i+2
						) && (
							tokens_[i+2].type_token == tokens.Types.plus      ||
							tokens_[i+2].type_token == tokens.Types.minus     ||
							tokens_[i+2].type_token == tokens.Types.divide    ||
							tokens_[i+2].type_token == tokens.Types.multiply  ||
							tokens_[i+2].type_token == tokens.Types.rpar      ||
							tokens_[i+2].type_token == tokens.Types.lpar			||
							tokens_[i+2].type_token == tokens.Types.number    ||
							tokens_[i+2].type_token == tokens.Types.float
						) {
							println("s")
							for ;; {
								if tokens_.len > i {
									if tokens_[i].type_token == tokens.Types.float     ||
									   tokens_[i].type_token == tokens.Types.number    ||
									   tokens_[i].type_token == tokens.Types.lpar      ||
									   tokens_[i].type_token == tokens.Types.rpar      ||
									   tokens_[i].type_token == tokens.Types.plus      ||
									   tokens_[i].type_token == tokens.Types.minus     ||
									   tokens_[i].type_token == tokens.Types.divide    ||
									   tokens_[i].type_token == tokens.Types.multiply { // Math!
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
														  	last_t := main_ast.body[main_ast.body.len-1].body_tokens[main_ast.body[main_ast.body.len-1].body_tokens.len-1]
														  // (1+2)*3/4
														  	// if (main_ast.body[main_ast.body.len-1].body_tokens[main_ast.body[main_ast.body.len-1].body_tokens.len-1].type_token != tokens.Types.lpar) == false ||
															//    (main_ast.body[main_ast.body.len-1].body_tokens[main_ast.body[main_ast.body.len-1].body_tokens.len-1].type_token != tokens.Types.rpar) == false {
															// 	    println((main_ast.body[main_ast.body.len-1].body_tokens[main_ast.body[main_ast.body.len-1].body_tokens.len-1].type_token != tokens.Types.rpar) == false)
															// 	    error.error_print_lexer(
															// 			tokens_[i].pos.row,
															// 			tokens_[i].pos.col,
															// 			tokens_[i].filename,
															// 			"Expected an number after an operator: " + tokens_[i].value,
															// 			"SyntaxError"
															// 		)
															// 		errors=true
															//    }
															if last_t.type_token == tokens.Types.rpar || last_t.type_token == tokens.Types.lpar {
																main_ast.body[main_ast.body.len-1].body_tokens << tokens_[i]
															} else {
																error.error_print_lexer(
																	tokens_[i].pos.row,
																	tokens_[i].pos.col,
																	tokens_[i].filename,
																	"Expected an number after an operator: " + tokens_[i].value,
																	"SyntaxError"
																)
																errors=true
															}
													} else if tokens_[i].type_token == tokens.Types.lpar ||
													          tokens_[i].type_token == tokens.Types.rpar {
														main_ast.body[main_ast.body.len-1].body_tokens << tokens_[i]
													} else {
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
												if tokens_[i].type_token == tokens.Types.lpar {
													body_tokens := [tokens_[i]]
													main_ast.body << create_node_ast(
														TypeExpressionAST.ast_math_puts,
														[],
														[],
														body_tokens
													)
													number = true
												} else {
													body_tokens := [tokens_[i]]
													main_ast.body << create_node_ast(
														TypeExpressionAST.ast_math_puts,
														[],
														[],
														body_tokens
													)
													number = true
												}
											} else {
												body_tokens := [tokens_[i]]
												main_ast.body << create_node_ast(
													TypeExpressionAST.ast_math_puts,
													[],
													[],
													body_tokens
												)
												number = true
											}
										}
									}
									} else {
										break
									}
									i++
							}
						}
					else if tokens_[i+1].type_token == tokens.Types.number {
						body_tokens := [tokens_[i+1]]
						main_ast.body << create_node_ast(
							TypeExpressionAST.ast_puts_build_in_function,
							["int"],
							[],
							body_tokens
						)
						freeze += 1
					} else if tokens_[i+1].type_token == tokens.Types.float {
						body_tokens := [tokens_[i+1]]
						main_ast.body << create_node_ast(
							TypeExpressionAST.ast_puts_build_in_function,
							["float"],
							[],
							body_tokens
						)
						freeze += 1
					} else {
						type_ := tokens_[i+1].type_token
						error.error_print_lexer(
							tokens_[i].pos.row,
							tokens_[i].pos.col,
							tokens_[i].filename,
							"Expected an string type after puts keyword no $type_ type.",
							"SyntaxError"
						)
						errors = true
					}
				} else {
					error.error_print_lexer(
						tokens_[i].pos.row,
						tokens_[i].pos.col,
						tokens_[i].filename,
						"Expected an string after puts keyword",
						"SyntaxError"
					)
					errors = true
				}
			} else if tokens_[i].value == keyword___rust {
				if tokens_.len > i+1 {
					 if tokens_[i+1].type_token == tokens.Types.string_ {
					 	body_tokens := [tokens_[i+1]]
 						main_ast.body << create_node_ast(
 							TypeExpressionAST.ast___rust_input_code,
 							[],
 							[],
 							body_tokens
 						)
 						freeze += 1
					 } else {
							 type_ := tokens_[i+1].type_token
							 error.error_print_lexer(
		 						tokens_[i].pos.row,
		 						tokens_[i].pos.col,
		 						tokens_[i].filename,
		 						"Expected an string after __rust keyword no $type_ type.",
		 						"SyntaxError"
		 					)
		 					errors = true
					 }
				} else {
					error.error_print_lexer(
						tokens_[i].pos.row,
						tokens_[i].pos.col,
						tokens_[i].filename,
						"Expected an string after __rust keyword",
						"SyntaxError"
					)
					errors = true
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
