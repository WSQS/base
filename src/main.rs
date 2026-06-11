use std::{collections::HashMap, io, ops::Index};

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
enum Expr {
    Number(i64),
    Ident(String),
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Equal,
    Semicolon,
    Number(i64),
    Ident(String),
    Print,
    Let,
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
    for c in input.chars() {
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
                    } else {
                        result.push(Token::Ident(value.clone()));
                    }
                }
                value = "".to_string();
            }
        };
        if c.is_alphanumeric() {
            value = value + &c.to_string();
        } else if c == ' ' {
            flush();
        } else if c == '+' {
            flush();
            result.push(Token::Plus)
        } else if c == '-' {
            flush();
            result.push(Token::Minus)
        } else if c == '*' {
            flush();
            result.push(Token::Star)
        } else if c == '/' {
            flush();
            result.push(Token::Slash)
        } else if c == '(' {
            flush();
            result.push(Token::LParen)
        } else if c == ')' {
            flush();
            result.push(Token::RParen)
        } else if c == '=' {
            flush();
            result.push(Token::Equal)
        } else if c == ';' {
            flush();
            result.push(Token::Semicolon)
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
            } else {
                result.push(Token::Ident(value));
            }
        }
    }
    result
}

fn parse_token(token: &Token) -> Expr {
    match token {
        Token::Number(i) => Expr::Number(*i),
        Token::Ident(name) => Expr::Ident(name.clone()),
        _ => {
            println!("invalid token:{token:?}");
            Expr::Number(0)
        }
    }
}

fn parse_expr(tokens: &Vec<Token>, i: &mut usize) -> Expr {
    let t = tokens.index(*i);
    let l = parse_token(t);
    if *i + 1 > tokens.len() || matches!(tokens.index(*i + 1), Token::Semicolon) {
        *i = *i + 1;
        return l;
    }
    let t_n = tokens.index(*i + 1);
    match t_n {
        Token::Plus => {}
        Token::Minus => {}
        Token::Star => {}
        Token::Slash => {}
        Token::Equal => {}
        _ => {
            println!("invalid token:{t_n:?}");
            return l;
        }
    }
    if *i + 3 > tokens.len() {
        *i = *i + 2;
        return l;
    }
    let t_n_n = tokens.index(*i + 2);
    let r = parse_token(t_n_n);
    *i += 3;
    return Expr::Binary {
        left: Box::new(l),
        op: t_n.clone(),
        right: Box::new(r),
    };
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
                    println!("Length is not enough for print")
                }
                i += 1;
                let expr = parse_expr(&tokens, &mut i);
                let t_next_next = tokens.index(i);
                if !matches!(t_next_next, &Token::Semicolon) {
                    println!("Missing semicolon")
                }
                result.stmts.push(Stmt::Print { expr });
                i += 2
            }
            Token::Let => {
                if i + 4 > tokens.len() {
                    println!("Length is not enough for let")
                }
                let t_next = tokens.index(i + 1);
                let indent_name = if let Token::Ident(name) = t_next {
                    name.clone()
                } else {
                    "".to_string()
                };
                let t_next_next = tokens.index(i + 2);
                if !matches!(t_next_next, &Token::Equal) {
                    println!("Missing equal")
                }
                i += 3;
                let expr = parse_expr(&tokens, &mut i);
                let t_next_next_next_next = tokens.index(i);
                if !matches!(t_next_next_next_next, &Token::Semicolon) {
                    println!("Missing semicolon")
                }
                result.stmts.push(Stmt::Let {
                    name: indent_name,
                    expr,
                });
                i += 1
            }
            _ => {
                println!("invalid token:{t:?}");
                i += 1
            }
        }
    }
    result
}

fn eval_expr(expr: &Expr, env: &HashMap<String, i64>) -> i64 {
    match expr {
        Expr::Number(i) => *i,
        Expr::Binary { left, op, right } => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);
            match op {
                Token::Plus => l + r,
                Token::Minus => l - r,
                Token::Star => l * r,
                Token::Slash => l / r,
                _ => {
                    println!("Unsupported token:{op:?}");
                    0
                }
            }
        }
        Expr::Ident(name) => *env.get(name).expect("Can't get identifier"),
        _ => {
            println!("Unavailable expr:{expr:?}");
            0
        }
    }
}

fn eval_program(program: &Program) {
    let mut env = HashMap::new();
    for stmt in &program.stmts {
        match stmt {
            Stmt::Print { expr } => {
                let result = eval_expr(expr, &env);
                println!("{result}")
            }
            Stmt::Let { name, expr } => {
                let result = eval_expr(expr, &env);
                env.insert(name.clone(), result);
            }
            _ => {
                print!("Unsupported Stmt:{stmt:?}")
            }
        }
    }
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
        assert!(matches!(result, 12));
    }
}
