use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guessing game");
    
    loop {
        // this will put new commit changes here
        let sec_number = rand::thread_rng().gen_range(1..=100);
        println!("please enter your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&sec_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal =>{
                println!("you got it right");
                break;
            }
        }
        println!("You guessed: {} and my new number is {}",guess,sec_number);
    }
}
