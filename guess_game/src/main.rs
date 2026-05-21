use std::io;
use std::cmp::Ordering;
use rand::Rng;
 
fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::rng().random_range(1..=101);
    
    println!("Please choose who guesses: you or the Computer");
    println!("(I/C)");
   
    let mut choice = String::new();
   
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
 
    let choices = choice.chars().next().unwrap(); 
    if choices == 'I' {
        println!("Your choice: {choices}. You guess.");
        println!("The secret number is: {secret_number}");
  
        loop { 
            println!("Please input your guess.");
  
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
    
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
 
            println!("You guessed: {guess}");
    
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        } 
    } else if choices == 'C' {
        println!("Your choice: {choices}. The computer guesses.");
    }

}
