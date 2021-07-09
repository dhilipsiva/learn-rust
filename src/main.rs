use rand::Rng;
use std::io;

fn main() {
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Please enter a number: ");
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read from command line");
    println!("The number you entered is: {}", guess);
    println!("The number I guessed is: {}", secret_number);
}
