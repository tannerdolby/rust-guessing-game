use std::io; // using the input/output module from the standard library
use rand::Rng; // grab the rand library crate for random numbers
use std::cmp::Ordering; // grab compare submodule from standard library and Ordering enum

fn main() {
    
    println!("Guess a number between 1-10");
    
    let secret_number = rand::thread_rng().gen_range(1, 11); // use associated function thread_rng() for particular rng generator, one that is local to the current thread of execution and seeded by the operating system.
    
    loop {
        
        println!("Please input your guess:");

        let mut guess = String::new(); // creates a mutable variable bound to new empty instance of String type ie an empty growable string (new associated function implemented on the String type)

        // calling the stdin function from the io module to read bytes input by user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // trim away the \n after hitting enter of user input and parse String type to u32 numerical
        // handling invalid input ie non-number guess by continuing the program rather than crashing, match has two arms here
        let guess: u32 = match guess.trim().parse() {
            // specifying the Result enum variants Ok and Err for Result type
            Ok(num) => num,
            Err(_) => continue,
        };
            
        println!("You guessed: {}", guess);

        // compare the secret number with user input using a match expression (made of 3 arms from the cmp method)
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small, guess again"),
            Ordering::Greater => println!("Too big, guess again"),
            Ordering::Equal => {
                println!("You win! You've guessed the secret number {} correctly!", secret_number);
                break;
            }
        }
    }
}
