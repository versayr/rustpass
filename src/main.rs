mod utils;
use utils::*;
use clap::Parser;

#[derive(Parser)]
struct Arguments {
// X padding char
// X separator char
// X # of digits
// X # of words
// X # of padding chars
// X # of passwords to generate
// . min length
// . max length
// . capitalization scheme
    /// How many words in each passphrase?
    #[arg(long, default_value_t = 4)]
    word_count: usize,
    /// Which character should pad the beginning and end of the passphrase?
    #[arg(long, default_value_t = '.')]
    padding_char: char,
    /// How many padding characters at the beginning and end of each passphrase?
    #[arg(long, default_value_t = 3)]
    padding_count: usize,
    /// Which character should separate the words in the passphrase?
    #[arg(long, default_value_t = '-')]
    separator: char,
    /// How many digits should be used at the beginning and end of each passphrase?
    #[arg(long, default_value_t = 3)]
    digit_count: usize,
    /// How many passphrases should be generated?
    #[arg(long, short, default_value_t = 3)]
    count: usize,
    /// Should output be inline or separated by newlines?
    #[arg(long, short, default_value_t = false)]
    inline: bool,
}

fn main() {
    let args : Arguments = Arguments::parse();
    let mut words = get_random_words(args.word_count * args.count);

    while !words.is_empty() {
        let next = words.split_off(words.len() - args.word_count);
        let password = generate_password(
            next, 
            args.padding_char, 
            args.padding_count, 
            args.separator, 
            args.digit_count);
        if args.inline { print!("{}", password); }
        else { println!("{}", password) }
    }
}

// Todo
// X add list of words 
// X create function to select random words 
// X filter random lines into just the words
// X shuffle list of random words
// X remove unused code
// X move functions to utils folder
// X . get random words
// X . format a passphrase
// . get arguments from user (?)
// . make file accessible in Windows systems
//
// REFACTOR
// X generate_password (reduce for loops, reduce duplicate code)
//
// X create function to format:
// X . words
// X . padding
// X . numbers
// X . separators
// X print three generated passwords
// set default parameters to XKCD suggestions
// . allow defaults to be stored locally (not in my public project)
// set up cli parser to allow user to adjust settings
// params:
// . padding char
// . separator char
// . # of digits
// . # of words
// . # of padding chars
// . min length
// . max length
// . capitalization scheme
// . # of passwords to generate
// . custom wordlists
// tests
// switch to cryptographically secure prng
// 
