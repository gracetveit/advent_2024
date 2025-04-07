pub mod input_reader;
mod solutions;
use solutions::day_01::solution::day_01_solution;

fn main() {
    let solution = day_01_solution("inputs/day_01.txt");
    println!("{solution:}");
}
