// Functions to get a list of words and randomly choose one

use std::fs::File;
use std::io::{prelude::*, BufReader};
use rand::Rng;


pub fn get_word_file(file: &str) -> Vec<String> {
    let file =  File::open(file).expect("File Not Found");
    let buffer = BufReader::new(file);
    buffer
        .lines()
        .map(|l| l.expect("Failed to Parse Line"))
        .collect()
}


pub fn get_word(words: &Vec<String>) -> &String {
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..words.len());
    &words[i]
}