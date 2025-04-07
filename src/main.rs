pub mod input_reader;
mod solutions;

fn main() {
    println!("# Day 1\n");
    println!(
        "Part 1: {:}",
        solutions::day_01::solution::day_01_solution("inputs/day_01.txt")
    );
    println!(
        "Part 2: {:}",
        solutions::day_01::solution::part_two("inputs/day_01.txt")
    );
}
