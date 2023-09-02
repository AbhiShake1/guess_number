use std::cmp::Ordering;
use std::io::stdin;
use rand::{Rng, thread_rng};

fn main() {
    let random = thread_rng().gen_range(1..=10);

    loop {
        println!("Enter your guess from 1 to 10: ");

        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("failed to read");
        let guess_num: i32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };

        match guess_num.cmp(&random) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("congrats! {guess_num} is the right guess");
                break;
            }
        }
    }
}
