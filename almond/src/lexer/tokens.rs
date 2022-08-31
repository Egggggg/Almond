use logos::Logos;
use std::{fmt, ops::{Range, RangeInclusive}};

#[derive(Logos, Clone, Debug, PartialEq)]
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
    #[token("input")]
    Input,
	#[token("output")]
	Output,
    #[token("as")]
    As,
    #[token(",")]
    Comma,
    #[token(".")]
    Access,
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
    #[regex(r"//[^\n]*")]
    Comment,
    #[error]
    Error,
	EOF,

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
	#[token("!=")]
	NotEquals,
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
	#[token("if")]
	If,
	#[token("else")]
	Else,

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
    #[regex(r#""[^"]*""#)]
	#[regex(r#"'[^']*'"#)]
    String,
    #[regex(r"-?\d[\d_]*", |lex| lex.slice().parse())]
    Int(i64),
    #[regex(r"-?(\d[\d_]*)?\.(\d[\d_]*)([eE]?\d[\d_]*)?", |lex| lex.slice().parse())]
    Float(f64),
    #[token("true")]
    True,
    #[token("false")]
    False,
	#[regex(r#"-?\d[\d_]*\.\.-?\d[\d_]*"#, |lex| {
		let nums: Vec<i64> = lex.slice().split("..").map(|i| i.parse::<i64>().unwrap()).collect();
		nums[0]..nums[1]
	})]
	Range(Range<i64>),
	#[regex(r#"-?\d[\d_]*\.\.=-?\d[\d_]*"#, |lex| {
		let nums: Vec<i64> = lex.slice().split("..=").map(|i| i.parse::<i64>().unwrap()).collect();
		nums[0]..=nums[1]
	})]
	IRange(RangeInclusive<i64>),
}

impl fmt::Display for TokenKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", String::from(self))
	}
}

impl From<&TokenKind> for String {
	fn from(other: &TokenKind) -> String {
		let out = match other {
			TokenKind::Ident => "Ident",
			TokenKind::Assign => "Assign",
			TokenKind::End => "End",
			TokenKind::Scope => "Scope",
			TokenKind::Input => "Input",
			TokenKind::Output => "Output",
			TokenKind::As => "As",
			TokenKind::Comma => "Comma",
			TokenKind::Access => "Access",
			TokenKind::Whitespace => "",
			TokenKind::Comment => "Comment",
			TokenKind::Error => "Error",
			TokenKind::EOF => "EOF",
			TokenKind::LCurly => "LCurly",
			TokenKind::RCurly => "RCurly",
			TokenKind::LSquare => "LSquare",
			TokenKind::RSquare => "RSquare",
			TokenKind::LParen => "LParen",
			TokenKind::RParen => "RParen",
			TokenKind::Equals => "Equals",
			TokenKind::NotEquals => "NotEquals",
			TokenKind::Lt => "Lt",
			TokenKind::Gt => "Gt",
			TokenKind::Lte => "Lte",
			TokenKind::Gte => "Gte",
			TokenKind::And => "And",
			TokenKind::Or => "Or",
			TokenKind::Not => "Not",
			TokenKind::If => "If",
			TokenKind::Else => "Else",
			TokenKind::Add => "Add",
			TokenKind::Sub => "Sub",
			TokenKind::Mul => "Mul",
			TokenKind::Div => "Div",
			TokenKind::Mod => "Mod",
			TokenKind::Exp => "Exp",
			TokenKind::Range(_) => "Range",
			TokenKind::IRange(_) => "IRange",
			TokenKind::String => "String",
			TokenKind::Int(_) => "Int",
			TokenKind::Float(_) => "Float",
			TokenKind::True => "True",
			TokenKind::False => "False",
		};

		out.to_owned()
	}
}
