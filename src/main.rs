use std::io;
extern crate reqwest;

fn main() {

    welcome_message();  
    let target_word: String = strip_first_and_last(get_word());
    let mut lives: u8 = 15;
    
    let mut hidden_word: Vec<char> = Vec::new();
    let mut guess: Vec<char> = Vec::new();
    let mut wrong_guess: Vec<char> = Vec::new();
  
    for l in target_word.chars() {
        hidden_word.push(l);
        match l {
            ' ' => guess.push(' '),
            _ => guess.push('_')
        }  
    }  
    
    loop {
        
        if lives == 0 {
            println!("No more lives, the word was '{}' :-(\n", target_word);
            break
        } else if &hidden_word == &guess {
            println!("The word was '{}', you win!\n", target_word);
            break
        }
    
        println!("Guess a letter! (lives left: {})", lives);
        println!("{:?}", guess);
        
        let mut player_guess: String = String::new();
        io::stdin().read_line(&mut player_guess).expect("Failed to read line");
        
        let player_guess: char = match player_guess.trim().parse() {
            Ok(letter) => letter,
            Err(_) => {
                println!("That is NOT a letter!");
                continue
            },
        };
        
        if hidden_word.contains(&player_guess) {
            println!("{} is in the word!\n", player_guess);
            for i in 0..hidden_word.len() {
                if hidden_word[i] == player_guess {
                    guess[i] = player_guess;
                }
            }
            continue
        } else if wrong_guess.contains(&player_guess) {
            println!("You already tried that, no lives deducted!\n");
            continue
        }
        
        wrong_guess.push(player_guess);
        println!("{} is not in the word!\n", player_guess);
        lives = lives - 1;
    }
}

fn welcome_message() -> () {
    println!("\nWelcome to hangman!\n")
}

fn strip_first_and_last(input: String) -> String {
    
    /* This function is silly and should not be 
    needed, todo: find a better way to remove 
    quotation marks around JSON return word */
    
    let first_last_off: &str = &input[1..input.len() - 1];
    String::from(first_last_off)
}

fn get_word() -> String {
    
    /* Retrieve a random word from online API.
    This should be made, such that the code does
    not panick on errors. Match the Result enum 
    variants instead*/
    
    let data = reqwest::blocking::get("https://randomword-gen.herokuapp.com/api/six")
        .expect("Could not execute request.")
        .text()
        .expect("Could not parse response body.");
        return data;
}
