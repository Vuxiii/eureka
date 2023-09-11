use std::borrow::BorrowMut;
use std::iter::Peekable;
use std::str::Chars;

const INPUT: &str = r#"
2 + 3
"#;

enum Token {
    Number(i64),
    Plus,
}

struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
}

fn main() {
    let mut input = INPUT.chars().peekable();
    let peeked = input.peek();
    while let Some(car) = peeked {
        if car.is_digit(10) {
            // We have found a number :)
            let asdf: String = input.take_while(|diller| diller.is_digit(10)).collect();
            let number: i64 = match asdf.parse() {
                Ok(value) => value,
                Err(error) => panic!("ad"),
            };
        }
    }
}

fn parse(input: Peekable<Chars<'static>>) -> Vec<Token> {
    unimplemented!();
}
fn parse_number<'a>(lexer: &'a mut Lexer) -> &'a Lexer<'a> {
    match lexer
        .input
        .borrow_mut()
        .take_while(|current| current.is_digit(10))
        .collect::<String>()
        .parse::<i64>()
    {
        Ok(number) => {
            lexer.tokens.push(Token::Number(number));
            lexer.as_ref()
        }
        Err(error) => panic!("{error}"),
    }
}
