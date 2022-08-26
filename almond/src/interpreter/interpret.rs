use std::collections::HashMap;

use crate::parser::ast::{Expr, Literal};

#[derive(Debug)]
pub struct Store {
	contents: HashMap<String, Expr>,
}

impl Store {
	pub fn new() -> Store {
		let contents: HashMap<String, Expr> = HashMap::new();
		
		Store { contents }
	}

	pub fn get<T>(&self, ident: T, history: Option<Vec<String>>) -> Literal
	where T: Into<String> {
		let ident = ident.into();
		let mut history = history.unwrap_or_default();

		if history.contains(&ident) {
			return Literal::Circular(ident.to_owned());
		}

		let found = self.contents.get(&ident);
		
		history.push(ident);
		
		match found {
			Some(e) => e.eval(self, history),
			None => Literal::None,
		}
	}

	pub fn insert(&mut self, key: String, expr: Expr) -> Option<Expr> {
		self.contents.insert(key, expr)
	}
}

impl Expr {
	pub(crate) fn eval(&self, store: &Store, history: Vec<String>) -> Literal {
		match self {
			Expr::Literal(e) => *e,
			_ => todo!(),
		}
	}
}