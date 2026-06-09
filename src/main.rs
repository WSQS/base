use std::io;

#[derive(Debug)]
enum Token {
    Number(i64),
    Plus,
}

fn main() {
    println!("Hello, world from main!");
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
    println!("get input:{input}");
    let mut result = Vec::new();
    let mut has_value = false;
    let mut value: i64 = 0;
    for c in input.chars() {
        if c.is_numeric() {
            has_value = true;
            let digit = c.to_digit(10).unwrap() as i64;
            value = value * 10 + digit;
        } else if c == ' ' {
            if has_value {
                result.push(Token::Number(value));
                value = 0;
            }
            has_value = false;
        } else if c == '+' {
            if has_value {
                result.push(Token::Number(value));
                value = 0;
            }
            has_value = false;
            result.push(Token::Plus)
        } else {
            println!("Not support char:\"{c}\"")
        }
    }
    if has_value {
        result.push(Token::Number(value));
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_values() {
        let tokens = scan(&"12 34  5".to_string());
        assert!(matches!(tokens[0], Token::Number(12)));
        assert!(matches!(tokens[1], Token::Number(34)));
        assert!(matches!(tokens[2], Token::Number(5)));
    }

    #[test]
    fn test_plus() {
        let tokens = scan(&"12+34 + 5".to_string());
        assert!(matches!(tokens[0], Token::Number(12)));
        assert!(matches!(tokens[1], Token::Plus));
        assert!(matches!(tokens[2], Token::Number(34)));
        assert!(matches!(tokens[3], Token::Plus));
        assert!(matches!(tokens[4], Token::Number(5)));
    }
}
