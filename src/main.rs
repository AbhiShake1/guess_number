use std::cmp::Ordering;
use std::io::stdin;
use rand::{Rng, thread_rng};

fn main() {
    let random = thread_rng().gen_range(1..=10);

    println!("Enter your guess from 1 to 10: ");

    loop {
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("failed to read");
        let guess_num: i32 = guess.trim().parse().expect("invalid number");

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
