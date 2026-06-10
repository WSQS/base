use std::{io, ops::Index};

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

#[derive(Debug)]
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
    let tokens = scan(&input);
    for t in tokens {
        println!("{:?}", t)
    }
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
            result.push(Token::Ident(value));
        }
    }
    result
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
                let t_next = tokens.index(i + 1);
                let expr = match t_next {
                    Token::Number(i) => Expr::Number(*i),
                    Token::Ident(name) => Expr::Ident(name.clone()),
                    _ => {
                        println!("invalid token:{t:?}");
                        Expr::Number(0)
                    }
                };
                let t_next_next = tokens.index(i + 2);
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
                let t_next_next_next = tokens.index(i + 3);
                let expr = match t_next_next_next {
                    Token::Number(i) => Expr::Number(*i),
                    Token::Ident(name) => Expr::Ident(name.clone()),
                    _ => {
                        println!("invalid token:{t:?}");
                        Expr::Number(0)
                    }
                };
                let t_next_next_next_next = tokens.index(i + 4);
                if !matches!(t_next_next_next_next, &Token::Semicolon) {
                    println!("Missing semicolon")
                }
                result.stmts.push(Stmt::Let {
                    name: indent_name,
                    expr,
                });
                i += 4
            }
            _ => {
                println!("invalid token:{t:?}");
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
}
