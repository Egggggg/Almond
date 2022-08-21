mod syntax;

use logos::{Lexer, Logos};
use std::collections::HashMap;
use syntax::{TokenKind, Expr};

fn parse_var(lex: &mut Lexer<TokenKind>, out: &mut HashMap<&str, Vec<Expr>>) {
	
}

fn parse(lex: &mut Lexer<TokenKind>) -> HashMap<&str, Vec<Expr>> {
	let mut out: HashMap<&str, Vec<Expr>> = HashMap::new();

	loop {
		let next = lex.next().unwrap_or_else(|| return out);

		match next {
			TokenKind::Ident => parse_var(lex, &mut out),
			_ => panic!("Unexpected token: {}", lex.slice()),
		}
	}
}

fn main() {
    let mut lex = TokenKind::lexer("nice = 4;");
	
}
