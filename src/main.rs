use itertools::Itertools;
use rand::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    wordlist: PathBuf,
}

fn main() {
    let mut wordlist = Wordlist::from_file("./wordlist.10000");
    let game = wordlist.gen_game();
    println!("{:#?}", game)
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

            if line.len() < 3 || line.len() > 7 {
                continue;
            }
            if line.len() == 7 && line.chars().unique().collect::<Vec<char>>().len() == 7 {
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
    center_letter: char,
    radial_letters: Vec<char>,
    solutions: Vec<String>,
    panagram: String,
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
