use logos::Logos;
use std::iter::Peekable;

use crate::lexer::tokens::{TokenKind, T};

pub(crate) mod ast;
pub(crate) mod expressions;

pub struct Parser<'a> {
	lexer: Peekable<logos::Lexer<'a, TokenKind>>
}

impl<'a> Parser<'a> {
	pub fn new(input: &'a str) -> Parser<'a> {
		Parser { lexer: TokenKind::lexer(input).peekable() }
	}

	pub(crate) fn peek(&mut self) -> &TokenKind {
		self.lexer.peek().unwrap_or(&T![EOF])
	}

	pub(crate) fn at(&mut self, kind: &TokenKind) -> bool {
		self.peek() == kind
	}

	pub(crate) fn next(&mut self) -> Option<TokenKind> {
		self.lexer.next()
	}

	pub(crate) fn consume(&mut self, expected: TokenKind) {
		let token = self.next().expect(&format!(
			"Expected to consume `{}`, but there was no next token",
			expected
		));
		
		assert_eq!(
			token,
			expected,
			"Expected to consume `{}`, but found `{}`",
			expected,
			token
		)
	}
}