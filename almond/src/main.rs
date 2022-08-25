mod syntax;
mod parser;

use logos::Logos;
use syntax::TokenKind;
use parser::parse;

fn main() {
	let input = r#"nice = 23; cool = 46; epic = nice + cool;"#;
    let mut parser = parser::Parser(input);
	
	println!("{:#?}", parser)

	/*
	let out = parse(&mut lex);

	let nice = out.get("nice", None);
	let cool = out.get("cool", None);
	let epic = out.get("epic", None);
	
	println!("input: {}", input);
	println!("nice: {}\ncool: {}\nepic: {}", nice, cool, epic);
	println!("nice + cool: {}", nice + cool);
	*/
}
