use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // ===== general =====
    #[regex(r"[a-zA-Z][\w_]*")]
    Ident,
    #[token("=")]
    Assign,
    #[token(";")]
    End,
    #[token("$")]
    Scope,
    #[token("$#")]
    IScope,
    #[token("import")]
    Import,
    #[token("as")]
    As,
    #[token(",")]
    Comma,
    #[token(".")]
    Access,
    #[error]
    Error,
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Skipped,

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
    #[regex(r"\d[\d_]*")]
    Int,
    #[regex(r"(\d[\d_]*)?\.(\d[\d_]*)([eE]\d[\d_]*)?")]
    Float,
    #[token("true")]
    True,
    #[token("false")]
    False,
}
