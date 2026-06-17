use crate::{ast::Token, log};
use std::ops::Index;

pub fn scan(input: &str) -> Vec<Token> {
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
