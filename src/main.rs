use random_word::gen;
use std::io;

fn main() {
    display_intro();
    
    // choose random word for user to guess
    let target_word = gen();

    println!("random word is: {}", target_word);

    loop {
        println!("Enter your guesss: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // check user input
        let guess: char = match guess.trim().parse() {
            Ok(char) => char,
            Err(_) => {
                println!("Error: Enter only letters.");
                continue;
            },
        };

        // check for special character to quit program
        if guess == '~' {
            println!("Exiting program.");
            break;
        }

        else if !guess.is_alphabetic() {
            println!("Error: Enter only letters.");
            continue;
        }

        println!("your guess was {}", guess);
    }

}

fn display_intro(){
    println!("Welcome to Hangy, a hangman game written in rust");
    println!("A random word will be chosen... and you need to guess it!");
    println!("To quit at any time, press ~");
}

