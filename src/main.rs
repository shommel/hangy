use random_word::gen;
use std::io;

fn main() {
    print_intro();
    
    // choose random word for user to guess
    let target_word = gen();

    println!("random word is: {}", target_word);

    // number of wrong guesses user has, 6 and they lose!
    let mut wrong_guesses: u8 = 0;

    loop {
        print_hangman(wrong_guesses);
        
        // check if user lost
        if wrong_guesses >= 6 {
            println!("You lose, the word was {}", target_word);
            break;
        }

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

        if !target_word.contains(guess) {
            wrong_guesses+=1;
            println!("Incorrect!");
        }

        else {
            println!("Correct!");
        }

    }

}

fn print_intro(){
    println!("Welcome to Hangy, a hangman game written in rust");
    println!("A random word will be chosen... and you need to guess it!");
    println!("To quit at any time, press ~");
}

fn print_hangman(wrong_guesses: u8) {
    // prints the proper hangman ascii depending on number of wrong guesses

    let hangman_str: &str = match wrong_guesses {
        0 => "\n  +---+\n  |   |\n      |\n      |\n      |\n      |\n=========",
        1 => "\n  +---+\n  |   |\n  O   |\n      |\n      |\n      |\n=========",
        2 => "\n  +---+\n  |   |\n  O   |\n  |   |\n      |\n      |\n=========",
        3 => "\n  +---+\n  |   |\n  O   |\n /|   |\n      |\n      |\n=========",
        4 => "\n  +---+\n  |   |\n  O   |\n /|\\  |\n      |\n      |\n=========",
        5 => "\n  +---+\n  |   |\n  O   |\n /|\\  |\n /    |\n      |\n=========",
        _ => "\n  +---+\n  |   |\n  O   |\n /|\\  |\n / \\  |\n      |\n=========",
    };
    println!("{}", hangman_str);
}
