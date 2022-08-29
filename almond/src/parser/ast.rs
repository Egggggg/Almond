use std::collections::HashMap;

use crate::lexer::tokens::TokenKind;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
	Array(Vec<Literal>),
	Circular(String),
	None,
}

#[derive(Debug)]
pub enum Expr {
	Literal(Literal),
	Ref(String),
	Scope(Vec<Expr>),
	IScope(Vec<Expr>),
	FnCall { fn_name: String, args: Vec<Expr> },
	PrefixOp {op: TokenKind, expr: Box<Expr> },
	InfixOp { op: TokenKind, lhs: Box<Expr>, rhs: Box<Expr> },
    Conditional { condition: Box<Expr>, block: Box<Expr>, else_block: Box<Expr> },
}

impl From<String> for Expr {
	fn from(other: String) -> Expr {
		Expr::Literal(Literal::String(other))
	}
}

impl From<i64> for Expr {
	fn from(other: i64) -> Expr {
		Expr::Literal(Literal::Int(other))
	}
}

impl From<f64> for Expr {
	fn from(other: f64) -> Expr {
		Expr::Literal(Literal::Float(other))
	}
}

impl From<bool> for Expr {
	fn from(other: bool) -> Expr {
		Expr::Literal(Literal::Bool(other))
	}
}

impl From<Vec<Literal>> for Expr {
	fn from(other: Vec<Literal>) -> Expr {
		Expr::Literal(Literal::Array(other))
	}
}

#[derive(Debug)]
pub struct Store {
	pub contents: HashMap<String, Expr>,
}

impl Store {
	pub fn new() -> Store {
		let contents: HashMap<String, Expr> = HashMap::new();
		
		Store { contents }
	}

	pub fn insert(&mut self, key: String, expr: Expr) -> Option<Expr> {
		self.contents.insert(key, expr)
	}
}