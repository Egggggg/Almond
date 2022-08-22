use logos::{Lexer};
use std::collections::HashMap;
use crate::syntax;
use syntax::{TokenKind, Expr};

pub fn parse_var<'a>(lex: &'a mut Lexer<'a, TokenKind>, out: &mut HashMap<&'a str, Vec<Expr<'a>>>) -> &'a mut Lexer<'a, TokenKind>{
	let ident = lex.slice();
	let next = lex.next().unwrap();

	match next {
		TokenKind::Assign => {},
		_ => panic!("Expected '=', got {}", lex.slice())
	};

	let next = lex.next().unwrap();

	let expr = match next {
		TokenKind::Int(e) => syntax::VarType::Int(e),
		TokenKind::String => syntax::VarType::String(lex.slice()),
		TokenKind::Float(e) => syntax::VarType::Float(e),
		TokenKind::True => syntax::VarType::Bool(true),
		TokenKind::False => syntax::VarType::Bool(false),
		_ => panic!("Expected statement, got {}", lex.slice())
	};

	match lex.next().unwrap() {
		TokenKind::End => out.insert(ident, vec![Expr::assign_literal(ident, expr)]),
		_ => panic!("Expected ';', got {}", lex.slice())
	};

	return lex;
}

pub fn parse<'a>(mut lex: &'a mut Lexer<'a, TokenKind>) -> HashMap<&'a str, Vec<Expr<'a>>> {
	let mut out: HashMap<&str, Vec<Expr>> = HashMap::new();

	loop {
		let next = lex.next();

		let next = match next {
			None => return out,
			Some(e) => e
		};

		match next {
			TokenKind::Ident => lex = parse_var(lex, &mut out),
			_ => panic!("Unexpected token: {}", lex.slice()),
		}
	}
}