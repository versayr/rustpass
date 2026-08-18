use core::iter;
use rand::{RngExt, prelude::SliceRandom, rngs::ThreadRng, seq::IteratorRandom};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "src/eff_large_wordlist.txt";

pub fn get_random_words(amount: usize) -> Vec<String> {
    let file = File::open(FILENAME).expect("File not found");
    let file = BufReader::new(file);

    let lines = file.lines().map(|line| line.expect("Couldn't read line"));

    let mut chosen = lines.sample(&mut ThreadRng::default(), amount);
    chosen.shuffle(&mut ThreadRng::default());
    chosen
        .into_iter()
        .map(|w| get_word(&w))
        .collect::<Vec<String>>()
}

pub fn generate_password(
    words: &[String],
    padding_char: char,
    padding_count: usize,
    separator: char,
    digit_count: usize,
) -> String {
    let body: String = words.join(separator.to_string().as_str());
    let padding: String = iter::repeat_n(padding_char, padding_count).collect();
    format!(
        "{0}{3}{2}{1}{2}{4}{0}",
        padding,
        body,
        separator,
        gen_numeric(digit_count),
        gen_numeric(digit_count)
    )
}

fn get_word(line: &str) -> String {
    let (_, word) = line.rsplit_once('\t').expect("Failed to parse words list.");
    word.to_owned()
}

fn gen_numeric(length: usize) -> String {
    let mut rng = rand::rng();

    (0..length)
        .map(|_| rng.random_range(0..=9).to_string())
        .collect()
}
