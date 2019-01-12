use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut num_tries = 1;
    println!("Guess the number");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                num_tries += 1;
            },
            Ordering::Greater => {
                println!("Too big!");
                num_tries += 1;
            },
            Ordering::Equal => {
                if num_tries > 1 {
                    println!("You win! It took you {} tries", num_tries.to_string());
                } else {
                    println!("You win! It only took you 1 attempt, Great job!");
                }

                break;
            }
        }
    }
}