#[derive(Debug)]
enum Token {
    Number(i64),
}

fn main() {
    println!("Hello, world from main!");
    let tokens = scan(&"12 34 5".to_string());
    for t in tokens {
        println!("{:?}", t)
    }
}

fn scan(input: &String) -> Vec<Token> {
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
        } else {
            println!("Not support char:{c}")
        }
    }
    if has_value {
        result.push(Token::Number(value));
    }
    result
}

#[test]
fn test_number_values() {
    let tokens = scan(&"12 34  5".to_string());
    assert!(matches!(tokens[0], Token::Number(12)));
    assert!(matches!(tokens[1], Token::Number(34)));
    assert!(matches!(tokens[2], Token::Number(5)));
}
