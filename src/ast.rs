#[derive(Debug)]
pub struct Program {
    pub(crate) stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Let { name: String, expr: Expr },
    Expr { expr: Expr },
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Number(i64),
    Boolean(bool),
    Wildcard,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub(crate) pattern: Pattern,
    pub(crate) value: Box<Expr>,
}

#[derive(Debug, Clone)]
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
    Fn {
        params: Vec<String>,
        body: Box<Expr>,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
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
    Let,
    Match,
    Wildcard,
    Fn,
    True,
    False,
}
