use crate::ast::{Expr, Pattern, Program, Stmt, Token};
use crate::log;
use crate::parse::parse;
use crate::value::Value;
use std::collections::HashMap;
use std::iter::zip;

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
        Expr::Ident(name) => env
            .get(name)
            .unwrap_or_else(|| panic!("Can't get identifier: {name}"))
            .clone(),
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
        Expr::Fn { params, body } => Value::Fn {
            params: params.to_vec(),
            body: body.as_ref().clone(),
        },
        Expr::Call { func, args } => {
            let fun = eval_expr(func, env);
            let real_args = &mut Vec::new();
            for arg in args {
                real_args.push(eval_expr(arg, env));
            }
            match fun {
                Value::Fn { params, body } => {
                    if params.len() != real_args.len() {
                        log!("Unmatched params and args number.");
                        Value::Integer(0)
                    } else {
                        let mut new_env = env.clone();
                        for (param, arg) in zip(params, real_args) {
                            new_env.insert(param, arg.clone());
                        }
                        eval_expr(&body, &new_env)
                    }
                }
                Value::BuiltinFn(func) => func(real_args.to_vec()),
                _ => {
                    log!("Expected function, get:{fun:?}");
                    Value::Integer(0)
                }
            }
        }
        Expr::List(l) => {
            let list = &mut Vec::new();
            for i in l {
                list.push(eval_expr(i, env));
            }
            Value::List(list.to_vec())
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
            Stmt::Expr { expr } => {
                eval_expr(expr, &env);
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

fn print_builtin(args: Vec<Value>) -> Value {
    match &args[0] {
        Value::Integer(i) => print!("{i}\n"),
        Value::Boolean(b) => print!("{b}\n"),
        Value::String(s) => print!("{s}\n"),
        Value::List(l) => {
            let mut first = true;
            print!("[");
            for i in l {
                if !first {
                    print!(" ,")
                }
                first = false;
                let arg = &mut Vec::new();
                arg.push(i.clone());
                print_builtin(arg.to_vec());
            }
            print!("]")
        }
        _ => print!("{:?}\n", args[0]),
    }
    args[0].clone()
}

fn len_builtin(args: Vec<Value>) -> Value {
    match &args[0] {
        Value::List(l) => Value::Integer(l.len() as i64),
        _ => {
            log!("Expected list, get:{args:?}");
            Value::Integer(0)
        }
    }
}

fn head_builtin(args: Vec<Value>) -> Value {
    match &args[0] {
        Value::List(l) => {
            if l.len() >= 1 {
                l[0].clone()
            } else {
                log!("List is empty");
                Value::Integer(0)
            }
        }
        _ => {
            log!("Expected list, get:{args:?}");
            Value::Integer(0)
        }
    }
}

fn tail_builtin(args: Vec<Value>) -> Value {
    match &args[0] {
        Value::List(l) => {
            if l.len() >= 1 {
                let mut tail = l.clone();
                tail.remove(0);
                Value::List(tail)
            } else {
                log!("List is empty");
                Value::List(Vec::new())
            }
        }
        _ => {
            log!("Expected list, get:{args:?}");
            Value::Integer(0)
        }
    }
}

fn push_builtin(args: Vec<Value>) -> Value {
    match &args[0] {
        Value::List(l) => {
            let mut new_l = l.clone();
            new_l.push(args[1].clone());
            Value::List(new_l)
        }
        _ => {
            log!("Expected list, get:{args:?}");
            Value::Integer(0)
        }
    }
}

fn get_builtin_env() -> HashMap<String, Value> {
    let mut env = HashMap::new();
    env.insert("print".to_string(), Value::BuiltinFn(print_builtin));
    env.insert("len".to_string(), Value::BuiltinFn(len_builtin));
    env.insert("head".to_string(), Value::BuiltinFn(head_builtin));
    env.insert("tail".to_string(), Value::BuiltinFn(tail_builtin));
    env.insert("push".to_string(), Value::BuiltinFn(push_builtin));
    env
}

pub fn eval_program(program: &Program) {
    let mut env = get_builtin_env();
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

    #[test]
    fn test_eval_function() {
        let program = parse("let f = fn(x){x+1};let x = f(5);");
        let env = &mut HashMap::new();
        eval_program_with_env(&program, env);

        assert!(matches!(
            env.get("x").expect("can't get x"),
            Value::Integer(i) if  *i == 6
        ));
    }

    #[test]
    fn test_eval_builtin_function() {
        let program = parse("let x = add(1,2);");
        let env = &mut HashMap::new();
        fn add_builtin(args: Vec<Value>) -> Value {
            match (&args[0], &args[1]) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
                _ => {
                    log!("add expects two integers");
                    Value::Integer(0)
                }
            }
        }
        env.insert("add".to_string(), Value::BuiltinFn(add_builtin));
        eval_program_with_env(&program, env);

        assert!(matches!(
            env.get("x").expect("can't get x"),
            Value::Integer(i) if  *i == 3
        ));
    }

    #[test]
    fn test_eval_list() {
        let program = parse("let x = [1,2];");
        let env = &mut HashMap::new();
        eval_program_with_env(&program, env);

        assert!(matches!(
            env.get("x").expect("can't get x"),
            Value::List(l)
            if l.len() == 2
            && matches!(
                l[0],
                Value::Integer(i) if i == 1
            )
            && matches!(
                l[1],
                Value::Integer(i) if i == 2
            )
        ));
    }

    #[test]
    fn test_eval_list_builtin() {
        let program = parse(
            "let x = [1,2]; let y = head(x); let z = tail(x); let l = len(z); let new_x = push(x, 1);",
        );
        let env = &mut get_builtin_env();
        eval_program_with_env(&program, env);

        assert!(matches!(
            env.get("x").expect("can't get x"),
            Value::List(l)
            if l.len() == 2
            && matches!(
                l[0],
                Value::Integer(i) if i == 1
            )
            && matches!(
                l[1],
                Value::Integer(i) if i == 2
            )
        ));
        assert!(matches!(
            env.get("y").expect("can't get y"),
            Value::Integer(1)
        ));
        assert!(matches!(
            env.get("z").expect("can't get z"),
            Value::List(l)
            if l.len() == 1
            && matches!(
                l[0],
                Value::Integer(2)
            )
        ));
        assert!(matches!(
            env.get("l").expect("can't get l"),
            Value::Integer(1)
        ));
        assert!(matches!(
            env.get("new_x").expect("can't get new_x"),
            Value::List(l)
            if l.len() == 3
            && matches!(
                l[0],
                Value::Integer(i) if i == 1
            )
            && matches!(
                l[1],
                Value::Integer(i) if i == 2
            )
            && matches!(
                l[2],
                Value::Integer(i) if i == 1
            )
        ));
    }
}
