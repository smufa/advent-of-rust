mod day1;

#[path = "utils.rs"]
mod utils;

fn main() {
    let puzzle = utils::read("./input1");
    println!("Solution to first part: {}", puzzle.day1::first());
    println!("Solution to second part: {}", puzzle.second());
}
