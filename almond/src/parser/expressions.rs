use logos::Lexer;

use crate::lexer::tokens::TokenKind;

use super::{Parser, ast::Expr, ast::Store};

impl<'a> Parser<'a> {
	fn parse_expression(&mut self) -> Expr {
		match self.next().unwrap_or(TokenKind::EOF) {
			TokenKind::Ident => Expr::Ref(self.slice().to_owned()),
			TokenKind::String => {
				let slice = self.slice();
				let value = &slice[1..slice.len()-2];

				Expr::from(value.to_owned())
			},
			TokenKind::Int(e) => Expr::from(e),
			TokenKind::Float(e) => Expr::from(e),
			TokenKind::True => Expr::from(true),
			TokenKind::False => Expr::from(false),
			_ => todo!(),
		}
	}

	fn parse_assign(&mut self, ident: &'a str, out: &mut Store) {
		let next = self.next();

		if !matches!(next, Some(TokenKind::Equals)) {
			panic!("`=` expected, found {}", self.slice())
		}

		let value = self.parse_expression();

		out.insert(ident.to_owned(), value);
	}

	pub(crate) fn parse(&mut self) {
		let mut out = Store::new();
	
		loop {
			let next = self.next();
	
			let next = match next {
				None => return,
				Some(e) => e
			};
	
			match next {
				TokenKind::Ident => self.parse_assign(self.slice(), &mut out),
				_ => panic!("Unexpected token: {}", self.slice()),
			}
		}
	}
}

/*
pub fn parse_var<'a>(lex: &'a mut Lexer<'a, TokenKind>, out: &mut Store) -> &'a mut Lexer<'a, TokenKind>{
	let ident = lex.slice().to_owned();
	let next = lex.next().unwrap();
	let mut out_vec: Vec<Expr> = Vec::new();
	let mut state = VarState::Start;

	match next {
		TokenKind::Assign => {},
		_ => panic!("Expected '=', got {}", lex.slice())
	};

	state = VarState::Assign;

	loop {
		let next = lex.next().unwrap();
		let slice = lex.slice();

		let expr: syntax::Statement = match next {
			TokenKind::Int(e) => syntax::Raw::Int(e).into(),
			TokenKind::String => syntax::Raw::String(slice[1..slice.len()-1].to_owned()).into(),
			TokenKind::Float(e) => syntax::Raw::Float(e).into(),
			TokenKind::True => syntax::Raw::Bool(true).into(),
			TokenKind::False => syntax::Raw::Bool(false).into(),
			TokenKind::Ident => syntax::Ident::from(lex.slice()).into(),
			_ => panic!("Expected statement, found {}", slice)
		};

		match lex.next().unwrap() {
			TokenKind::End => {
				match state {
					VarState::Assign => {
						out_vec.push(Expr::output(expr));
						out.insert(ident, ExprChain::new(out_vec));
					},
					VarState::Add(e) => {
            println!("{:#?}", out_vec);
					  if let Some(s) = out_vec.first_mut() {
              s.operand(expr)
            }
          },
					_ => panic!("Invalid state"),
				}

				return lex;
			},
			TokenKind::Add => {
				out_vec.push(Expr::IAssign("z".into(), Box::new(Expr::Add(AddExpr::new(expr, None)))));
				let index = out_vec.len() - 1;
				state = VarState::Add(index);
			},
			_ => panic!("Expected one of (';', '+'), found {}", lex.slice())
		};
	}
}
*/