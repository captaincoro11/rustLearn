// working to make a guessing game in Rust

use rand::Rng;

fn main() {
    println!("Welcome to the guessing game");
    println!("Ok let's enter your string here");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {}",secret_number);

    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed {}",guess);

}