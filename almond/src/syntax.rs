use logos::Logos;

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
    #[token("'")]
    SQuote,
    #[token(r#"""#)]
    DQuote,

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
    #[regex(r#""[^(\n)(\r\n)]*""#)]
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

enum VarType {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

pub enum Expr {
    Add(VarType, VarType),
    Sub(VarType, VarType),
    Mul(VarType, VarType),
    Div(VarType, VarType),
    Mod(VarType, VarType),
    Exp(Statement, VarType),
    Conditional(Condition, VarType, VarType),
	Ref(Ref),
	TRef(TRef),
	Tmp(Tmp),
}

struct Ref {
	pub to: Ident,
}

struct TRef {
	pub to: Ident,
}

pub struct Tmp {
	pub name: Ident,
	pub expr: Box<Expr>,
}

enum Assignment {
    Expr,
    Statement,
}

struct Assign {
    lhs: Ident,
    rhs: Assignment,
}

struct Condition {
    lhs: Statement,
    comparison: Comparison,
    rhs: Statement,
}

enum Statement {
    VarType(VarType),
    Ident(Ident),
}

enum Comparison {
    Equals,
    Lt,
    Gt,
    Lte,
    Gte,
}