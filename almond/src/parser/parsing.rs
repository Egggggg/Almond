use crate::lexer::tokens::TokenKind;

use super::{Parser, ast::{Expr, Store}};

impl<'a> Parser<'a> {
	fn parse_expression(&mut self) -> Expr {
		let lhs = match self.next().unwrap_or(TokenKind::EOF) {
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
			TokenKind::LParen => {
				self.consume(TokenKind::LParen);
				let expr = self.parse_expression();
				self.consume(TokenKind::RParen);

				expr
			},
			TokenKind::Not => {
				self.consume(TokenKind::Not);
				
				let expr = self.parse_expression();

				Expr::PrefixOp { 
					op: TokenKind::Not, 
					expr: Box::new(expr),
				}
			},
			kind => panic!("Unknown start of expression `{}`", kind),
		};

		lhs
	}

	fn parse_assign(&mut self, ident: &'a str, output: &mut Store) {
		let next = self.next();
		
		if !matches!(next, Some(TokenKind::Assign)) {
			panic!("`=` expected, found {}", self.slice())
		}

		let value = self.parse_expression();

		output.insert(ident.to_owned(), value);
	}

	pub(crate) fn parse_input(&mut self, output: &mut Store) {
		loop {
			let next = self.next();
	
			let next = match next {
				None => return,
				Some(e) => e
			};
	
			match next {
				TokenKind::Ident => self.parse_assign(self.slice(), output),
				_ => panic!("Unexpected token: {}", self.slice()),
			}
		}
	}
}