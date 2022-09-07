use core::fmt;

use logos::Span;

use crate::lexer::tokens::TokenKind;

pub trait AlmondError: Clone + fmt::Display {}

#[derive(Debug, Clone)]
pub struct NameConflictError {
	name: String,
	scope: String
}

impl NameConflictError {
	pub fn new<T, E>(name: T, scope: E) -> NameConflictError 
	where
	String: From<T>,
	String: From<E> {
		let name = name.into();
		let scope = scope.into();
		
		NameConflictError { name, scope }
	}
}

impl fmt::Display for NameConflictError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "NameConflictError: Member with name `{}` already exists in scope `{}`", self.name, self.scope)
	}
}

impl AlmondError for NameConflictError {}

#[derive(Debug, Clone)]
pub struct SyntaxError {
	found: TokenKind,
	at: Span
}

impl SyntaxError {
	pub fn new(found: TokenKind, at: Span) -> SyntaxError {
		SyntaxError { found, at }
	}
}

impl fmt::Display for SyntaxError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "SyntaxError: Unexpected token `{}` at char `{}`", self.found, self.at.next().unwrap())
	}
}

impl AlmondError for SyntaxError {}