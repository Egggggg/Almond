mod parser;

fn main() {
	let input = r#"nice = 23; cool = 46; epic = nice + cool;"#;
    let mut parser = parser::Parser::new(input);
	
	println!("{:#?}", parser)
}
