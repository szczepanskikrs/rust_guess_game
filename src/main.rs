use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess");

        let mut guessed = String::new();

        io::stdin()
            .read_line(&mut guessed)
            .expect("Failed to read line");

        let guessed_value: u32 = match guessed.trim().parse() {
            Ok(number) => number,
            Err(_) => continue
        };

        match guessed_value.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("This is it!");
                break;
            }
        }
    }
}
