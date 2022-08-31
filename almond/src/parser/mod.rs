use logos::Logos;

use crate::lexer::tokens::TokenKind;

use self::ast::Store;

pub mod ast;
pub mod expressions;

#[derive(Debug)]
pub struct Parser<'a> {
	lexer: logos::Lexer<'a, TokenKind>,
	current: Option<TokenKind>,
	slice: &'a str,
	next: Option<TokenKind>
}

impl<'a> Parser<'a> {
	pub fn new(input: &'a str) -> Parser<'a> {
		let mut lexer = TokenKind::lexer(input);
		let current = None;
		let slice = lexer.slice();
		let next = lexer.next();

		Parser { lexer, current, slice, next }
	}

	pub fn parse(&mut self) -> Store {
		let mut output = Store::new();

		self.parse_input(&mut output);

		output
	}

	pub(crate) fn peek(&self) -> Option<TokenKind> {
		self.next.clone()
	}

	pub(crate) fn next(&mut self) -> Option<TokenKind> {
		self.current = self.next.clone();
		self.slice = self.lexer.slice();
		self.next = self.lexer.next();

		self.current.clone()
	}

	pub(crate) fn slice(&self) -> &'a str {
		self.slice
	}

	pub(crate) fn consume<T: AsRef<TokenKind>>(&mut self, expected: T) {
		let expected = expected.as_ref().to_owned();

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
		);
	}
}

pub fn eval<'a>(input: &'a str) -> Store {
	let mut parser = Parser::new(input);
	parser.parse()
}