use crate::input_reader::input_reader;
use regex::Regex;

pub struct Solution {
    data: Vec<(i32, i32)>,
}

impl Solution {
    pub fn new(filename: &str) -> Solution {
        let re = Regex::new(r"mul\((?<x>\d+),(?<y>\d+)\)").expect("Invalid Regex");
        let data = input_reader(String::from(filename));
        let captures = re.captures_iter(data.as_str());
        let parsed_data: Vec<(i32, i32)> = captures
            .map(|caps| {
                let x = caps.name("x").unwrap().as_str().parse::<i32>().unwrap();
                let y = caps.name("y").unwrap().as_str().parse::<i32>().unwrap();

                (x, y)
            })
            .collect();
        return Solution { data: parsed_data };
    }

    pub fn new_with_do(filename: &str) -> Solution {
        let mut data = vec![];
        let re = Regex::new(r"(?<mull>mul\((?<x>\d+),(?<y>\d+)\))|(?<not>don't\(\))|(?<do>do\(\))")
            .expect("Invalid Regex");
        let unparsed_data = input_reader(String::from(filename));
        let mut do_mode = true;
        for captures in re.find_iter(&unparsed_data.as_str()).map(|m| m.as_str()) {
            match do_mode {
                true => match Solution::parse_instruction(captures) {
                    Instruction::DoNot => {
                        do_mode = false;
                    }
                    Instruction::Mul(x, y) => data.push((x, y)),
                    _ => {}
                },
                false => match Solution::parse_instruction(captures) {
                    Instruction::Do => {
                        do_mode = true;
                    }
                    _ => {}
                },
            }
            // println!("{captures:#?}")
        }
        return Solution { data };
    }
    fn parse_instruction(instruction_string: &str) -> Instruction {
        match instruction_string {
            "do()" => return Instruction::Do,
            "don't()" => return Instruction::DoNot,
            _ => {
                let vals = Solution::parse_mul(instruction_string);
                return Instruction::Mul(vals.0, vals.1);
            }
        }
    }

    pub fn parse_mul(mul: &str) -> (i32, i32) {
        let mut vals: Vec<i32> = vec![];
        let re = Regex::new(r"\d+").expect("Invalid Regex");
        for x in re.find_iter(mul) {
            vals.push(x.as_str().parse().expect("Not a number!"));
        }
        (vals[0], vals[1])
    }

    pub fn mul(&self) -> i32 {
        let mut sum = 0;
        for vals in &self.data {
            sum += vals.0 * vals.1;
        }
        return sum;
    }
}

enum Instruction {
    Do,
    DoNot,
    Mul(i32, i32),
}

#[test]
fn test_solution() {
    assert_eq!(Solution::new("inputs/day_03_test.txt").mul(), 161)
}

#[test]
fn test_part_two() {
    assert_eq!(Solution::new_with_do("inputs/day_03_test_02.txt").mul(), 48)
}
