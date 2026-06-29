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
                    } else if value == "fn" {
                        result.push(Token::Fn);
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
        } else if *c == '[' {
            flush();
            result.push(Token::LBracket)
        } else if *c == ']' {
            flush();
            result.push(Token::RBracket)
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
            } else if value == "fn" {
                result.push(Token::Fn);
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
        assert!(matches!(&tokens[5], Token::Ident(s) if s == "print"));
        assert!(matches!(&tokens[6], Token::Ident(name) if name == "x"));
        assert!(matches!(tokens[7], Token::Semicolon));
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
    fn test_scan_string() {
        let tokens = scan("let x = \"hello world\";");
        assert!(matches!(tokens[0], Token::Let));
        assert!(matches!(&tokens[1], Token::Ident(i) if *i == *"x"));
        assert!(matches!(tokens[2], Token::Equal));
        assert!(matches!(&tokens[3], Token::String(s) if *s  == *"hello world"));
        assert!(matches!(tokens[4], Token::Semicolon));
    }

    #[test]
    fn test_scan_function() {
        let tokens = scan("let add = fn(){1};");
        assert!(matches!(tokens[0], Token::Let));
        assert!(matches!(&tokens[1], Token::Ident(i) if *i == *"add"));
        assert!(matches!(tokens[2], Token::Equal));
        assert!(matches!(tokens[3], Token::Fn));
        assert!(matches!(tokens[4], Token::LParen));
        assert!(matches!(tokens[5], Token::RParen));
        assert!(matches!(tokens[6], Token::LBrace));
        assert!(matches!(tokens[7], Token::Number(i) if i == 1));
        assert!(matches!(tokens[8], Token::RBrace));
        assert!(matches!(tokens[9], Token::Semicolon));
    }

    #[test]
    fn test_scan_bracket() {
        let tokens = scan("let l = [];");
        assert!(matches!(tokens[0], Token::Let));
        assert!(matches!(&tokens[1], Token::Ident(i) if *i == *"l"));
        assert!(matches!(tokens[2], Token::Equal));
        assert!(matches!(tokens[3], Token::LBracket));
        assert!(matches!(tokens[4], Token::RBracket));
        assert!(matches!(tokens[5], Token::Semicolon));
    }
}
