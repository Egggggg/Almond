use std::{collections::HashMap, ops::{Range, RangeInclusive}};

use crate::lexer::tokens::TokenKind;

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
	Array(Vec<Expr>),
	Circular(String),
	None,
}

impl PartialEq<Literal> for Literal {
	fn eq(&self, other: &Literal) -> bool {
		match (self, other) {
			(Literal::String(e), Literal::String(s)) => e == s,
			(Literal::Int(e), Literal::Int(s)) => e == s,
			(Literal::Float(e), Literal::Float(s)) => e == s,
			(Literal::Float(e), Literal::Int(s)) => *e == *s as f64,
			(Literal::Int(e), Literal::Float(s)) => *e as f64 == *s,
			(Literal::Bool(e), Literal::Bool(s)) => e == s,
			(Literal::Array(e), Literal::Array(s)) => e == s,
			(Literal::Circular(_), Literal::Circular(_)) => false,
			(Literal::None, Literal::None) => false,
			_ => false,
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
	Literal(Literal),
	Ref(String),
	Scope(HashMap<String, Expr>),
	FnCall { fn_name: String, args: Vec<Expr> },
	PrefixOp {op: TokenKind, expr: Box<Expr> },
	InfixOp { op: TokenKind, lhs: Box<Expr>, rhs: Box<Expr> },
    Conditional { condition: Box<Expr>, then_block: Box<Expr>, else_block: Box<Expr> },
	ArrayAccess { lhs: Box<Expr>, index: Box<Expr> },
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

impl From<Vec<Expr>> for Expr {
	fn from(other: Vec<Expr>) -> Expr {
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

	pub fn insert<T: Into<String>>(&mut self, key: T, expr: Expr) -> Option<Expr> {
		let key = key.into();
		self.contents.insert(key, expr)
	}

	#[cfg(test)]
	pub(crate) fn get_ast<T: Into<String>>(&self, key: T) -> Option<&Expr> {
		let key = key.into();
		self.contents.get(&key)
	}
}