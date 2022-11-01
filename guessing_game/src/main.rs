use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("secret number = {secret_number}");

    loop {
        println!("input your number :)");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line :(");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too smmall!"),
            Ordering::Greater => println!("too large!"),
            Ordering::Equal => {
                println!("good job, this is correct! :)");
                break;
            }
        }
    }
}
