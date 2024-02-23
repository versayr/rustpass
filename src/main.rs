mod utils;
use utils::*;
use clap::Parser;

#[derive(Parser)]
struct Arguments {
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

