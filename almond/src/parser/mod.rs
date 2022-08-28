use logos::Logos;

use crate::lexer::tokens::TokenKind;

use self::ast::Store;

pub mod ast;
pub mod parsing;

#[derive(Debug)]
pub struct Parser<'a> {
	lexer: logos::Lexer<'a, TokenKind>,
	current: Option<TokenKind>,
	prev: Option<TokenKind>,
	prev_slice: Option<&'a str>,
}

impl<'a> Parser<'a> {
	pub fn new(input: &'a str) -> Parser<'a> {
		let lexer = TokenKind::lexer(input);
		let current = None;
		let prev = None;
		let prev_slice = None;

		Parser { lexer, current, prev, prev_slice }
	}

	pub fn parse(&mut self/*, input: HashMap<String, Literal>*/) -> Store {
		let mut output = Store::new();

		self.parse_input(&mut output);

		output
	}

	pub(crate) fn current(&self) -> Option<TokenKind> {
		self.current.clone()
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
		);

		self.next();
	}
}