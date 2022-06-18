use crate::handlers::ui_handler::*;



pub fn display_menu() {
    let menu = 
        format!("{}\n{}\n{}\n{}",
                "<CHOOSE AN OPTION>", 
                "<1> Play",
                "<2> See Instructions",                
                "<3> Exit" 
    );
    let choice = loop {
        clear();
        println!("{}", menu);
        match take_input(
            "Choose an Option: "
                    .to_string())
                    .parse::<u8>() {
            Ok(1) => break 1,
            Ok(2) => break 2,
            Ok(3) => break 3,
            _ => continue
        };
    };

    match choice {
        1 => return (),
        2 => instructions(),
        3 => quit(),
        _ => panic!("How Could This Even Happen??!!!")
    };
}



fn instructions() {
    println!("THese are the instructions!!")
}

fn quit() {
    println!("QUITTING...")
}