use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    // println!("Hello, world!");
    // hello();
    guess_number();
}

fn hello(){
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

fn guess_number(){
    println!("Guess the number!");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("Did not enter a correct string");
    println!("You entered: {}", number);
}
