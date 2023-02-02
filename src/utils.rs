use std::fs::File;
use std::io::{self, BufRead};

pub struct Puzzle {
    pub input: Vec<String>,
}

pub trait IsPuzzle {
    fn first(&self) -> String;
    fn second(&self) -> String;
}

/// Reads the puzzle input from `path` and returns a puzzle object
pub fn read(path: &str) -> Puzzle {
    return Puzzle {
        input: io::BufReader::new(File::open(path.to_string()).unwrap())
            .lines()
            .map(|l| l.unwrap())
            .collect(),
    };
}
