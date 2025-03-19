use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
   
    println!("Guess the number!");

    // Create secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    
    loop {
        // Prompt user for input
        println!("Please input your guess.");

        // Instantiate guess
        let mut guess = String::new();

        // Get user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // Parse input to int
        let guess: u32 = match guess.trim().parse() {
            // On invalid input, continue 
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }

        }
        
    }
    


}

