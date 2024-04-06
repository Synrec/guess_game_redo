use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main (){
    println!("Guess the number!");
    println!("Your Input?");
    let mut guess = String::new();
    let _correct = rand::thread_rng().gen_range(1..=10);
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    let guess: u32 = guess.trim().replace(" ", "").parse().expect("Please Type A Valid Number!!!!!");
    println!("You guessed: {guess}");
    match guess.cmp(&_correct){
        Ordering::Less => println!("{guess} is too small. Correct number is: {_correct}"),
        Ordering::Greater => println!("{guess} is too big. Correct number is: {_correct}"),
        Ordering::Equal => println!("{guess} is equal to: {_correct}, you win!")
    }
    println!("Thanks for playing!");
}