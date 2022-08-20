mod token;

use anyhow::Error;
use logos::{Lexer, Logos};
use token::Token;

type Ident = String;

#[derive(Clone)]
enum VarType {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Clone)]
enum Expr {
    Add(VarType, VarType),
    Sub(VarType, VarType),
    Mul(VarType, VarType),
    Div(VarType, VarType),
    Mod(VarType, VarType),
    Exp(Statement, VarType),
    Conditional(Condition, VarType, VarType),
}

#[derive(Clone)]
enum Assignment {
    Expr,
    Statement,
}

#[derive(Clone)]
struct Assign {
    lhs: Ident,
    rhs: Assignment,
}

#[derive(Clone)]
struct Condition {
    lhs: Statement,
    comparison: Comparison,
    rhs: Statement,
}

#[derive(Clone)]
enum Statement {
    VarType(VarType),
    Ident(Ident),
}

#[derive(Clone)]
enum Comparison {
    Equals,
    Lt,
    Gt,
    Lte,
    Gte,
}

fn error(message: &str) -> Error {
	return Error { inner: message };
}

fn parse(lex: &mut Lexer<Token>) -> Result<Vec<Assign>, Error> {
    let mut out: Vec<Assign> = Vec::new();
    let mut currentIdent: Option<Ident>;
    let mut currentAssign: Assignment;
    let mut assigning = false;

    loop {
        if let Some(e) = lex.next() {
            match e {
                Token::Ident => currentIdent = Some(lex.slice().to_owned()),
                Token::Assign => assigning = true,
                Token::End => {
                    assigning = false;
                    
					let assignment = Assignment {

					}

					let assign = Assign {
						lhs: currentIdent?,
						rhs: assignment,
					};
                }
            }
        } else {
			return error("Unexpected EOF");
        }
    }
}

fn main() {
    let mut lex = Token::lexer("a = 12; b = a + 2; c = d + 9; d = b * 2;");

    let data = parse(&mut lex).unwrap();
}
