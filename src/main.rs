use std::{collections::HashMap, io, ops::Index};
mod value;
use value::Value;
mod ast;
use ast::{Expr, MatchArm, Pattern, Program, Stmt, Token};
mod scan;
use scan::scan;
mod parse;
mod util;
use parse::parse;
mod eval;
use eval::{eval_expr, eval_program, eval_program_with_env};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error in read_line");
    let program = parse(&input);
    eval_program(&program);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_values() {
        let tokens = scan("12 34  5");
        assert!(matches!(tokens[0], Token::Number(12)));
        assert!(matches!(tokens[1], Token::Number(34)));
        assert!(matches!(tokens[2], Token::Number(5)));
    }

    #[test]
    fn test_plus() {
        let tokens = scan("12+34 + 5");
        assert!(matches!(tokens[0], Token::Number(12)));
        assert!(matches!(tokens[1], Token::Plus));
        assert!(matches!(tokens[2], Token::Number(34)));
        assert!(matches!(tokens[3], Token::Plus));
        assert!(matches!(tokens[4], Token::Number(5)));
    }
    #[test]
    fn test_four_sign() {
        let tokens = scan("12 - 3 *4/2");
        assert!(matches!(tokens[0], Token::Number(12)));
        assert!(matches!(tokens[1], Token::Minus));
        assert!(matches!(tokens[2], Token::Number(3)));
        assert!(matches!(tokens[3], Token::Star));
        assert!(matches!(tokens[4], Token::Number(4)));
        assert!(matches!(tokens[5], Token::Slash));
        assert!(matches!(tokens[6], Token::Number(2)));
    }
    #[test]
    fn test_statement_symbols() {
        let tokens = scan("(12+3)=15;");
        assert!(matches!(tokens[0], Token::LParen));
        assert!(matches!(tokens[1], Token::Number(12)));
        assert!(matches!(tokens[2], Token::Plus));
        assert!(matches!(tokens[3], Token::Number(3)));
        assert!(matches!(tokens[4], Token::RParen));
        assert!(matches!(tokens[5], Token::Equal));
        assert!(matches!(tokens[6], Token::Number(15)));
        assert!(matches!(tokens[7], Token::Semicolon));
    }
    #[test]
    fn test_identifier() {
        let tokens = scan("(foo+3)=bar;");
        assert!(matches!(tokens[0], Token::LParen));
        assert!(matches!(&tokens[1], Token::Ident(name) if name == "foo"));
        assert!(matches!(tokens[2], Token::Plus));
        assert!(matches!(tokens[3], Token::Number(3)));
        assert!(matches!(tokens[4], Token::RParen));
        assert!(matches!(tokens[5], Token::Equal));
        assert!(matches!(&tokens[6], Token::Ident(name) if name == "bar"));
        assert!(matches!(tokens[7], Token::Semicolon));
    }

    #[test]
    fn test_keyword() {
        let tokens = scan("let x = 1; print x;");
        assert!(matches!(tokens[0], Token::Let));
        assert!(matches!(&tokens[1], Token::Ident(name) if name == "x"));
        assert!(matches!(tokens[2], Token::Equal));
        assert!(matches!(tokens[3], Token::Number(1)));
        assert!(matches!(tokens[4], Token::Semicolon));
        assert!(matches!(tokens[5], Token::Print));
        assert!(matches!(&tokens[6], Token::Ident(name) if name == "x"));
        assert!(matches!(tokens[7], Token::Semicolon));
    }

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
    fn test_eval_expr() {
        let result = eval_expr(
            &Expr::Binary {
                left: Box::new(Expr::Number(5)),
                op: Token::Plus,
                right: Box::new(Expr::Number(7)),
            },
            &HashMap::new(),
        );
        assert!(matches!(result, Value::Integer(i)if i ==12));
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
    fn test_less_great() {
        let tokens = scan(" 5 > 6 < 7");
        assert!(matches!(tokens[0],Token::Number(i) if i == 5));
        assert!(matches!(tokens[1], Token::Greater));
        assert!(matches!(tokens[2],Token::Number(i) if i == 6));
        assert!(matches!(tokens[3], Token::Less));
        assert!(matches!(tokens[4],Token::Number(i) if i == 7));
    }

    #[test]
    fn test_double_char_comparison_scan() {
        let tokens = scan("1 == 2 != 3 <= 4 >= 5");
        assert!(matches!(tokens[0],Token::Number(i) if i == 1));
        assert!(matches!(tokens[1], Token::EqualEqual));
        assert!(matches!(tokens[2],Token::Number(i) if i == 2));
        assert!(matches!(tokens[3], Token::BangEqual));
        assert!(matches!(tokens[4],Token::Number(i) if i == 3));
        assert!(matches!(tokens[5], Token::LessEqual));
        assert!(matches!(tokens[6],Token::Number(i) if i == 4));
        assert!(matches!(tokens[7], Token::GreaterEqual));
        assert!(matches!(tokens[8],Token::Number(i) if i == 5));
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
    fn test_comparison_eval() {
        let program = parse("let x = 1 == 2;");
        let mut env = HashMap::new();
        eval_program_with_env(&program, &mut env);
        assert!(matches!(
            env.get("x").expect("Nox variable x"),
            Value::Boolean(b) if matches!(b,false)
        ));
    }
    #[test]
    fn test_boolean_literals() {
        let program = parse("let x = true; let y = false;");
        let mut env = HashMap::new();
        eval_program_with_env(&program, &mut env);
        assert!(matches!(
            env.get("x").expect("Nox variable x"),
            Value::Boolean(b) if matches!(b,true)
        ));
        assert!(matches!(
            env.get("y").expect("Nox variable y"),
            Value::Boolean(b) if matches!(b,false)
        ));
    }

    #[test]
    fn test_scan_match() {
        let tokens = scan("match true { true => 1, _ => 2 }");
        assert!(matches!(tokens[0], Token::Match));
        assert!(matches!(tokens[1], Token::True));
        assert!(matches!(tokens[2], Token::LBrace));
        assert!(matches!(tokens[3], Token::True));
        assert!(matches!(tokens[4], Token::EqualGreater));
        assert!(matches!(tokens[5],Token::Number(i) if i == 1));
        assert!(matches!(tokens[6], Token::Comma));
        assert!(matches!(tokens[7], Token::Wildcard));
        assert!(matches!(tokens[8], Token::EqualGreater));
        assert!(matches!(tokens[9],Token::Number(i)if i == 2));
        assert!(matches!(tokens[10], Token::RBrace));
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
    fn test_eval_match() {
        let program = parse("let x = true;let y = match x { true => 1, _ => 2 };");
        let env = &mut HashMap::new();
        eval_program_with_env(&program, env);
        assert!(matches!(env.get("y").expect("Can't get y"),Value::Integer(i) if *i == 1))
    }

    #[test]
    fn test_scan_string() {
        let tokens = scan("let x = \"hello world\";");
        assert!(matches!(tokens[0], Token::Let));
        assert!(matches!(&tokens[1], Token::Ident(i) if *i == *"x"));
        assert!(matches!(tokens[2], Token::Equal));
        assert!(matches!(&tokens[3], Token::String(s) if *s  == *"hello world"));
        assert!(matches!(tokens[4], Token::Semicolon));
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

    #[test]
    fn test_eval_string() {
        let program = parse("let x = \"hello world\";");
        let env = &mut HashMap::new();
        eval_program_with_env(&program, env);

        assert!(matches!(
            env.get("x").expect("can't get x"),
            Value::String(s) if *s == *"hello world"
        ));
    }
}
