use crate::ast::{Expr, MatchArm, Pattern, Program, Stmt, Token};
use crate::log;
use crate::scan::scan;
use std::ops::Index;

fn parse_primary(tokens: &Vec<Token>, i: &mut usize) -> Expr {
    let t = tokens.index(*i);
    *i += 1;
    match t {
        Token::Number(i) => Expr::Number(*i),
        Token::String(s) => Expr::String(s.clone()),
        Token::True => Expr::Boolean(true),
        Token::False => Expr::Boolean(false),
        Token::Ident(name) => Expr::Ident(name.clone()),
        Token::LParen => {
            let e = parse_expr(tokens, i);
            let t_n = tokens.index(*i + 1);
            if matches!(t_n, Token::RParen) {
                *i += 1
            } else {
                log!("Expected right paren, get:{t_n:?}")
            }
            e
        }
        Token::Match => {
            let subject = parse_expr(tokens, i);
            let mut t_n = tokens.index(*i);
            if matches!(t_n, Token::LBrace) {
                *i += 1
            } else {
                log!("Expected left brace, get:{t_n:?}")
            };
            let mut arms = Vec::new();
            t_n = tokens.index(*i);
            while matches!(t_n, Token::Number(_))
                || matches!(t_n, Token::True)
                || matches!(t_n, Token::False)
                || matches!(t_n, Token::Wildcard)
            {
                let pattern = match t_n {
                    Token::Number(i) => Pattern::Number(*i),
                    Token::True => Pattern::Boolean(true),
                    Token::False => Pattern::Boolean(false),
                    Token::Wildcard => Pattern::Wildcard,
                    _ => {
                        log!("Invalid pattern:{t_n:?}");
                        Pattern::Boolean(false)
                    }
                };
                *i += 1;
                t_n = tokens.index(*i);
                if matches!(t_n, Token::EqualGreater) {
                    *i += 1
                } else {
                    log!("Expected equal greater, get:{t_n:?}")
                };
                let value = parse_expr(tokens, i);
                arms.push(MatchArm {
                    pattern,
                    value: Box::new(value),
                });
                t_n = tokens.index(*i);
                if matches!(t_n, Token::Comma) {
                    *i += 1
                } else {
                    log!("Expected Comma, get:{t_n:?}")
                };
                t_n = tokens.index(*i)
            }
            t_n = tokens.index(*i);
            if matches!(t_n, Token::RBrace) {
                *i += 1
            } else {
                log!("Expected right brach, get:{t_n:?}")
            };
            Expr::Match {
                subject: Box::new(subject),
                arms: arms,
            }
        }
        _ => {
            log!("invalid token:{t:?}");
            Expr::Number(0)
        }
    }
}

fn parse_mul_div(tokens: &Vec<Token>, i: &mut usize) -> Expr {
    let mut result = parse_primary(tokens, i);
    let mut t = tokens.index(*i);
    while matches!(t, Token::Star) || matches!(t, Token::Slash) {
        *i += 1;
        let r = parse_primary(tokens, i);
        result = Expr::Binary {
            left: Box::new(result),
            op: t.clone(),
            right: Box::new(r),
        };
        t = tokens.index(*i);
    }
    result
}

fn parse_add_sub(tokens: &Vec<Token>, i: &mut usize) -> Expr {
    let mut result = parse_mul_div(tokens, i);
    let mut t = tokens.index(*i);
    while matches!(t, Token::Plus) || matches!(t, Token::Minus) {
        *i += 1;
        let r = parse_mul_div(tokens, i);
        result = Expr::Binary {
            left: Box::new(result),
            op: t.clone(),
            right: Box::new(r),
        };
        t = tokens.index(*i);
    }
    result
}

fn parse_comparison(tokens: &Vec<Token>, i: &mut usize) -> Expr {
    let mut result = parse_add_sub(tokens, i);
    let mut t = tokens.index(*i);
    while matches!(t, Token::Less)
        || matches!(t, Token::Greater)
        || matches!(t, Token::EqualEqual)
        || matches!(t, Token::BangEqual)
        || matches!(t, Token::GreaterEqual)
        || matches!(t, Token::LessEqual)
    {
        *i += 1;
        let r = parse_add_sub(tokens, i);
        result = Expr::Binary {
            left: Box::new(result),
            op: t.clone(),
            right: Box::new(r),
        };
        t = tokens.index(*i);
    }
    result
}

fn parse_expr(tokens: &Vec<Token>, i: &mut usize) -> Expr {
    let e = parse_comparison(tokens, i);
    let t = tokens.index(*i);
    if !matches!(t, Token::Semicolon) {
        log!("Expected semicolon, get:{t:?}")
    }
    e
}

pub fn parse(input: &str) -> Program {
    let tokens = scan(input);
    let mut i = 0;
    let mut result = Program { stmts: Vec::new() };
    while i < tokens.len() {
        let t = tokens.index(i);
        match t {
            Token::Print => {
                if i + 2 > tokens.len() {
                    log!("Length is not enough for print")
                }
                i += 1;
                let expr = parse_expr(&tokens, &mut i);
                let t_next_next = tokens.index(i);
                if !matches!(t_next_next, &Token::Semicolon) {
                    log!("Missing semicolon")
                }
                result.stmts.push(Stmt::Print { expr });
                i += 2
            }
            Token::Let => {
                if i + 4 > tokens.len() {
                    log!("Length is not enough for let")
                }
                let t_next = tokens.index(i + 1);
                let indent_name = if let Token::Ident(name) = t_next {
                    name.clone()
                } else {
                    "".to_string()
                };
                let t_next_next = tokens.index(i + 2);
                if !matches!(t_next_next, &Token::Equal) {
                    log!("Missing equal")
                }
                i += 3;
                let expr = parse_expr(&tokens, &mut i);
                let t_next_next_next_next = tokens.index(i);
                if !matches!(t_next_next_next_next, &Token::Semicolon) {
                    log!("Missing semicolon")
                }
                result.stmts.push(Stmt::Let {
                    name: indent_name,
                    expr,
                });
                i += 1
            }
            _ => {
                log!("invalid token:{t:?}");
                i += 1
            }
        }
    }
    result
}
