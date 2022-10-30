const TITLE: &str = " ███████████      ███████    ███████████   ██████████   █████       ██████████
░░███░░░░░███   ███░░░░░███ ░░███░░░░░███ ░░███░░░░███ ░░███       ░░███░░░░░█
 ░███    ░███  ███     ░░███ ░███    ░███  ░███   ░░███ ░███        ░███  █ ░ 
 ░██████████  ░███      ░███ ░██████████   ░███    ░███ ░███        ░██████   
 ░███░░░░░███ ░███      ░███ ░███░░░░░███  ░███    ░███ ░███        ░███░░█   
 ░███    ░███ ░░███     ███  ░███    ░███  ░███    ███  ░███      █ ░███ ░   █
 █████   █████ ░░░███████░   █████   █████ ██████████   ███████████ ██████████
░░░░░   ░░░░░    ░░░░░░░    ░░░░░   ░░░░░ ░░░░░░░░░░   ░░░░░░░░░░░ ░░░░░░░░░░ 
                                                                              ";

const SUB_TITLE: &str = "A simple WORDLE clone";
const DESC: &str = "Guess the secret word in 5 guesses";
const INITIAL_DISPLAY: &str = "░ ░ ░ ░ ░\n";
fn main() {
    println!("{}", TITLE);
    println!("{}", SUB_TITLE);
    println!("{}", DESC);
    println!("{}", INITIAL_DISPLAY);

    // TODO: Get word from randomly from file
    let secret_word = String::from("penny");
    let mut guesses = 0;
    let secret_as_chars: Vec<char> = secret_word.chars().collect();

    loop {
        if guesses >= 5 {
            println!("You lose!");
            return
        }

        let mut loop_count = 0;
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
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

        println!("{}\n", response_display);
        if response == secret_word {
            println!("\nGreate success! You have brought honor to this dojo.");
            break;
        } else if response != secret_word && guesses < 5{
            println!("\nNot quite, You have {} more tries. Try again.\n", 5 - guesses);
        }
    }
}
