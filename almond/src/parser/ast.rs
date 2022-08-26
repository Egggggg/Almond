use crate::{lexer::tokens::TokenKind, interpreter::interpret::Store};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Literal {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
	Circular(String),
	None,
}

#[derive(Debug)]
pub(crate) enum Expr {
	Literal(Literal),
	Ref(String),
	FnCall { fn_name: String, args: Vec<Expr> },
	PrefixOp {op: TokenKind, expr: Box<Expr> },
	InfixOp { op: TokenKind, lhs: Box<Expr>, rhs: Box<Expr> },
    Conditional { condition: Box<Expr>, block: Box<Expr>, else_block: Box<Expr> },
}
