pub mod input_reader;
mod solutions;
use solutions::day_01::solution::part_two as solution_fn;

fn main() {
    let solution = solution_fn("inputs/day_01.txt");
    println!("{solution:}");
}
