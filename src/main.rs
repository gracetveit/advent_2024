mod solutions;
pub mod input_reader;

fn main() {
    let solution = solutions::day_01::solution::sol(
        "inputs/day_01.txt"
    );
    println!("{solution:}");
}
