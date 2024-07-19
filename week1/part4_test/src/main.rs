use std::collections::HashSet;
use std::io;

fn main() {
    let secret_word = "rustacean";
    let mut guessed_letters = HashSet::new();
    let mut remaining_guesses = 6;
    let mut current_state = vec!['-'; secret_word.len()];

    println!("Welcome to Hangman!");
    
    while remaining_guesses > 0 {
        println!("\nThe word so far is: {}", current_state.iter().collect::<String>());
        println!("You have guessed the following letters: {:?}", guessed_letters);
        println!("You have {} guesses left", remaining_guesses);
        println!("Please guess a letter:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim().chars().next().expect("Please enter a letter");

        if guessed_letters.contains(&guess) {
            println!("You already guessed that letter.");
            continue;
        }

        guessed_letters.insert(guess);

        if secret_word.contains(guess) {
            for (i, c) in secret_word.chars().enumerate() {
                if c == guess {
                    current_state[i] = guess;
                }
            }
            if current_state.iter().collect::<String>() == secret_word {
                println!("Congratulations! You guessed the word: {}", secret_word);
                return;
            }
        } else {
            println!("Sorry, that letter is not in the word.");
            remaining_guesses -= 1;
        }
    }

    println!("\nGame over! The word was: {}", secret_word);
}
