mod utils;
use clap::Parser;
use utils::{generate_password, get_random_words};

#[derive(Parser)]
struct Arguments {
    /// How many words in each passphrase?
    #[arg(long, default_value_t = 3)]
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
    let args: Arguments = Arguments::parse();
    let amount = args
        .word_count
        .checked_mul(args.count)
        .expect("Too many passwords with too many words requested - count has overflowed.");
    let mut words = get_random_words(amount);

    while !words.is_empty() {
        let split = words
            .len()
            .checked_sub(args.word_count)
            .expect("Too many passwords requested - words list has run out.");
        let next = words.split_off(split);
        let password = generate_password(
            &next,
            args.padding_char,
            args.padding_count,
            args.separator,
            args.digit_count,
        );
        if args.inline {
            print!("{password}");
        } else {
            println!("{password}");
        }
    }
}
