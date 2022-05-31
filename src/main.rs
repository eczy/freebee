use itertools::Itertools;
use rand::prelude::*;
use std::fs::File;
use std::io;
use std::io::*;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::collections::HashMap;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

static RULES: &str = r"
RULES:

1. Words must include the center letter.
2. Words must contain at least four letters.
3. Letters can be used more than once.
4. Our word list does not include words that are offensive, obscure, hyphenated or proper nouns.
5. Four-letter words are worth one point each.
6. Longer words earn one point per letter. A six-letter word is worth six points.
7. Each puzzle includes at least one “pangram,” which uses every letter at least once. A pangram is worth an additional seven points.
";

type Grid = HashMap<char, Vec<u8>>;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    wordlist: PathBuf,
}

fn main() {
    let mut wordlist = Wordlist::from_file("./wordlist.10000");
    let game = wordlist.gen_game();
    println!("{:#?}", game);
    let mut input = String::new();

    println!("{}, {}", &game.center_letter, &game.radial_letters.iter().collect::<String>());

    loop {
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input).expect("Error: unable to read input");
        let trimmed = input.trim();
        match trimmed {
            r"\rules" => println!("{}", RULES),
            r"\grid" => print_grid(&game.grid()),
            r"\quit" => break,
            "" => {},
            _ => println!("Try again."),
        }
        input = String::from("");
    }
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

    print!("{:width$}", "", width=width);
    for i in 0..count_sums.len() {
        print!("{:width$}", i + 4, width=width);
    }
    print!("{:>width$}", 'Σ', width=width);
    print!("\n");

    for (letter, counts) in grid {
        print!("{:width$}", letter, width=width);
        for i in 0..count_sums.len() {
            if i < counts.len() && counts[i] > 0 {
                print!("{:width$}", counts[i], width=width);
            } else {
                print!("{:>width$}", '-', width=width)
            }
        }
        print!("{:width$}", counts.iter().sum::<u8>(), width=width);
        print!("\n")
    }

    print!("{:<width$}", 'Σ', width=width);
    for count in &count_sums {
        print!("{:width$}", count, width=width);
    }
    print!("{:width$}", count_sums.iter().sum::<u8>(), width=width);
    print!("\n");
}

#[derive(Default, Debug)]
struct Wordlist {
    pub words: Vec<String>,
    pub seven_letter_words: Vec<usize>,

    rng: ThreadRng,
}

impl Wordlist {
    pub fn from_file(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut words = Vec::<String>::new();
        let mut seven_letter_words = Vec::<usize>::new();
        for line in reader.lines() {
            let line = line.unwrap();
            if line.chars().any(|c| !ASCII_LOWER.contains(&c)) {
                continue;
            }

            if line.len() < 4 {
                continue;
            }
            if line.chars().unique().collect::<Vec<char>>().len() == 7 {
                seven_letter_words.push(words.len())
            }
            words.push(line);
        }

        let rng = thread_rng();
        Self {
            words,
            seven_letter_words,
            rng,
        }
    }

    pub fn gen_game(&mut self) -> Game {
        let panagram_index = self.seven_letter_words[..].choose(&mut self.rng).unwrap();
        let panagram = self.words[*panagram_index].to_owned();
        let center_letter = panagram.chars().choose(&mut self.rng).unwrap();
        let mut radial_letters: Vec<char> = Vec::new();
        for c in panagram.chars() {
            if c != center_letter {
                radial_letters.push(c)
            }
        }
        radial_letters = radial_letters.into_iter().unique().collect();
        radial_letters.shuffle(&mut self.rng);
        let mut solutions: Vec<String> = Vec::new();
        for word in self.words.iter() {
            if word.chars().all(|c| radial_letters.contains(&c)) {
                solutions.push(word.to_owned())
            }
        }
        Game {
            center_letter: center_letter,
            radial_letters: radial_letters,
            solutions: solutions,
            panagram: panagram,
        }
    }
}

#[derive(Default, Debug)]
struct Game {
    pub center_letter: char,
    pub radial_letters: Vec<char>,
    pub solutions: Vec<String>,
    pub panagram: String,
}

impl Game {
    pub fn grid(&self) -> Grid {
        let mut grid = Grid::new();
        for letter in self.panagram.chars() {
            grid.insert(letter, Vec::new());
        }

        for word in &self.solutions {
            let first = word.chars().nth(0).unwrap();
            let counts = grid.entry(first).or_insert(vec![0;word.len() - 4]);
            if word.len() - 3 > counts.len() {
                counts.resize(word.len() - 3, 0)
            }

            *counts.get_mut(word.len() - 4).unwrap() += 1;
        }
        grid
    }
}

// RULES
// =====
// Words must include the center letter.

// Words must contain at least four letters.

// Letters can be used more than once.

// Our word list does not include words that are offensive, obscure, hyphenated or proper nouns.

// Four-letter words are worth one point each.

// Longer words earn one point per letter. A six-letter word is worth six points.

// Each puzzle includes at least one “pangram,” which uses every letter at least once. A pangram is worth an additional seven points.
