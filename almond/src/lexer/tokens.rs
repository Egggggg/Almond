use logos::Logos;
use std::fmt;

#[derive(Logos, Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    // ===== general =====
    #[regex(r"[a-zA-Z][\w_]*")]
    Ident,
    #[token("=")]
    Assign,
	#[token(":=")]
	Walrus,
    #[token(";")]
    End,
    #[regex(r"\$[a-zA-Z][\w_]*")]
    Scope,
    #[token("import")]
    Import,
	#[token("export")]
	Export,
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
	#[token("..")]
	Range,
	#[token("..=")]
	IRange,

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
			TokenKind::Walrus => "Walrus",
			TokenKind::End => "End",
			TokenKind::Scope => "Scope",
			TokenKind::Import => "Export",
			TokenKind::Export => "Import",
			TokenKind::As => "As",
			TokenKind::Comma => "Comma",
			TokenKind::Access => "Access",
			TokenKind::Whitespace => "",
			TokenKind::Comment => "Comment",
			TokenKind::Error => "Error",
			TokenKind::EOF => "EOF",
			TokenKind::Range => "Range",
			TokenKind::IRange => "IRange",
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
			TokenKind::String => "String",
			TokenKind::Int(_) => "Int",
			TokenKind::Float(_) => "Float",
			TokenKind::True => "True",
			TokenKind::False => "False",
		};

		out.to_owned()
	}
}

impl AsRef<TokenKind> for TokenKind {
	fn as_ref(&self) -> &TokenKind {
		&self
	}
}
