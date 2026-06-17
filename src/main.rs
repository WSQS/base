use std::{collections::HashMap, io, ops::Index};

macro_rules! log {
    ($($arg:tt)*) => {
        println!("[{}:{}] {}", file!(), line!(), format_args!($($arg)*))
    };
}

#[derive(Debug, Clone)]
enum Value {
    Integer(i64),
    Boolean(bool),
}

#[derive(Debug)]
struct Program {
    stmts: Vec<Stmt>,
}

#[derive(Debug)]
enum Stmt {
    Let { name: String, expr: Expr },
    Print { expr: Expr },
}

#[derive(Debug)]
enum Pattern {
    Number(i64),
    Boolean(bool),
    Wildcard,
}

#[derive(Debug)]
struct MatchArm {
    pattern: Pattern,
    value: Box<Expr>,
}

#[derive(Debug)]
enum Expr {
    Number(i64),
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
enum Token {
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
    True,
    False,
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error in read_line");
    let program = parse(&input);
    eval_program(&program);
}

fn scan(input: &str) -> Vec<Token> {
    let mut result = Vec::new();
    let mut value: String = "".to_string();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars.index(i);
        i += 1;
        let mut flush = || {
            if !value.is_empty() {
                let head = value.chars().nth(0);
                if head.is_some_and(|head| head.is_numeric()) {
                    result.push(Token::Number(
                        value.parse::<i64>().expect("Parse int failed"),
                    ));
                } else {
                    if value == "let" {
                        result.push(Token::Let);
                    } else if value == "print" {
                        result.push(Token::Print);
                    } else if value == "true" {
                        result.push(Token::True);
                    } else if value == "false" {
                        result.push(Token::False);
                    } else if value == "match" {
                        result.push(Token::Match);
                    } else if value == "_" {
                        result.push(Token::Wildcard);
                    } else {
                        result.push(Token::Ident(value.clone()));
                    }
                }
                value = "".to_string();
            }
        };
        if c.is_alphanumeric() || *c == '_' {
            value = value + &c.to_string();
        } else if *c == '"' {
            flush();
            let mut c = chars.index(i);
            i += 1;
            let mut s = String::new();
            while *c != '"' {
                s += &*c.to_string();
                c = chars.index(i);
                i += 1;
            }
            result.push(Token::String(s));
        } else if *c == ' ' {
            flush();
        } else if *c == '+' {
            flush();
            result.push(Token::Plus)
        } else if *c == '-' {
            flush();
            result.push(Token::Minus)
        } else if *c == '*' {
            flush();
            result.push(Token::Star)
        } else if *c == '/' {
            flush();
            result.push(Token::Slash)
        } else if *c == '<' {
            flush();
            let c = chars.index(i);
            i += 1;
            if *c == '=' {
                result.push(Token::LessEqual);
            } else {
                i -= 1;
                result.push(Token::Less);
            }
        } else if *c == '>' {
            flush();
            let c = chars.index(i);
            i += 1;
            if *c == '=' {
                result.push(Token::GreaterEqual);
            } else {
                i -= 1;
                result.push(Token::Greater);
            }
        } else if *c == '(' {
            flush();
            result.push(Token::LParen)
        } else if *c == ')' {
            flush();
            result.push(Token::RParen)
        } else if *c == '{' {
            flush();
            result.push(Token::LBrace)
        } else if *c == '}' {
            flush();
            result.push(Token::RBrace)
        } else if *c == '=' {
            flush();
            let c = chars.index(i);
            i += 1;
            if *c == '=' {
                result.push(Token::EqualEqual);
            } else if *c == '>' {
                result.push(Token::EqualGreater);
            } else {
                i -= 1;
                result.push(Token::Equal);
            }
        } else if *c == '!' {
            flush();
            let c = chars.index(i);
            i += 1;
            if *c == '=' {
                result.push(Token::BangEqual);
            } else {
                i -= 1;
                log!("Invalid char: {c:?}")
            }
        } else if *c == ';' {
            flush();
            result.push(Token::Semicolon)
        } else if *c == ',' {
            flush();
            result.push(Token::Comma)
        } else {
            println!("Not support char:\"{c}\"")
        }
    }
    if !value.is_empty() {
        let head = value.chars().nth(0);
        if head.is_some_and(|head| head.is_numeric()) {
            result.push(Token::Number(
                value.parse::<i64>().expect("Parse int failed"),
            ));
        } else {
            if value == "let" {
                result.push(Token::Let);
            } else if value == "print" {
                result.push(Token::Print);
            } else if value == "true" {
                result.push(Token::True);
            } else if value == "false" {
                result.push(Token::False);
            } else if value == "match" {
                result.push(Token::Match);
            } else if value == "_" {
                result.push(Token::Wildcard);
            } else {
                result.push(Token::Ident(value));
            }
        }
    }
    result
}

fn parse_primary(tokens: &Vec<Token>, i: &mut usize) -> Expr {
    let t = tokens.index(*i);
    *i += 1;
    match t {
        Token::Number(i) => Expr::Number(*i),
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

fn parse(input: &str) -> Program {
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

fn eval_expr(expr: &Expr, env: &HashMap<String, Value>) -> Value {
    match expr {
        Expr::Number(i) => Value::Integer(*i),
        Expr::Boolean(b) => Value::Boolean(*b),
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

fn eval_program_with_env(program: &Program, env: &mut HashMap<String, Value>) {
    for stmt in &program.stmts {
        match stmt {
            Stmt::Print { expr } => {
                let result = eval_expr(expr, &env);
                match result {
                    Value::Integer(i) => print!("{i}\n"),
                    Value::Boolean(b) => print!("{b}\n"),
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

fn eval_program(program: &Program) {
    let mut env = HashMap::new();
    eval_program_with_env(program, &mut env);
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
    fn test_scan_string () {
        let tokens = scan("let x = \"hello world\";");
        assert!(matches!(tokens[0], Token::Let));
        assert!(matches!(&tokens[1], Token::Ident(i) if *i == *"x"));
        assert!(matches!(tokens[2], Token::Equal));
        assert!(matches!(&tokens[3], Token::String(s) if *s  == *"hello world"));
        assert!(matches!(tokens[4], Token::Semicolon));
    }
}
