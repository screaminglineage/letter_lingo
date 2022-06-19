use crate::handlers::ui_handler::*;
use colored::Colorize;



pub fn display_menu() {
    let menu = 
        format!("{}\n{}\n{}\n{}",
                "<CHOOSE AN OPTION>", 
                "<1> Play",
                "<2> See Instructions",                
                "<3> Exit" 
    );
    loop {
        clear();
        println!("{}", menu);
        match take_input(
            "Choose an Option: "
                    .to_string())
                    .parse::<u8>() {
            Ok(1) => break,
            Ok(2) => {instructions(); continue},
            Ok(3) => std::process::exit(0),
            _ => continue
        };
    };
}



fn instructions() {
    clear();
    println!("{}", 
            "-- -- -- -- -- -- -- -- -- INSTRUCTIONS -- -- -- -- -- -- -- -- -- --"
                .cyan().bold());
    println!("\n{}\n\n{}\n{}\n\n{}\n{}\n{}\n\n{}\n",
             "You have 6 tries to guess a 5 letter word"
                .purple(),
             "As the words are entered, you get more clues \
             about the actual word"
                .purple(),
             "by the colour of the entered letters"
                .purple(),
             "White -> Letter Not Present in Word"
                .bold(),
             "Yellow -> Letter Present in Word But not in Correct Position"
                .truecolor(246, 255, 0).bold(),
             "Green -> Letter Present in Word And in Correct Position"
                .truecolor(0, 255, 64).bold(),
             "See if you can guess the word!"
                .purple()

    );
    pause("Press ENTER To Continue".blue().to_string());
}

