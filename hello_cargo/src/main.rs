use std::io;

fn main() {
    let x = 5;
    let y = 2;

    println!("The value of x is: {x} and y is: {y}");

    let mut guess = String::new();

    println!("Guess the number!");

    println!("Please input your guess : ");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
