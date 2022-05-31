use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::{rngs::StdRng};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub type Grid = HashMap<char, Vec<u8>>;

pub struct Wordlist {
    pub words: Vec<String>,
    pub seven_letter_words: Vec<usize>,
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

        Self {
            words,
            seven_letter_words,
        }
    }

    pub fn gen_game(&mut self, seed: Option<u64>) -> Game {
        let mut rng;
        match seed {
            Some(inner) => rng = StdRng::seed_from_u64(inner),
            None => rng = StdRng::from_entropy(),
        }

        let panagram_index = self.seven_letter_words[..].choose(&mut rng).unwrap();
        let panagram = self.words[*panagram_index].to_owned();
        let center_letter = *panagram.as_bytes().choose(&mut rng).unwrap() as char;
        let mut radial_letters: Vec<char> = Vec::new();
        for c in panagram.chars() {
            if c != center_letter {
                radial_letters.push(c)
            }
        }
        radial_letters = radial_letters.into_iter().unique().collect();
        radial_letters.shuffle(&mut rng);
        let mut solutions: Vec<String> = vec![panagram];
        for word in self.words.iter() {
            if !word.chars().contains(&center_letter) {
                continue;
            }
            if word
                .chars()
                .all(|c| radial_letters.contains(&c) || center_letter == c)
            {
                solutions.push(word.to_owned())
            }
        }
        Game {
            center_letter: center_letter,
            radial_letters: radial_letters,
            solutions: solutions,
        }
    }
}

#[derive(Default, Debug)]
pub struct Game {
    pub center_letter: char,
    pub radial_letters: Vec<char>,
    pub solutions: Vec<String>,
}

impl Game {
    pub fn grid(&self) -> Grid {
        let mut grid = Grid::new();
        for letter in &self.radial_letters {
            grid.insert(*letter, Vec::new());
        }
        grid.insert(self.center_letter, Vec::new());

        for word in &self.solutions {
            let first = word.chars().nth(0).unwrap();
            let counts = grid.entry(first).or_insert(vec![0; word.len() - 4]);
            if word.len() - 3 > counts.len() {
                counts.resize(word.len() - 3, 0)
            }

            *counts.get_mut(word.len() - 4).unwrap() += 1;
        }
        grid
    }
}
