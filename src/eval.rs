use crate::ast::{Expr, Pattern, Program, Stmt, Token};
use crate::log;
use crate::parse::parse;
use crate::value::Value;
use std::collections::HashMap;

pub fn eval_expr(expr: &Expr, env: &HashMap<String, Value>) -> Value {
    match expr {
        Expr::Number(i) => Value::Integer(*i),
        Expr::Boolean(b) => Value::Boolean(*b),
        Expr::String(s) => Value::String(s.clone()),
        Expr::Binary { left, op, right } => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);
            match (&l, &r) {
                (Value::Integer(lv), Value::Integer(rv)) => match op {
                    Token::Plus => Value::Integer(lv + rv),
                    Token::Minus => Value::Integer(lv - rv),
                    Token::Star => Value::Integer(lv * rv),
                    Token::Slash => Value::Integer(lv / rv),
                    Token::EqualEqual => Value::Boolean(lv == rv),
                    Token::BangEqual => Value::Boolean(lv != rv),
                    Token::Less => Value::Boolean(lv < rv),
                    Token::Greater => Value::Boolean(lv > rv),
                    Token::LessEqual => Value::Boolean(lv <= rv),
                    Token::GreaterEqual => Value::Boolean(lv >= rv),
                    _ => {
                        log!("Unsupported token:{op:?}");
                        Value::Integer(0)
                    }
                },
                _ => {
                    log!("Unsupported Binary Operation:{l:?} {op:?} {r:?}");
                    Value::Integer(0)
                }
            }
        }
        Expr::Ident(name) => env.get(name).expect("Can't get identifier").clone(),
        Expr::Match { subject, arms } => {
            let s = eval_expr(subject, env);
            eval_expr(
                arms.iter()
                    .find(|arm| match (&s, &arm.pattern) {
                        (Value::Boolean(b_v), Pattern::Boolean(b_p)) if b_v == b_p => true,
                        (Value::Integer(i_v), Pattern::Number(i_p)) if i_v == i_p => true,
                        (_, Pattern::Wildcard) => true,
                        _ => false,
                    })
                    .expect("Can't get right arm.")
                    .value
                    .as_ref(),
                env,
            )
        }
        _ => {
            log!("Unavailable expr:{expr:?}");
            Value::Integer(0)
        }
    }
}

pub fn eval_program_with_env(program: &Program, env: &mut HashMap<String, Value>) {
    for stmt in &program.stmts {
        match stmt {
            Stmt::Print { expr } => {
                let result = eval_expr(expr, &env);
                match result {
                    Value::Integer(i) => print!("{i}\n"),
                    Value::Boolean(b) => print!("{b}\n"),
                    Value::String(s) => print!("{s}\n"),
                    _ => print!("Invalid value:{result:?}"),
                }
            }
            Stmt::Let { name, expr } => {
                let result = eval_expr(expr, &env);
                env.insert(name.clone(), result);
            }
            _ => {
                log!("Unsupported Stmt:{stmt:?}")
            }
        }
    }
}

pub fn eval_program(program: &Program) {
    let mut env = HashMap::new();
    eval_program_with_env(program, &mut env);
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_eval_match() {
        let program = parse("let x = true;let y = match x { true => 1, _ => 2 };");
        let env = &mut HashMap::new();
        eval_program_with_env(&program, env);
        assert!(matches!(env.get("y").expect("Can't get y"),Value::Integer(i) if *i == 1))
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
