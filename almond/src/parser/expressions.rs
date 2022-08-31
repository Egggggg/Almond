use crate::lexer::tokens::TokenKind;

use super::{Parser, ast::{Expr, Store}};

// right side binding power for TokenKind::Not
// not put in a function since its the only prefix operator
const NOT_POWER: u8 = 51u8;

impl<'a> Parser<'a> {
	fn parse_expression(&mut self, binding_power: u8) -> Expr {
		let mut lhs = match self.peek().unwrap_or(TokenKind::EOF) {
			TokenKind::Ident => {
				self.next();
				Expr::Ref(self.slice().to_owned())
			},
			TokenKind::String => {
				self.next();
				let slice = self.slice();
				let value = &slice[1..slice.len()-1];

				Expr::from(value.to_owned())
			},
			TokenKind::Int(e) => {
				self.next();
				Expr::from(e)
			},
			TokenKind::Float(e) => {
				self.next();
				Expr::from(e)
			},
			TokenKind::True => {
				self.next();
				Expr::from(true)
			},
			TokenKind::False => {
				self.next();
				Expr::from(false)
			},
			TokenKind::LSquare => {
				self.consume(TokenKind::LSquare);
				let mut out: Vec<Expr> = Vec::new();

				loop {
					let next = self.parse_expression(0);
					out.push(next);
					
					match self.peek().unwrap_or(TokenKind::EOF) {
						TokenKind::Comma => self.consume(TokenKind::Comma),
						TokenKind::RSquare => break,
						kind => panic!("Expected Comma or RSquare, got {}", kind),
					}
				}

				self.consume(TokenKind::RSquare);
				Expr::from(out)
			},
			TokenKind::LParen => {
				self.consume(TokenKind::LParen);
				let expr = self.parse_expression(0);
				self.consume(TokenKind::RParen);

				expr
			},
			TokenKind::Not => {
				self.consume(TokenKind::Not);
				let expr = self.parse_expression(NOT_POWER);

				Expr::PrefixOp { 
					op: TokenKind::Not, 
					expr: Box::new(expr),
				}
			},
			kind => panic!("Unknown start of expression `{}`", kind),
		};

		loop {
			if let Some(peek) = self.peek() {
				let op = match peek {
					op @ TokenKind::As
					| op @ TokenKind::Equals
					| op @ TokenKind::Lt
					| op @ TokenKind::Gt
					| op @ TokenKind::Lte
					| op @ TokenKind::Gte
					| op @ TokenKind::And
					| op @ TokenKind::Or
					| op @ TokenKind::Not
					| op @ TokenKind::Add
					| op @ TokenKind::Sub
					| op @ TokenKind::Mul
					| op @ TokenKind::Div
					| op @ TokenKind::Mod
					| op @ TokenKind::Exp => op,
					TokenKind::EOF => break,
					TokenKind::RCurly
					| TokenKind::RSquare
					| TokenKind::RParen
					| TokenKind::End
					| TokenKind::Comma => break,
					kind => panic!("Unknown operator: {}", kind),
				};

				if let Some((left_binding_power, right_binding_power)) = op.infix_binding_power() {
					if left_binding_power < binding_power {
						break;
					}
					
					self.consume(op);
					let rhs = self.parse_expression(right_binding_power);
					lhs = Expr::InfixOp { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };

					continue;
				}

				break;
			}

			break;
		}

		lhs
	}

	fn parse_assign(&mut self, ident: &'a str, output: &mut Store) {
		self.consume(TokenKind::Assign);

		let value = self.parse_expression(0);

		output.insert(ident.to_owned(), value);
	}

	pub(crate) fn parse_input(&mut self, output: &mut Store) {
		loop {
			let next = self.next();

			match next.unwrap_or(TokenKind::EOF) {
				TokenKind::Ident => self.parse_assign(self.slice(), output),
				TokenKind::Comment => continue,
				TokenKind::EOF => return,
				_ => panic!("Unexpected token: {}", self.slice()),
			}

			self.consume(TokenKind::End);
		}
	}
}

trait Operator {
	fn infix_binding_power(&self) -> Option<(u8, u8)>;
}

impl Operator for TokenKind {
	fn infix_binding_power(&self) -> Option<(u8, u8)> {
		let result = match self {
			TokenKind::As => (1, 2),
			TokenKind::Or => (3, 4),
			TokenKind::And => (5, 6),
			TokenKind::Equals
			| TokenKind::NotEquals => (7, 8),
			TokenKind::Lt
			| TokenKind::Gt
			| TokenKind::Lte
			| TokenKind::Gte => (9, 10),
			TokenKind::Add
			| TokenKind::Sub => (11, 12),
			TokenKind::Mul
			| TokenKind::Div => (13, 14),
			TokenKind::Mod => (15, 16),
			TokenKind::Exp => (17, 18),
			_ => return None,
		};

		Some(result)
	}
}

#[cfg(test)]
mod test {
    use crate::{parser::{eval, ast::{Expr, Literal}}, lexer::tokens::TokenKind};

	#[test]
	fn literal() {
		let store = eval("nice = 23;");

		assert_eq!(
			store.get_ast("nice"),
			Some(
				&Expr::Literal(
					Literal::Int(23)
				)
			)
		)
	}

	#[test]
	fn expr() {
		let store = eval("nice = 23 + 7;");

		assert_eq!(
			store.get_ast("nice"),
			Some(
				&Expr::InfixOp { 
					op: TokenKind::Add,
					lhs: Box::new(
						Expr::Literal(
							Literal::Int(23)
						)
					),
					rhs: Box::new(
						Expr::Literal(
							Literal::Int(7)
						)
					)
				} 
			)
		)
	}

	#[test]
	fn chained_expr() {
		let store = eval("nice = 23 + 7 * 3;");

		assert_eq!(
			store.get_ast("nice"),
			Some(
				&Expr::InfixOp {
					op: TokenKind::Add,
					lhs: Box::new(
						Expr::Literal(
							Literal::Int(23)
						)
					),
					rhs: Box::new(
						Expr::InfixOp {
							op: TokenKind::Mul,
							lhs: Box::new(
								Expr::Literal(
									Literal::Int(7)
								)
							),
							rhs: Box::new(
								Expr::Literal(
									Literal::Int(3)
								)
							)
						}
					)
				}
			)
		)
	}

	#[test]
	fn grouped_expr() {
		let store = eval("nice = (23 + 7) * 3;");

		assert_eq!(
			store.get_ast("nice"),
			Some(
				&Expr::InfixOp {
					op: TokenKind::Mul,
					lhs: Box::new(
						Expr::InfixOp {
							op: TokenKind::Add,
							lhs: Box::new(
								Expr::Literal(
									Literal::Int(23)
								)
							),
							rhs: Box::new(
								Expr::Literal(
									Literal::Int(7)
								)
							)
						}
					),
					rhs: Box::new(
						Expr::Literal(
							Literal::Int(3)
						)
					)
				}
			)
		)
	}
}