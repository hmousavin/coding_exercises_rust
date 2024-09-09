use std::io;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number?");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if secret < guess {
            println!("It's not that much!");
        }
        else if guess < secret {
            println!("It's bigger than this!");
        }
        else {
            println!("You guessed the correct value: {secret} \n well done!");
            break;
        }
    }
}