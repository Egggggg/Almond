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
					op @ TokenKind::Equals
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
					| op @ TokenKind::Exp
					| op @ TokenKind::Range
					| op @ TokenKind::IRange => op,
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
					
					self.consume(&op);
					let rhs = self.parse_expression(right_binding_power);
					lhs = Expr::InfixOp { op: op.clone(), lhs: Box::new(lhs), rhs: Box::new(rhs) };

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
			TokenKind::Or => (1, 2),
			TokenKind::And => (3, 4),
			TokenKind::Equals
			| TokenKind::NotEquals => (5, 6),
			TokenKind::Lt
			| TokenKind::Gt
			| TokenKind::Lte
			| TokenKind::Gte => (7, 8),
			TokenKind::Add
			| TokenKind::Sub => (9, 10),
			TokenKind::Mul
			| TokenKind::Div => (11, 12),
			TokenKind::Mod => (13, 14),
			TokenKind::Exp => (15, 16),
			TokenKind::Range
			| TokenKind::IRange => (17, 18),
			_ => return None,
		};

		Some(result)
	}
}

#[cfg(test)]
mod test {
    use crate::{parser::{eval, ast::{Expr, Literal}}, lexer::tokens::TokenKind};

	#[test]
	fn atomics() {
		let store = eval(r#"string = "nice";
							int = 23;
							float = 324.2356;
							boolF = false;
							boolT = true;
						"#);


		assert_eq!(
			store.get_ast("string"),
			Some(
				&Expr::Literal(
					Literal::String("nice".to_owned())
				)
			)
		);

		assert_eq!(
			store.get_ast("int"),
			Some(
				&Expr::Literal(
					Literal::Int(23)
				)
			)
		);

		assert_eq!(
			store.get_ast("float"),
			Some(
				&Expr::Literal(
					Literal::Float(324.2356)
				)
			)
		);

		assert_eq!(
			store.get_ast("boolF"),
			Some(
				&Expr::Literal(
					Literal::Bool(false)
				)
			)
		);

		assert_eq!(
			store.get_ast("boolT"),
			Some(
				&Expr::Literal(
					Literal::Bool(true)
				)
			)
		);
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

	#[test]
	fn array() {
		let store = eval(r#"nice = ["cool", 1, 3.245, true, false];"#);

		assert_eq!(
			store.get_ast("nice"),
			Some(
				&Expr::Literal(
					Literal::Array(
						vec![
							Expr::Literal(
								Literal::String("cool".to_owned())
							),
							Expr::Literal(
								Literal::Int(1)
							),
							Expr::Literal(
								Literal::Float(3.245)
							),
							Expr::Literal(
								Literal::Bool(true)
							),
							Expr::Literal(
								Literal::Bool(false)
							)
						]
					)
				)
			)
		)
	}

	#[test]
	fn nested_array() {
		let store = eval("nice = [1, 2, [3, 4]];");

		assert_eq!(
			store.get_ast("nice"),
			Some(
				&Expr::Literal(
					Literal::Array(
						vec![
							Expr::Literal(
								Literal::Int(1)
							),
							Expr::Literal(
								Literal::Int(2)
							),
							Expr::Literal(
								Literal::Array(
									vec![
										Expr::Literal(
											Literal::Int(3)
										),
										Expr::Literal(
											Literal::Int(4)
										)
									]
								)
							)
						]
					)
				)
			)
		)
	}

	#[test]
	fn array_with_expr() {
		let store = eval("nice = [1 + 4, 6, 5 * 2, 11];");

		assert_eq!(
			store.get_ast("nice"),
			Some(
				&Expr::Literal(
					Literal::Array(
						vec![
							Expr::InfixOp {
								op: TokenKind::Add,
								lhs: Box::new(
									Expr::Literal(
										Literal::Int(1)
									)
								),
								rhs: Box::new(
									Expr::Literal(
										Literal::Int(4)
									)
								)
							},
							Expr::Literal(
								Literal::Int(6)
							),
							Expr::InfixOp {
								op: TokenKind::Mul,
								lhs: Box::new(
									Expr::Literal(
										Literal::Int(5)
									)
								),
								rhs: Box::new(
									Expr::Literal(
										Literal::Int(2)
									)
								)
							},
							Expr::Literal(
								Literal::Int(11)
							)
						]
					)
				)
			)
		)
	}

	#[test]
	fn array_in_expr() {
		let store = eval("nice = [1, 2, 3, 4] + [4, 3, 2, 1];");

		assert_eq!(
			store.get_ast("nice"),
			Some(
				&Expr::InfixOp {
					op: TokenKind::Add,
					lhs: Box::new(
						Expr::Literal(
							Literal::Array(
								vec![
									Expr::Literal(
										Literal::Int(1)
									),
									Expr::Literal(
										Literal::Int(2)
									),
									Expr::Literal(
										Literal::Int(3)
									),
									Expr::Literal(
										Literal::Int(4)
									),
								]
							)
						)
					),
					rhs: Box::new(
						Expr::Literal(
							Literal::Array(
								vec![
									Expr::Literal(
										Literal::Int(4)
									),
									Expr::Literal(
										Literal::Int(3)
									),
									Expr::Literal(
										Literal::Int(2)
									),
									Expr::Literal(
										Literal::Int(1)
									),
								]
							)
						)
					)
				}
			)
		)
	}

	#[test]
	fn ranges() {
		let store = eval("nice = 1..5; cool = 1..=5;");

		assert_eq!(
			store.get_ast("nice"),
			Some(
				&Expr::InfixOp {
					op: TokenKind::Range,
					lhs: Box::new(
						Expr::Literal(
							Literal::Int(1)
						)
					),
					rhs: Box::new(
						Expr::Literal(
							Literal::Int(5)
						)
					)
				}
			)
		);

		assert_eq!(
			store.get_ast("cool"),
			Some(
				&Expr::InfixOp {
					op: TokenKind::IRange,
					lhs: Box::new(
						Expr::Literal(
							Literal::Int(1)
						)
					),
					rhs: Box::new(
						Expr::Literal(
							Literal::Int(5)
						)
					)
				}
			)
		)
	}

	#[test]
	fn ranges_in_expr() {
		let store = eval("nice = 1..5 * 3;");

		assert_eq!(
			store.get_ast("nice"),
			Some(
				&Expr::InfixOp {
					op: TokenKind::Mul,
					lhs: Box::new(
						Expr::InfixOp {
							op: TokenKind::Range,
							lhs: Box::new(
								Expr::Literal(
									Literal::Int(1)
								)
							),
							rhs: Box::new(
								Expr::Literal(
									Literal::Int(5)
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