use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Generate a random number using thread seeded RNG
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut num_tries = 1;

    println!("Guess the number");

    // Start up a loop, allows game to keep running while user is guessing
    loop {
        println!("Please input your guess.");

        // Create a mutable string variable to hold the user's guess
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        /*
            Shadow the guess string with a u32 variable of the same name,
            This allows us to use match to properly compare it's value
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // Return value if operation was successful
            Err(_) => continue // Junk out non integer input, return to top of loop
        };

        println!("You guessed: {}", guess);

        // Compare the shadowed version of guess
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

                break; // Exit the loop once the user guesses right
            }
        }
    }
}