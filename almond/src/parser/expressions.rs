use crate::lexer::tokens::TokenKind;

use super::{Parser, ast::{Expr, Store}};

impl<'a> Parser<'a> {
	fn parse_expression(&mut self, binding_power: u8) -> (Expr, bool) {
		let mut next_requires_end = true;
		let mut this_requires_end = true;

		let mut lhs = match self.peek().unwrap_or(TokenKind::EOF) {
			TokenKind::Ident => {
				self.consume(TokenKind::Ident);
				
				Expr::Ref(self.slice().to_owned())
			},
			TokenKind::String => {
				self.consume(TokenKind::String);

				let slice = self.slice();
				let value = &slice[1..slice.len()-1];

				Expr::from(value.to_owned())
			},
			TokenKind::Int(e) => {
				self.consume(TokenKind::Int(e));

				Expr::from(e)
			},
			TokenKind::Float(e) => {
				self.consume(TokenKind::Float(e));

				Expr::from(e)
			},
			TokenKind::True => {
				self.consume(TokenKind::True);

				Expr::from(true)
			},
			TokenKind::False => {
				self.consume(TokenKind::False);

				Expr::from(false)
			},
			TokenKind::LSquare => {
				self.consume(TokenKind::LSquare);
				let mut out: Vec<Expr> = Vec::new();

				loop {
					let next = self.parse_expression(0);
					out.push(next.0);
					
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

				expr.0
			},
			expr @ TokenKind::Not => {
				self.consume(TokenKind::Not);
				let expr_bp: u8;

				if let Some((_, bp)) = expr.prefix_binding_power() {
					expr_bp = bp;
				} else {
					expr_bp = 0;
				}

				let expr = self.parse_expression(expr_bp);

				Expr::PrefixOp { 
					op: TokenKind::Not, 
					expr: Box::new(expr.0),
				}
			},
			TokenKind::If => {
				self.consume(TokenKind::If);
				let condition = Box::new(self.parse_expression(0).0);
				self.consume(TokenKind::LCurly);
				let then_block = Box::new(self.parse_expression(0).0);

				match self.peek().unwrap_or(TokenKind::EOF) {
					TokenKind::RCurly => {},
					e @ TokenKind::End => self.consume(e),
					kind => panic!("Expected `End` or `RCurly` after if block contents, found {kind}")
				};

				self.consume(TokenKind::RCurly);

				// since all variables need a value, all ifs must have an else
				self.consume(TokenKind::Else);

				match self.peek().unwrap_or(TokenKind::EOF) {
					e @ TokenKind::LCurly => self.consume(e),
					TokenKind::If => {},
					kind => panic!("Expected `If` or `LCurly` after `Else`, found {kind}"),
				}

				let else_block = Box::new(self.parse_expression(0).0);

				match self.peek().unwrap_or(TokenKind::EOF) {
					TokenKind::RCurly => {},
					e @ TokenKind::End => self.consume(e),
					kind => panic!("Expected `End` or `RCurly` after else block contents, found {kind}")
				};

				self.consume(TokenKind::RCurly);

				next_requires_end = false;
				this_requires_end = false;

				Expr::Conditional { condition, then_block, else_block }
			}
			kind => panic!("Unknown start of expression `{}`", kind),
		};

		if let Some(TokenKind::LSquare) = self.peek() {
			self.consume(TokenKind::LSquare);
			let index = self.parse_expression(0);

			lhs = Expr::ArrayAccess { lhs: Box::new(lhs), index: Box::new(index.0) };
			
			self.consume(TokenKind::RSquare);
		}

		loop {
			let peek = self.peek().unwrap_or(TokenKind::EOF);

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
				TokenKind::EOF
				| TokenKind::LCurly
				| TokenKind::RCurly
				| TokenKind::RSquare
				| TokenKind::RParen
				| TokenKind::Comma
				| TokenKind::End => break,
				kind => {
					if !this_requires_end {
						break;
					}

					panic!("Unknown operator: {}", kind)
				}
			};

			if let Some((left_binding_power, right_binding_power)) = op.infix_binding_power() {
				if left_binding_power < binding_power {
					break;
				}
				
				self.consume(&op);
				let rhs = self.parse_expression(right_binding_power);
				lhs = Expr::InfixOp { op: op.clone(), lhs: Box::new(lhs), rhs: Box::new(rhs.0) };

				if let Some(TokenKind::RCurly) = self.peek() {
					return (lhs, false)
				}

				continue;
			}

			break;
		}

		(lhs, next_requires_end)
	}

	/// returns whether the next token should be TokenKind::End
	/// for cases like scopes and conditionals
	fn parse_assign(&mut self, ident: &'a str, output: &mut Store) -> bool {
		self.consume(TokenKind::Assign);

		let value = self.parse_expression(0);

		output.insert(ident.to_owned(), value.0);

		return value.1
	}

	pub(crate) fn parse_input(&mut self, output: &mut Store) {
		loop {
			let next = self.next();

			let end_required = match next.unwrap_or(TokenKind::EOF) {
				TokenKind::Ident => self.parse_assign(self.slice(), output),
				TokenKind::Comment => continue,
				TokenKind::EOF => return,
				_ => panic!("Unexpected token: {}", self.slice()),
			};

			if end_required {
				self.consume(TokenKind::End);
			}
		}
	}
}

trait Operator {
	fn prefix_binding_power(&self) -> Option<((), u8)>;
	fn infix_binding_power(&self) -> Option<(u8, u8)>;
	fn postfix_binding_power(&self) -> Option<(u8, ())>;
}

impl Operator for TokenKind {
	fn prefix_binding_power(&self) -> Option<((), u8)> {
		let result = match self {
			TokenKind::Not => ((), 51),
			_ => return None,
		};

		Some(result)
	}

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

	fn postfix_binding_power(&self) -> Option<(u8, ())> {
		let result = match self {
			TokenKind::LSquare => (53, ()),
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
		let store = eval(r#"
			string = "nice";
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
	fn multiple() {
		let store = eval("nice = 23; cool = 7;");


		assert_eq!(
			store.get_ast("nice"),
			Some(
				&Expr::Literal(
					Literal::Int(23)
				)
			)
		);

		assert_eq!(
			store.get_ast("cool"),
			Some(
				&Expr::Literal(
					Literal::Int(7)
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
	fn array_index() {
		let store = eval("nice = [1, 2, 3, 4]; cool = nice[3];");

		assert_eq!(
			store.get_ast("cool"),
			Some(
				&Expr::ArrayAccess {
					lhs: Box::new(
						Expr::Ref("nice".to_owned())
					),
					index: Box::new(
						Expr::Literal(
							Literal::Int(3)
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

	#[test]
	fn conditional() {
		let store = eval(r#"
			nice = 4;
			cool = if nice < 10 {
				10;
			} else {
				nice;
			}

			epic = cool * 2;
		"#);

		assert_eq!(
			store.get_ast("nice"),
			Some(
				&Expr::Literal(
					Literal::Int(4)
				)
			)
		);

		assert_eq!(
			store.get_ast("cool"),
			Some(
				&Expr::Conditional {
					condition: Box::new(
						Expr::InfixOp {
							lhs: Box::new(
								Expr::Ref("nice".to_owned())
							),
							op: TokenKind::Lt,
							rhs: Box::new(
								Expr::Literal(
									Literal::Int(10)
								)
							)
						}
					),
					then_block: Box::new(
						Expr::Literal(
							Literal::Int(10)
						)
					),
					else_block: Box::new(
						Expr::Ref("nice".to_owned())
					)
				}
			)
		);

		assert_eq!(
			store.get_ast("epic"),
			Some(
				&Expr::InfixOp {
					op: TokenKind::Mul,
					lhs: Box::new(
						Expr::Ref("cool".to_owned())
					),
					rhs: Box::new(
						Expr::Literal(
							Literal::Int(2)
						)
					)
				}
			)
		)
	}
}