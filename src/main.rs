use std::io;

fn main (){
    println!("Guess the number!");
    println!("Your Input?");
    let mut guess = String::new();
    let _correct = 5;
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    println!("You guessed: {guess}");
    println!("Correct answer is: {_correct}");
    println!("Thanks for playing!");
}