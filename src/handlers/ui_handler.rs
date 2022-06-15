// Functions to handle displaying the output on the terminal

use colored::Colorize;
use std::io;
use std::io::Write;

// Displays a prompt to the user and returns their input
pub fn take_input(prompt: String) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush buffer");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_ascii_lowercase()
}

// Waits for ENTER to be pressed
pub fn pause(prompt: String) {
    let mut x = String::with_capacity(1);
    println!("\n{}", prompt);
    io::stdin().read_line(&mut x).expect("Failed to Read");
    println!("\n\n\n\n\n\n\n")
}

/*  Returns a string containing the letters of the word
    in the following colour scheme:
    - green -> letter in answer and correct position ('g')
    - yellow -> letter in answer but wrong position ('y')
    - white -> letter not in answer ('w')
    - red -> anything else (shouldnt normally be possible)
*/
pub fn get_hint(checks: &Vec<char>, word: &String) -> String {
    let mut hint = String::new();
    if word.len() != 0 {
        let letters: Vec<char> = word.chars().collect();
        for i in 0..letters.len() {
            let letter = letters[i].to_string();

            hint = format!(
                "{}{}",
                hint,
                match checks[i] {
                    'g' => letter.truecolor(0, 255, 64).bold(),
                    'y' => letter.truecolor(246, 255, 0).bold(),
                    'w' => letter.white(),
                    _ => letter.red().bold(),
                }
            );
        }
    }
    return hint;
}

// Prints the contents of a vector containing all the past
// attempts with the letters highlighted for hints
pub fn display_hints(hints: &Vec<String>) {
    println!("");
    for hint in hints {
        println!("{}", hint);
    }
}

// Returns a disclaimer for the number of tries left
pub fn get_disclaimer(i: u32, hints: &Vec<String>) -> String {
    let disclaimer = if i != 1 {
        format!("{} {}", (i).to_string().blue(), "Tries Remaining...".blue())
    } else {
        format!("{} {}", (i).to_string().red(), "Try Remaining...".blue())
    };
    if hints.len() > 0 {
        return format!("{}\n{}", "--------------------".purple(), disclaimer);
    } else {
        return disclaimer;
    }
}

// Clears the screen
pub fn clear() {
    for _ in 0..1000 {
        println!("\n");
    }
}

// Displays the end screen
pub fn show_end_screen(won: bool, answer: &String) {
    // End Screen
    if won {
        println!("\n{}", "You Won!".purple());
    } else {
        println!("\n{}", "You Lost!".red());
        println!(
            "{} {}",
            "The word was".green(),
            answer.to_ascii_uppercase().truecolor(0, 255, 64).bold()
        );
    }
}
