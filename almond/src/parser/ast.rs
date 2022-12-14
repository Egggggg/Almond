use std::collections::HashMap;

use crate::lexer::tokens::TokenKind;

use super::errors::NameConflictError;

#[derive(Debug)]
pub enum InputType {
	String,
	Int,
	Float,
	Bool,
	Array,
	Any,
}

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
	Import,
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
pub struct Input {
	pub static_type: InputType,
	pub default: Option<Expr>,
	pub current: Option<Literal>,
}

#[derive(Debug)]
pub struct Store {
	pub contents: HashMap<String, Expr>,
	pub inputs: HashMap<String, Input>,
	pub outputs: Vec<String>,
}

impl Store {
	pub fn new() -> Store {
		let contents: HashMap<String, Expr> = HashMap::new();
		let inputs: HashMap<String, Input> = HashMap::new();
		let outputs: Vec<String> = Vec::new();
		
		
		Store { contents, inputs, outputs }
	}

	pub fn insert<T: Into<String>>(&mut self, key: T, expr: Expr) -> Result<Option<Expr>, NameConflictError> {
		let key = key.into();

		if self.contents.contains_key(&key) {
			return Err(NameConflictError::new(key, "global"));
		}

		Ok(self.contents.insert(key, expr))
	}

	pub fn insert_input<T: Into<String>>(&mut self, key: T, input: Input) -> Result<Option<Input>, NameConflictError> {
		let key = key.into();

		self.insert(key, Expr::Import)?;

		Ok(self.inputs.insert(key, input))
	}

	#[cfg(test)]
	pub(crate) fn get_ast<T: Into<String>>(&self, key: T) -> Option<&Expr> {
		let key = key.into();
		self.contents.get(&key)
	}
}