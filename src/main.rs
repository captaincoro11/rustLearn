use std::cmp::Ordering;

// Making a guessing game in Rust
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("Enter your guess");
        std::io::stdin().read_line(& mut guess).expect("Failed to enter a string");
        let guess : u32 = guess.trim().parse().expect("expecting a number here");
        
        match guess.cmp(& secret_number) {
            Ordering::Less => println!("You're less"),
            Ordering::Greater => println!("You're greater"),
            Ordering::Equal => {
                println!("Right Answer");
                break;
            }
        }
    }

}