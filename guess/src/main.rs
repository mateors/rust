use std::io;
use rand::Rng;


fn main() {

    //https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("the secret number is {}",secret_number);

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("you gusessd: {}",guess)
}
