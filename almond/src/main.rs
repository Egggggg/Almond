use almond::parser;

fn main() {
	let input = r#"nice = 12 + 39 * 43289 % 392 / 9 ** (12 + 4); cool = nice * (3 + nice);"#;
    let mut parser = parser::Parser::new(input);
	
	println!("{:#?}", parser);

	let store = parser.parse();

	println!("{:#?}", store);
}
