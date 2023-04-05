use random_word::gen;
use std::io;

fn print_intro() {
    println!("Welcome to Hangy, a Hangman game written in Rust");
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

fn main() {
    print_intro();
    
    // choose random word for user to guess
    let target_word = gen();

    let mut wrong_guesses: u8 = 0;
    let mut gusssed_letters = Vec::new();

    let mut user_guessed_word = vec!['*'; target_word.len()];

    loop {
        print_hangman(wrong_guesses);
        println!("{:}", user_guessed_word.iter().collect::<String>());

        // check if user won or lost
        if wrong_guesses >= 6 {
            println!("You lost. The word was {}", target_word);
            break;
        }

        else if !user_guessed_word.contains(&'*') {
            println!("You won!");
            break;
        }

        // print sorted collection of user guessed letters
        println!(
            "Guessed letters: {}",
            gusssed_letters.iter().collect::<String>()
        );

        println!("Enter your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // check user input
        let guess: char = match guess.trim().to_lowercase().parse() {
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

        // check for already guessed letters
        else if gusssed_letters.contains(&guess) {
            println!("Error: Already guessed {}", guess);
            continue;
        }

        gusssed_letters.push(guess);
        gusssed_letters.sort();

        // get all occurences of guessed letter in target word
        let indices_of_guess: Vec<_> = target_word.match_indices(guess).collect();

        if !target_word.contains(guess) {
            wrong_guesses+=1;
            println!("Incorrect!");
        }

        else {
            println!("Correct!");
            for v in indices_of_guess{
                user_guessed_word[v.0] = guess;
            }
        }

    }

}
