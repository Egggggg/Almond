use logos::Logos;

use crate::lexer::tokens::TokenKind;

pub(crate) mod ast;
pub(crate) mod expressions;

#[derive(Debug)]
pub struct Parser<'a> {
	lexer: logos::Lexer<'a, TokenKind>,
	current: Option<TokenKind>,
}

impl<'a> Parser<'a> {
	pub fn new(input: &'a str) -> Parser<'a> {
		let lexer = TokenKind::lexer(input);
		let current = lexer.next();

		Parser { lexer, current }
	}

	pub(crate) fn current(&self) -> Option<TokenKind> {
		self.current
	}

	pub(crate) fn next(&mut self) -> Option<TokenKind> {
		self.current = self.lexer.next();

		self.current()
	}

	pub(crate) fn slice(&self) -> &'a str {
		self.lexer.slice()
	}

	pub(crate) fn consume(&mut self, expected: TokenKind) {
		let token = self.current().expect(&format!(
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