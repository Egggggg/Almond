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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident {
	pub contents: String,
}

impl<'a> From<&'a str> for Ident {
	fn from(other: &'a str) -> Ident {
		Ident { contents: other.to_owned() }
	}
}

impl From<String> for Ident {
	fn from(contents: String) -> Ident {
		Ident { contents }
	}
}

impl From<&Ident> for String {
	fn from(other: &Ident) -> String {
		(&other.contents).to_owned()
	}
}

impl AsRef<String> for Ident {
	fn as_ref(&self) -> &String {
		&self.contents
	}
}

#[derive(Debug, Clone)]
pub enum Raw {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
	Circular(String),
	None,
}

impl fmt::Display for Raw {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Raw::String(e) => write!(f, "{}", e),
			Raw::Int(e) => write!(f, "{}", e),
			Raw::Float(e) => write!(f, "{}", e),
			Raw::Bool(e) => write!(f, "{}", e),
			Raw::Circular(e) => write!(f, "[CIRCULAR DEPENDENCY ({})]", e),
			Raw::None => write!(f, "None")
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
					Raw::Circular(e) => format!("[CIRCULAR DEPENDENCY ({})]", e).to_owned(),
					Raw::None => "None".to_owned(),
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
					Raw::Circular(_) => 0,
					Raw::None => 0,
				};

				Raw::Int(e + other)
			},
			Raw::Float(e) => {
				let other = match other {
					Raw::String(s) => s.parse::<f64>().unwrap_or(0.0),
					Raw::Int(s) => s as f64,
					Raw::Float(s) => s,
					Raw::Bool(s) => s as i8 as f64,
					Raw::Circular(_) => 0.0,
					Raw::None => 0.0,
				};

				Raw::Float(e + other)
			},
			Raw::Bool(_) => self,
			Raw::Circular(_) => self,
			Raw::None => self,
		}
	}
}

#[derive(Debug)]
pub enum Expr {
    Add(AddExpr),
    Sub(Statement, Option<Statement>),
    Mul(Statement, Option<Statement>),
    Div(Statement, Option<Statement>),
    Mod(Statement, Option<Statement>),
    Exp(Statement, Option<Statement>),
    Conditional(Condition, Option<Statement>),
	Output(Output),
	Ref(Ident),
	IRef(Ident),
	IAssign(Ident, Box<Expr>),
}

impl<'a> Expr {
	pub fn output(out: Statement) -> Expr {
		let output = Output::from(out);

		return Expr::Output(output)
	}

	pub fn operand(&self, operand: Statement) {
		match self {
			Expr::Add(e) => e.operand(operand),
			_ => todo!("Operand not yet implemented"),
		}
	}

	pub fn eval(&self, data: &Store, history: Vec<String>) -> Raw {
		match self {
			Expr::Output(e) => e.eval(data, history),
			Expr::Ref(e) => data.get(e, Some(history)),
			_ => Raw::Int(-1),
		}
	}
}

impl From<String> for Expr {
	fn from(other: String) -> Expr {
		Expr::Ref(other.into())
	}
}

pub trait BinaryExpr {
	fn operand(&mut self, operand: Statement);
}

#[derive(Debug)]
pub struct AddExpr {
	lhs: Statement,
	rhs: Option<Statement>,
}

impl AddExpr {
	pub fn new(lhs: Statement, rhs: Option<Statement>) -> AddExpr {
		AddExpr { lhs, rhs }
	}
}

impl BinaryExpr for AddExpr {
	fn operand(&mut self, operand: Statement) {
		self.rhs = Some(operand);
	}
}

#[derive(Debug)]
pub struct Store {
	contents: HashMap<String, ExprChain>,
}

impl Store {
	pub fn new() -> Store {
		let contents: HashMap<String, ExprChain> = HashMap::new();

		Store { contents }
	}

	pub fn get<T>(&self, ident: T, history: Option<Vec<String>>) -> Raw 
	where T: Into<String> {
		let ident = ident.into();
		let mut history = history.unwrap_or_default();

		if history.contains(&ident) {
			return Raw::Circular(ident.to_owned());
		}

		let found = self.contents.get(&ident);
		
		history.push(ident);
		
		match found {
			Some(e) => e.eval(&self, history),
			None => Raw::None,
		}
	}

	pub fn insert(&mut self, key: String, chain: ExprChain) -> Option<ExprChain> {
		self.contents.insert(key, chain)
	}
}

#[derive(Debug)]
enum Output {
    Expr(Box<Expr>),
    Raw(Raw),
}

impl<'a> Output {
	pub fn eval(&self, data: &Store, history: Vec<String>) -> Raw {
		match self {
			Output::Expr(e) => e.eval(data, history),
			Output::Raw(e) => e.to_owned(),
		}
	}
}

impl From<Statement> for Output {
	fn from(other: Statement) -> Output {
		match other {
			Statement::Ident(e) => Output::Expr(Box::new(Expr::Ref(e))),
			Statement::Raw(e) => Output::Raw(e),
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

impl From<Ident> for Statement {
	fn from(other: Ident) -> Statement {
		Statement::Ident(other)
	}
}

impl From<Raw> for Statement {
	fn from(other: Raw) -> Statement {
		Statement::Raw(other)
	}
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
	chain: Vec<Expr>,
}

impl<'a> ExprChain {
	pub fn new(data: Vec<Expr>) -> ExprChain {
		ExprChain { chain: data }
	}

	pub fn eval(&self, data: &Store, history: Vec<String>) -> Raw {
		for expr in &self.chain {
			match expr {
				Expr::Output(e) => return e.eval(data, history),
				_ => todo!("Not yet implemented"),
			}
		}

		panic!("No output was found in the chain")
	}
}