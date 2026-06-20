#[derive(Debug)]
pub struct Program {
    pub(crate) stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Let { name: String, expr: Expr },
    Print { expr: Expr },
}

#[derive(Debug)]
pub enum Pattern {
    Number(i64),
    Boolean(bool),
    Wildcard,
}

#[derive(Debug)]
pub struct MatchArm {
    pub(crate) pattern: Pattern,
    pub(crate) value: Box<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    String(String),
    Boolean(bool),
    Ident(String),
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Match {
        subject: Box<Expr>,
        arms: Vec<MatchArm>,
    },
}

#[derive(Debug, Clone)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    EqualEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    BangEqual,
    EqualGreater,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Equal,
    Semicolon,
    Comma,
    Number(i64),
    String(String),
    Ident(String),
    Print,
    Let,
    Match,
    Wildcard,
    Fn,
    True,
    False,
}
