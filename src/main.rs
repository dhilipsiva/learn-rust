use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Please enter a number: ");
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read from command line");
    let guess: i32 = guess
        .trim()
        .parse()
        .expect("Please enter a number to parse");
    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("Correct"),
        Ordering::Greater => println!("Too big"),
        Ordering::Less => println!("Too small"),
    }
    println!("The number you entered is: {}", guess);
    println!("The number I guessed is: {}", secret_number);
}
