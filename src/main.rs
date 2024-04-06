use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main (){
    println!("Guess the number!");
    let _correct = rand::thread_rng().gen_range(1..=10);
    let mut guesses: u32 = 0;
    loop {
        guesses += 1;
        let mut guess = String::new();
        println!("Your Input?");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        let guess: u32 = match guess.trim().replace(" ", "").parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("You guessed: {guess}");
        match guess.cmp(&_correct){
            Ordering::Less => println!("{guess} is a bit too low."),
            Ordering::Greater => println!("{guess} is a bit too high."),
            Ordering::Equal => {
                println!("{guess} is the correct number!! You win with {guesses} guesses!");
                break;
            }
        }
    }
    println!("Thanks for playing!");
}