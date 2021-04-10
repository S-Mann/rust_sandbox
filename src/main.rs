use rand::Rng;
use std::io;

fn main() {
    println!("Guess a number");
    println!("Please intput your number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Rip");

    println!("You guessed: {}", guess);
}
