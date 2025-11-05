// working to make a guessing game in Rust

fn main() {
    println!("Welcome to the guessing game");
    println!("Ok let's enter your string here");

    let mut guess = String::new();

    std::io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed {}",guess);

}