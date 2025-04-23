extern crate regex;
pub mod input_reader;
mod solutions;

fn main() {
    println!("# Day 1");
    println!(
        "Part 1: {:}",
        solutions::day_01::solution::day_01_solution("inputs/day_01.txt")
    );
    println!(
        "Part 2: {:}",
        solutions::day_01::solution::part_two("inputs/day_01.txt")
    );
    println!("\n# Day 2");
    let day_02 = solutions::day_02::Day02Solution::new("inputs/day_02.txt");
    println!("Part 1: {:?}", day_02.part_one());
    println!("Part 2: {:?}", day_02.part_two());

    println!("\n# Day 3");
    let day_03 = solutions::day_03::Solution::new("inputs/day_03.txt");
    println!("Part 1: {:?}", day_03.mul());
    let day_03_pt_2 = solutions::day_03::Solution::new_with_do("inputs/day_03.txt");
    println!("Part 2: {:?}", day_03_pt_2.mul());

    println!("\n# Day 4");
    let day_04 = solutions::day_04::Solution::new("inputs/day_04.txt");
    println!("Part 1: {:?}", day_04.part_one());
    println!("Part 2: {:?}", day_04.part_two());

    println!("\n# Day 5");
    let mut day_05 = solutions::day_05::Solution::new("inputs/day_05.txt");
    println!("Part 1: {:?}", day_05.part_one());
    println!("Part 2: {:}", day_05.part_two());
}
