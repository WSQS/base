use std::io;

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
}
