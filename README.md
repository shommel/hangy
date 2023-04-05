# Hangy

## Summary
This is a very novel implementation of a command-line Hangman game written in Rust. This purely is and made purposely for educational purposes as I learn more about the Rust language.

### Features
- Tracks guessed letters (avoids duplicate letters)
- Case-agnostic answers (defaults answers to lowercase)
- Error handling around user input
- ASCII Hangman art


### Build
```zsh
cargo build
```

### Running
```zsh
cargo run
```

### Linting
_NOTE: using default Rust linter_
```zsh
cargo clippy
```

#### ASCII Art
![Partially Hanged](pics/ascii1.png?raw=true)

![Fully Hanged](pics/ascii2.png?raw=true)