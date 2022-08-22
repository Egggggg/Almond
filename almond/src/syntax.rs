use logos::Logos;
use std::{collections::HashMap, fmt, ops::Add};

#[derive(Logos, Debug, PartialEq)]
pub enum TokenKind {
    // ===== general =====
    #[regex(r"[a-zA-Z][\w_]*")]
    Ident,
    #[token("=")]
    Assign,
    #[token(";")]
    End,
    #[regex(r"\$[a-zA-Z][\w_]*")]
    Scope,
    #[regex(r"\$#[a-zA-Z][\w_]*")]
    IScope,
    #[token("import")]
    Import,
    #[token("as")]
    As,
    #[token(",")]
    Comma,
    #[token(".")]
    Access,
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Skipped,
    #[regex(r"//[^\n]*")]
    Comment,
    #[error]
    Error,

    // ===== containers =====
    #[token("{")]
    LCurly,
    #[token("}")]
    RCurly,
    #[token("[")]
    LSquare,
    #[token("]")]
    RSquare,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    // ===== logic =====
    #[token("==")]
    Equals,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("<=")]
    Lte,
    #[token(">=")]
    Gte,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Not,

    // ===== math =====
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("%")]
    Mod,
    #[token("**")]
    Exp,

    // ===== literal =====
    #[regex(r#""[^"\n]*""#)]
    String,
    #[regex(r"\d[\d_]*", |lex| lex.slice().parse())]
    Int(i64),
    #[regex(r"(\d[\d_]*)?\.(\d[\d_]*)([eE]\d[\d_]*)?", |lex| lex.slice().parse())]
    Float(f64),
    #[token("true")]
    True,
    #[token("false")]
    False,
}

type Ident = String;

#[derive(Debug, Clone)]
pub enum Raw {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl fmt::Display for Raw {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Raw::String(e) => write!(f, "{}", e),
			Raw::Int(e) => write!(f, "{}", e),
			Raw::Float(e) => write!(f, "{}", e),
			Raw::Bool(e) => write!(f, "{}", e),
		}
	}
}

impl Add for Raw {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		match self {
			Raw::String(e) => {
				let other = match other {
					Raw::String(s) => s,
					Raw::Int(s) => s.to_string(),
					Raw::Float(s) => s.to_string(),
					Raw::Bool(s) => s.to_string(),
				};

				let out = e.to_owned() + &other;

				Raw::String(out)
			},
			Raw::Int(e) => {
				let other = match other {
					Raw::String(s) => s.parse::<i64>().unwrap_or(0),
					Raw::Int(s) => s,
					Raw::Float(s) => s as i64,
					Raw::Bool(s) => s as i64,
				};

				Raw::Int(e + other)
			},
			Raw::Float(e) => {
				let other = match other {
					Raw::String(s) => s.parse::<f64>().unwrap_or(0.0),
					Raw::Int(s) => s as f64,
					Raw::Float(s) => s,
					Raw::Bool(s) => s as i8 as f64,
				};

				Raw::Float(e + other)
			},
			Raw::Bool(e) => {
				self
			}
		}
	}
}

#[derive(Debug)]
pub enum Expr {
    Add(Statement, Statement),
    Sub(Statement, Statement),
    Mul(Statement, Statement),
    Div(Statement, Statement),
    Mod(Statement, Statement),
    Exp(Statement, Statement),
    Conditional(Condition, Statement),
	Output(Output),
	Ref(Ref),
	IRef(IRef),
	Assign(Intermediate),
}

impl<'a> Expr {
	pub fn output_literal(ident: Ident, to: Raw) -> Expr {
		let output = Output::Raw(to);

		return Expr::Output(output);
	}

	pub fn eval(&self, data: &HashMap<&'a str, ExprChain>) -> Raw {
		match self {
			Expr::Output(e) => e.eval(data),
			_ => Raw::Int(-1),
		}
	}
}

#[derive(Debug)]
struct Ref {
	pub to: Ident,
}

#[derive(Debug)]
struct IRef {
	pub to: Ident,
}

#[derive(Debug)]
pub struct Intermediate {
	pub name: Ident,
	pub expr: Box<Expr>,
}

#[derive(Debug)]
enum Output {
    Expr(Box<Expr>),
    Raw(Raw),
}

impl<'a> Output {
	pub fn eval(&self, data: &HashMap<&'a str, ExprChain>) -> Raw {
		match self {
			Output::Expr(e) => e.eval(data),
			Output::Raw(e) => e.to_owned(),
		}
	}
}

#[derive(Debug)]
enum Condition {
	If(IfCondition),
	Else
}

#[derive(Debug)]
struct IfCondition {
    lhs: Statement,
    comparison: Comparison,
    rhs: Statement,
}

#[derive(Debug)]
pub enum Statement {
    Ident(Ident),
	Raw(Raw),
}

#[derive(Debug)]
enum Comparison {
    Equals,
    Lt,
    Gt,
    Lte,
    Gte,
}

#[derive(Debug)]
pub struct ExprChain {
	pub chain: Vec<Expr>,
}

impl<'a> ExprChain {
	pub fn eval(&self, data: &HashMap<&'a str, ExprChain>) -> Raw {
		for expr in &self.chain {
			match expr {
				Expr::Output(e) => return e.eval(data),
				_ => todo!("Not yet implemented"),
			}
		}

		panic!("No output was found in the chain")
	}
}