use colored::Colorize;

mod config;
use config::{MAX_TRIES, WORD_LENGTH};

mod handlers {
    pub mod ui_handler;
    pub mod word_handler;
}
use handlers::ui_handler::*;
use handlers::word_handler::*;

fn main() {
    clear();
    let answer = "eagle".to_string();
    let mut hints: Vec<String> = Vec::new();
    let mut won = false;

    for i in 0..MAX_TRIES {
        // Displays the disclaimer and user prompt
        let disclaimer = get_disclaimer(MAX_TRIES - i, &hints);
        println!("{}", disclaimer);
        let mut user_input = take_input(format!("{}", "Guess the word: ".purple()));
        clear();

        // Keeps looping until a valid word is entered
        while user_input.len() != WORD_LENGTH {
            println!(
                "{} {} {}",
                "Only words with".blue(),
                WORD_LENGTH.to_string().blue(),
                "letters are accepted!".blue()
            );
            display_hints(&hints);
            println!("{}", disclaimer);
            user_input = take_input(format!("{}", "Guess the word: ".purple()));
            clear();
        }

        // Compares word and actual answer, and then displays hints
        let checks = check_word(&answer, &user_input);
        hints.push(get_hint(&checks, &user_input.to_ascii_uppercase()));
        display_hints(&hints);

        if checks == vec!['g'; WORD_LENGTH] {
            won = true;
            break;
        }
    }
    show_end_screen(won, &answer);
    pause(format!(
        "{} {} {}",
        "Press".blue(),
        "ENTER".blue().bold(),
        "to Continue...".blue()
    ));
}
