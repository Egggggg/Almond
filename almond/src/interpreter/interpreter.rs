use crate::parser::ast::{Expr, Literal, Store};

impl Store {
	pub fn get<T: Into<String>>(&self, ident: T, history: Option<Vec<String>>) -> Literal {
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
}

impl Expr {
	pub(crate) fn eval(&self, store: &Store, history: Vec<String>) -> Literal {
		match self {
			Expr::Literal(e) => e.clone(),
			_ => todo!(),
		}
	}
}