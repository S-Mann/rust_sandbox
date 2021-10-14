#[derive(Debug)]
pub enum GrammarItem {
    Sum,
    Sub,
    Number(u32),
}

#[derive(Debug)]
pub enum LexItem {
    Operator(char),
    Number(u32),
}

fn lex(input: &str) {
    let mut result = Vec::new();
    let radix: u32 = 10;

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                let mut n = 0;
                let mut counter = 0;
                while let Some(&val) = it.peek() {
                    if let Some(digit) = val.to_digit(radix) {
                        if counter >= 1 {
                            n *= 10;
                        }
                        n += digit;
                        counter += 1;
                        it.next();
                    } else {
                        break;
                    }
                }
                result.push(LexItem::Number(n));
                println!("'0'..='9' => {:?}", c);
                it.next();
            }
            '+' | '-' => {
                result.push(LexItem::Operator(c));
                println!("'+' | '-' => {:?}", c);
                it.next();
            }
            _ => {
                println!("        _ => {:?}", c);
                it.next();
            }
        }
    }
    println!("{:?}", result);
}

fn main() {
    lex("1 -020 +3 ");
}
