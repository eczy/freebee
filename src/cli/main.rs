use clap::Parser;
use freebee::{Grid, Wordlist};
use itertools::Itertools;
use std::collections::HashMap;
use std::io;
use std::io::*;
use std::path::PathBuf;

static RULES: &str = r"
Try to guess as many words as you can from the provided letters!

RULES:

1. Words must include the center letter.
2. Words must contain at least four letters.
3. Letters can be used more than once.
4. Our word list does not include words that are offensive, obscure, hyphenated or proper nouns.
5. Four-letter words are worth one point each.
6. Longer words earn one point per letter. A six-letter word is worth six points.
7. Each puzzle includes at least one “pangram,” which uses every letter at least once. A pangram is worth an additional seven points.
";

static HELP: &str = r"
HELP:

?      - display this message
\rules - display the rules of the game
\grid  - display a matrix letter x length for every word in the puzzle
\2ll   - (two letter list) display a count for the number of words starting with each two letter pair
\solve - print all solutions to the puzzle
\quit  - exit the game
";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to wordlist.
    #[clap(short, long)]
    wordlist: PathBuf,

    /// Seed for RNG. If none provided, seeds from entropy.
    #[clap(short, long)]
    seed: Option<u64>,
}

fn main() {
    let args = Args::parse();

    let mut wordlist = Wordlist::from_file(args.wordlist.to_str().unwrap());
    let game = wordlist.gen_game(args.seed);
    let mut input = String::new();

    let mut score = 0;
    let mut correct_guesses: Vec<String> = vec![];
    loop {
        println!("\nFound:");
        for guess in &correct_guesses {
            print!("{} ", guess);
        }
        println!("\n");
        println!("Score: {}", score);
        println!(
            "{}, {}",
            &game.center_letter,
            &game.radial_letters.iter().collect::<String>()
        );
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("Error: unable to read input");
        let trimmed = input.trim();
        match trimmed {
            r"\rules" => println!("{}", RULES),
            r"\grid" => print_grid(&game.grid()),
            r"\2ll" => print_two_letter_list(&game.solutions),
            r"\solve" => {
                println!("\nSolution:");
                for sol in &game.solutions {
                    print!("{} ", sol);
                }
                println!("\n");
            }
            r"?" => println!("{}", HELP),
            r"\quit" => break,
            "" => {}
            _ => {
                if game.solutions.contains(&trimmed.to_string()) {
                    correct_guesses.push(trimmed.to_string());
                    if game
                        .radial_letters
                        .iter()
                        .all(|e| trimmed.chars().contains(&e))
                        && trimmed.chars().contains(&game.center_letter)
                    {
                        score += 7
                    }
                    score += trimmed.len();
                } else {
                    println!("Try again.")
                }
            }
        }
        input = String::from("");
    }
}

fn print_two_letter_list(solutions: &Vec<String>) {
    let mut two_letter_list: HashMap<String, u8> = HashMap::new();
    for word in solutions {
        let first_two = word[0..2].to_string();
        two_letter_list
            .entry(first_two)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    let mut prev: char = ' ';
    for letters in two_letter_list.keys().sorted() {
        let first = letters.as_bytes()[0] as char;
        if first != prev {
            println!();
        }
        print!("{}: {}  ", letters, two_letter_list[letters]);
        prev = first;
    }
    println!();
}

fn print_grid(grid: &Grid) {
    let width = 4;
    let mut count_sums: Vec<u8> = Vec::new();
    for (_, counts) in grid {
        if counts.len() > count_sums.len() {
            count_sums.resize(counts.len(), 0);
        }
        for (i, count) in counts.iter().enumerate() {
            *count_sums.get_mut(i).unwrap() += count;
        }
    }

    print!("{:w$}", "", w = width);
    for i in 0..count_sums.len() {
        print!("{:w$}", i + 4, w = width);
    }
    print!("{:>w$}\n", 'Σ', w = width);

    for (letter, counts) in grid {
        print!("{:w$}", letter, w = width);
        for i in 0..count_sums.len() {
            if i < counts.len() && counts[i] > 0 {
                print!("{:w$}", counts[i], w = width);
            } else {
                print!("{:>w$}", '-', w = width)
            }
        }
        print!("{:w$}\n", counts.iter().sum::<u8>(), w = width);
    }

    print!("{:<w$}", 'Σ', w = width);
    for count in &count_sums {
        print!("{:w$}", count, w = width);
    }
    print!("{:w$}\n", count_sums.iter().sum::<u8>(), w = width);
}
