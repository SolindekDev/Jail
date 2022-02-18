/*
    Jail Programming Language Copyright (C) 2022 SolindekDev

	Contribuitors:
		https://github.com/SolindekDev/Jail/edit/main/contributors.md
*/

module ast

/* Import some packages */
import tokens

// TypeAST Enum
pub enum TypeAST {
	ast_program = 0
}

// TypeExpressionAST Enum
pub enum TypeExpressionAST {
	ast_math_operation = 0
}

// NodeAST Structure
struct NodeAST {
	pub mut:
		type_ast       TypeExpressionAST
		arguments[]    string
		expression[]   string
		body_tokens[]  tokens.Token
}

// MainAST Structure
struct MainAST {		
	pub mut:
		type_ast  TypeAST
		body[]    NodeAST
}	

// Simple function that return a done MainAST
fn create_main_ast() MainAST {
	mut main_ast := MainAST{}

	main_ast.type_ast = TypeAST.ast_program

	return main_ast
}

// Simple function that return a done NodeAST
fn create_node_ast(type_ast TypeExpressionAST, arguments[] string, expression[] string, body_tokens[] tokens.Token) NodeAST {
	mut node_ast := NodeAST{}

	node_ast.type_ast     = type_ast
	node_ast.expression   = expression
	node_ast.arguments    = arguments
	node_ast.body_tokens  = body_tokens

	return node_ast
}