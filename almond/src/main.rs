mod syntax;
mod parser;

use logos::Logos;
use syntax::TokenKind;
use parser::parse;

fn main() {
    let mut lex = TokenKind::lexer("nice = 4; hell = 69;");
	let out = parse(&mut lex);

	println!("{:#?}", out);
}
