// Functions to handle checking user input with answer

use crate::config::*;

// Converts word into an char vector
fn get_letters(word: &String) -> Vec<char> {
    let letters: Vec<char> = word.chars().collect();
    letters
}

/*  Compares answer and a word and returns a char vector containing
    - g -> letter in answer and correct position
    - y -> letter in answer but wrong position
    - w -> letter not in answer
*/
pub fn check_word(answer: &String, word: &String) -> Vec<char> {
    let mut ans_letters = get_letters(answer);
    let mut word_letters = get_letters(word);
    let mut check_vec: Vec<char> = Vec::new();

    // Marks the letters (green, yellow, white) in a vector
    // and returns the vector
    mark_correct(&mut ans_letters, &mut word_letters, &mut check_vec);
    mark_incorrect(&mut ans_letters, &mut word_letters, &mut check_vec);
    check_vec
}

/*  Checks for characters which are in the correct
    position and removes them after marking the position
    as correct in the check_vec vector
*/
pub fn mark_correct(
    ans_letters: &mut Vec<char>,
    word_letters: &mut Vec<char>,
    check_vec: &mut Vec<char>,
) {
    let length = WORD_LENGTH;
    let mut check: char;
    for i in 0..length {
        if ans_letters[i] == word_letters[i] {
            ans_letters[i] = ' ';
            word_letters[i] = ' ';
            check = 'g';
        } else {
            check = ' ';
        }
        check_vec.push(check);
    }
}

/*  Checks for every character in the input word which
    also present in the answer and removes them both after
    marking the position as wrong position but correct letter
    in the check_vec vector
*/
pub fn mark_incorrect(
    ans_letters: &mut Vec<char>,
    word_letters: &mut Vec<char>,
    check_vec: &mut Vec<char>,
) {
    let length = WORD_LENGTH;

    for i in 0..length {
        let mut flag = false;

        if check_vec[i] == ' ' {
            for _ in 0..length {
                let letter = word_letters[i];
                if ans_letters.contains(&letter) {
                    // Gets position of the letter in the answer
                    let pos = ans_letters
                        .iter()
                        .position(|&x| x == letter)
                        .expect("Failed to find letter in answer");

                    flag = true;
                    check_vec[i] = 'y';
                    word_letters[i] = ' ';
                    ans_letters[pos] = ' ';
                    break;
                }
            }
            if flag == false {
                check_vec[i] = 'w';
            }
        }
    }
}
