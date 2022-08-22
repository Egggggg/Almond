mod syntax;
mod parser;

use logos::Logos;
use syntax::TokenKind;
use parser::parse;

fn main() {
    let mut lex = TokenKind::lexer(r#"nice = "true"; hell = false;"#);
	let out = parse(&mut lex);

	let nice = out.get("nice").unwrap().eval(&out);
	let hell = out.get("hell").unwrap().eval(&out);

	println!("nice: {}, hell: {}", nice, hell);
	println!("nice + hell: {}", nice + hell);
}
