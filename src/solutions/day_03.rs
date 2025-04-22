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

    pub fn mul(&self) -> i32 {
        let mut sum = 0;
        for vals in &self.data {
            sum += vals.0 * vals.1;
        }
        return sum;
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::new("inputs/day_03_test.txt").mul(), 161)
}
