mod ownership;

use ferris_says::say;
use rand::Rng;
use std::io::{BufWriter, stdout};
use crate::ownership::ownership_entry;

fn main() {
    // println!("Hello, world!");
    // hello();
    //guess_number();
    ownership_entry();
}

fn hello() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

fn guess_number() {
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(1..=100);
    //println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");
        let mut guess_number = String::new();
        std::io::stdin()
            .read_line(&mut guess_number)
            .expect("Did not enter a correct string");
        println!("You entered: {}", guess_number);
        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input error num, please input a number again!");
                continue;
            }
        };
        match guess_number.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
