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