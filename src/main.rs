use std::iter::Peekable;

#[derive(Debug)]
enum LexItem {
    Number,
    Operator,
}

#[derive(Debug)]
struct ParseToken {
    value: Vec<char>,
    token_type: LexItem,
}

fn check_type(c: &char) -> Option<LexItem> {
    match c {
        '0'..='9' => Some(LexItem::Number),
        '+' | '*' => Some(LexItem::Operator),
        _ => None,
    }
}

fn fast_forward<I>(initial_value: char, it: &mut Peekable<I>) -> Vec<char>
where
    I: Iterator<Item = char>,
{
    let mut result = vec![initial_value];
    while let Some(&c) = it.peek() {
        if let Some(LexItem::Number) = check_type(&c) {
            result.push(c);
            it.next();
        } else {
            break;
        }
    }
    result
}

fn tokenizer<I>(it: &mut Peekable<I>) -> Vec<ParseToken>
where
    I: Iterator<Item = char>,
{
    let mut result = Vec::<ParseToken>::new();
    while let Some(&c) = it.peek() {
        it.next();
        if let Some(item) = check_type(&c) {
            match item {
                LexItem::Number => {
                    result.push(ParseToken {
                        value: fast_forward(c, it),
                        token_type: LexItem::Number,
                    });
                }
                LexItem::Operator => {
                    result.push(ParseToken {
                        value: vec![c],
                        token_type: LexItem::Operator,
                    });
                }
            }
        }
    }
    result
}

fn main() {
    let inp_val = "1     +  20   *9";
    let mut iter = inp_val.chars().peekable();
    let result = tokenizer(&mut iter);
    println!("Amazing Lexing: {:?}", result);
}
