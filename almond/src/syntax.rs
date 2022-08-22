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

type Ident<'a> = &'a str;

#[derive(Debug)]
pub enum VarType<'a> {
    String(&'a str),
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug)]
pub enum Expr<'a> {
    Add(Statement<'a>, Statement<'a>),
    Sub(Statement<'a>, Statement<'a>),
    Mul(Statement<'a>, Statement<'a>),
    Div(Statement<'a>, Statement<'a>),
    Mod(Statement<'a>, Statement<'a>),
    Exp(Statement<'a>, Statement<'a>),
    Conditional(Condition<'a>, VarType<'a>),
	Assign(Assign<'a>),
	Ref(Ref<'a>),
	IRef(IRef<'a>),
	IAssign(Intmdt<'a>),
}

impl<'a> Expr<'a> {
	pub fn assign_literal(ident: Ident<'a>, to: VarType<'a>) -> Expr<'a> {
		let statement = Statement::VarType(to);
		let assignment = Assignment::Statement(statement);
		let assign = Assign {
			lhs: ident,
			rhs: assignment,
		};

		return Expr::Assign(assign);
	}
}

#[derive(Debug)]
struct Ref<'a> {
	pub to: Ident<'a>,
}

#[derive(Debug)]
struct IRef<'a> {
	pub to: Ident<'a>,
}

#[derive(Debug)]
pub struct Intmdt<'a> {
	pub name: Ident<'a>,
	pub expr: Box<Expr<'a>>,
}

#[derive(Debug)]
enum Assignment<'a> {
    Expr(Box<Expr<'a>>),
    Statement(Statement<'a>),
}

#[derive(Debug)]
struct Assign<'a> {
    lhs: Ident<'a>,
    rhs: Assignment<'a>,
}

#[derive(Debug)]
enum Condition<'a> {
	If(IfCondition<'a>),
	Else
}

#[derive(Debug)]
struct IfCondition<'a> {
    lhs: Statement<'a>,
    comparison: Comparison,
    rhs: Statement<'a>,
}

#[derive(Debug)]
enum Statement<'a> {
    VarType(VarType<'a>),
    Ident(Ident<'a>),
}

#[derive(Debug)]
enum Comparison {
    Equals,
    Lt,
    Gt,
    Lte,
    Gte,
}