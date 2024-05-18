use std::{
    error::Error,
    io::{self, Write},
};

use rand::Rng;

fn main() -> Result<(), Box<dyn Error>> {
    let mut line = String::new();
    let random_number = rand::thread_rng().gen_range(0, 101);
    loop {
        print!("Enter your Guess (number) #> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut line)?;
        let guess: i32 = match line.trim().parse() {
            Ok(n) => {
                println!("Guessed Number: {}.", n);
                n
            }
            Err(_) => continue,
        };
        match guess {
            _ if guess > random_number => println!("To high."),
            _ if guess < random_number => println!("To low."),
            _ => {
                println!("we have a winner.");
                break Ok(());
            }
        }
        line.clear();
    }
}
