use rand::Rng;
use std::io;

const TITLE: &str = " ███████████      ███████    ███████████   ██████████   █████       ██████████
░░███░░░░░███   ███░░░░░███ ░░███░░░░░███ ░░███░░░░███ ░░███       ░░███░░░░░█
 ░███    ░███  ███     ░░███ ░███    ░███  ░███   ░░███ ░███        ░███  █ ░ 
 ░██████████  ░███      ░███ ░██████████   ░███    ░███ ░███        ░██████   
 ░███░░░░░███ ░███      ░███ ░███░░░░░███  ░███    ░███ ░███        ░███░░█   
 ░███    ░███ ░░███     ███  ░███    ░███  ░███    ███  ░███      █ ░███ ░   █
 █████   █████ ░░░███████░   █████   █████ ██████████   ███████████ ██████████
░░░░░   ░░░░░    ░░░░░░░    ░░░░░   ░░░░░ ░░░░░░░░░░   ░░░░░░░░░░░ ░░░░░░░░░░ 
                                                                              ";

const SUB_TITLE: &str = "A simple WORDLE clone\n\n";
const DESC: &str = "Guess the secret word in 5 guesses\n";
const INITIAL_DISPLAY: &str = "░ ░ ░ ░ ░\n";
fn main() {
    println!("{}", TITLE);
    println!("{}", SUB_TITLE);
    println!("{}", DESC);
    println!("{}", INITIAL_DISPLAY);

    let contents: String = std::fs::read_to_string("words.txt").unwrap();
    let words: Vec<&str>= contents.lines().collect();
    let idx :usize = rand::thread_rng().gen_range(0..49);
    let secret_word: String = String::from(words[idx]);
    let mut guesses: i32 = 0;
    let secret_as_chars: Vec<char> = secret_word.chars().collect();

    loop {
        if guesses >= 5 {
            println!("\nYou lose!");
            println!("The secret word was: {secret_word}");
            return
        }

        let mut loop_count:usize = 0;
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let mut response: String = String::new();
        let mut response_display: String = String::new();

        guesses = guesses + 1;

        for letter in guess.chars() {
            if letter == '\n' {
                break
            }
            if letter == secret_as_chars[loop_count] {
                response.push(letter);
                response_display.push(letter);
            } else {
                response.push('_');
                response_display.push('░');
            }
            response_display.push(' ');
            if loop_count < 4 {
                loop_count = loop_count + 1;
            }
        }

        println!("\n{response_display}");
        if response == secret_word {
            println!("\nGreate success! You have brought honor to this dojo.");
            break;
        } else if response != secret_word && guesses < 5{
            println!("\nNot quite.");
            println!("You have {} more tries\n", 5 - guesses);
        }
    }
}
