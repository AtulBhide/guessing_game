use std::io;


fn main() {
    println!("Guess the number!");
    println!("Please put in your guess.");

    let mut guess = String::new();


    io::stdin()
        .read_line( &mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    let str_len = guess.len();

    println!("Input size: {str_len}"); // expect +2 to the expected length as EOL ("\r\n" on windows and "\n" on Linux/Unix) are part of the input string (see definition of the read_line functionX)
}
