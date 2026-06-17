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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_output() {
        let program = parse("let x = 1 + 2; print x;");
        assert!(matches!(&program.stmts[0], Stmt::Let {
                name,
                expr: Expr::Binary {
                    left,
                    op: Token::Plus,
                    right,
                },
            } if name == "x" && matches!(left.as_ref(), Expr::Number(1)) && matches!(right.as_ref(), Expr::Number(2))
        ));
        assert!(matches!(&program.stmts[1],Stmt::Print { expr:Expr::Ident(x) } if x == "x"));
    }

    #[test]
    fn test_parse_simple_let() {
        let program = parse("let x = 1;");
        assert!(matches!(&program.stmts[0], Stmt::Let {
                name,
                expr: Expr::Number(1),
            } if name == "x"
        ));
    }

    #[test]
    fn test_parse_simple_print() {
        let program = parse("print x;");
        assert!(matches!(&program.stmts[0], Stmt::Print {
                expr: Expr::Ident(name),
            } if name == "x"
        ));
    }

    #[test]
    fn test_parse_paren() {
        let program = parse("print 1 + (2+ 3);");
        assert!(matches!(&program.stmts[0], Stmt::Print {
                expr: Expr::Binary { left, op, right },
            } if matches!(left.as_ref(),Expr::Number(1)) && matches!(op,Token::Plus) && matches!(right.as_ref(),Expr::Binary { left, op, right } if matches!(left.as_ref(),Expr::Number(2)) && matches!(op, Token::Plus) && matches!(right.as_ref(),Expr::Number(3)))
        ));
    }

    #[test]
    fn test_precedence_1() {
        let program = parse("print 1+ 2 * 3;");
        assert!(matches!(&program.stmts[0], Stmt::Print {
                expr: Expr::Binary { left, op, right },
            } if matches!(left.as_ref(),Expr::Number(1)) && matches!(op,Token::Plus) && matches!(right.as_ref(),Expr::Binary { left, op, right } if matches!(left.as_ref(),Expr::Number(2)) && matches!(op, Token::Star) && matches!(right.as_ref(),Expr::Number(3)))
        ));
    }

    #[test]
    fn test_precedence_2() {
        let program = parse("print 1* 2 + 3;");
        assert!(matches!(&program.stmts[0], Stmt::Print {
                expr: Expr::Binary { left, op, right },
            } if matches!(right.as_ref(),Expr::Number(3)) && matches!(op,Token::Plus) && matches!(left.as_ref(),Expr::Binary { left, op, right } if matches!(left.as_ref(),Expr::Number(1)) && matches!(op, Token::Star) && matches!(right.as_ref(),Expr::Number(2)))
        ));
    }

    #[test]
    fn test_precedence_3() {
        let program = parse("print 1 + 2 + 3;");
        log!("program:{program:?}");
        assert!(matches!(&program.stmts[0], Stmt::Print {
                expr: Expr::Binary { left, op, right },
            } if matches!(right.as_ref(),Expr::Number(3)) && matches!(op,Token::Plus) && matches!(left.as_ref(),Expr::Binary { left, op, right } if matches!(left.as_ref(),Expr::Number(1)) && matches!(op, Token::Plus) && matches!(right.as_ref(),Expr::Number(2)))
        ));
    }

    #[test]
    fn test_precedence_4() {
        let program = parse("print 1 * 2 * 3;");
        log!("program:{program:?}");
        assert!(matches!(&program.stmts[0], Stmt::Print {
                expr: Expr::Binary { left, op, right },
            } if matches!(right.as_ref(),Expr::Number(3)) && matches!(op,Token::Star) && matches!(left.as_ref(),Expr::Binary { left, op, right } if matches!(left.as_ref(),Expr::Number(1)) && matches!(op, Token::Star) && matches!(right.as_ref(),Expr::Number(2)))
        ));
    }

    #[test]
    fn test_precedence_5() {
        let program = parse("print 1 + 2 * 3 + 4;");
        log!("program:{program:?}");
        assert!(matches!(&program.stmts[0], Stmt::Print {
                expr: Expr::Binary { left, op, right },
            } if matches!(right.as_ref(),Expr::Number(4)) && matches!(op,Token::Plus) && matches!(left.as_ref(),Expr::Binary { left, op, right } if matches!(right.as_ref(),Expr::Binary { left, op, right } if matches!(left.as_ref(),Expr::Number(2))&& matches!(op,Token::Star) && matches!(right.as_ref(),Expr::Number(3))) && matches!(op, Token::Plus) && matches!(left.as_ref(),Expr::Number(1)))
        ));
    }

    #[test]
    fn test_comparison_parse1() {
        let program = parse("print 1 == 2;");
        assert!(
            matches!(&program.stmts[0],Stmt::Print { expr } if matches!(expr, Expr::Binary { left, op, right } if matches!(left.as_ref(),Expr::Number(1)) && matches!(op,Token::EqualEqual) && matches!(right.as_ref(),Expr::Number(2))))
        );
    }

    #[test]
    fn test_comparison_parse2() {
        let program = parse("print 1 * 2 == 2;");
        assert!(matches!(
            &program.stmts[0],
            Stmt::Print { expr }
            if matches!(
                expr, Expr::Binary { left, op, right }
                if matches!(
                    &left.as_ref(),
                    Expr::Binary { left, op, right }
                    if matches!(left.as_ref(),Expr::Number(1))
                    && matches!(op,Token::Star)
                    && matches!(right.as_ref(),Expr::Number(2))
                )
                && matches!(
                    op,
                    Token::EqualEqual)
                && matches!(
                    right.as_ref(),
                    Expr::Number(2))
            )
        ));
    }

    #[test]
    fn test_parse_match() {
        let program = parse("print match true { true => 1, _ => 2 };");
        log!("program:{program:?}");
        assert!(matches!(&program.stmts[0], Stmt::Print { expr }
        if matches!(
            expr,
            Expr::Match { subject, arms }
            if matches!(subject.as_ref(),Expr::Boolean(b) if *b == true)
            && matches!(
                &arms[0],
                MatchArm { pattern, value }
                if matches!(pattern, Pattern::Boolean(b) if *b == true)
                && matches!(value.as_ref(), Expr::Number(i) if *i == 1))
            && matches!(
                &arms[1],
                MatchArm { pattern, value }
                if matches!(pattern, Pattern::Wildcard)
                && matches!(value.as_ref(), Expr::Number(i) if *i == 2))
        )));
    }
    #[test]
    fn test_parse_string() {
        let program = parse("let x = \"hello world\";");
        assert!(matches!(
            &program.stmts[0],
            Stmt::Let { name, expr }
            if *name == *"x"
            && matches!(expr, Expr::String(s) if *s == *"hello world")
        ));
    }
}
