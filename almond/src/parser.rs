use logos::{Lexer};
use std::collections::HashMap;
use crate::syntax;
use syntax::{TokenKind, Expr, ExprChain};

pub fn parse_var<'a>(lex: &'a mut Lexer<'a, TokenKind>, out: &mut HashMap<&'a str, ExprChain>) -> &'a mut Lexer<'a, TokenKind>{
	let ident = lex.slice();
	let next = lex.next().unwrap();

	match next {
		TokenKind::Assign => {},
		_ => panic!("Expected '=', got {}", lex.slice())
	};

	let next = lex.next().unwrap();
	let slice = lex.slice();

	let expr = match next {
		TokenKind::Int(e) => syntax::Raw::Int(e),
		TokenKind::String => syntax::Raw::String(slice[1..slice.len()-1].to_owned()),
		TokenKind::Float(e) => syntax::Raw::Float(e),
		TokenKind::True => syntax::Raw::Bool(true),
		TokenKind::False => syntax::Raw::Bool(false),
		_ => panic!("Expected statement, got {}", slice)
	};

	match lex.next().unwrap() {
		TokenKind::End => out.insert(ident, ExprChain { chain: vec![Expr::output_literal(ident.to_owned(), expr)] }),
		_ => panic!("Expected ';', got {}", lex.slice())
	};

	return lex;
}

pub fn parse<'a>(mut lex: &'a mut Lexer<'a, TokenKind>) -> HashMap<&'a str, ExprChain> {
	let mut out: HashMap<&str, ExprChain> = HashMap::new();

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