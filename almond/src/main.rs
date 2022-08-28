use almond::parser;

fn main() {
	let input = r#"nice = !false cool = -923864"#;
    let mut parser = parser::Parser::new(input);
	
	println!("{:#?}", parser);

	let store = parser.parse();

	println!("{:#?}", store);
}
