use std::cmp::Ordering;

use crate::input_reader::input_reader;

pub struct Solution {
    library: RuleList,
    page_sets: Vec<Vec<i32>>
}

impl Solution {
    pub fn new(filename: &str) -> Solution {
        let raw_data = input_reader(String::from(filename));
        let split_data: Vec<&str> = raw_data.split("\n\n").collect();
        let raw_library = split_data[0];
        let raw_page_sets = split_data[1];

        let library = RuleList::from_str(raw_library);

        let mut page_sets = vec![];
        for line in raw_page_sets.split("\n") {
            if line == "" {continue;}
            page_sets.push(line.split(",").map(|num| {
                num.parse::<i32>().expect("Not a number!")
            }).collect::<Vec<i32>>())
        }
        return Solution { library, page_sets }
    }

    fn sort(&mut self) {
        // let mut new_page_sets = self.page_sets.clone();
        // for mut pages in new_page_sets {
        //     pages.sort()
        // }
        let library = self.library.clone();
        for mut pages in &mut self.page_sets {
            pages.sort_by(|a, b| {
                library.cmp(a, b)
            }
            )
        }
    }

    pub fn part_one(&self) -> i32 {
        let mut sum = 0;
        let library = self.library.clone();
        for pages in &self.page_sets {
            if library.passes_rules(pages) {
                sum += pages[((pages.len() + 1) / 2) - 1]
            }
        }
        return sum
    }

    pub fn part_two(&mut self) -> i32 {
        let mut sum = 0;
        let library = self.library.clone();
        for pages in &mut self.page_sets {
            if !library.passes_rules(pages) {
                pages.sort_by(|a, b| library.cmp(a, b));
                sum += pages[((pages.len() + 1) / 2) - 1]
            }
        }

        return sum
    }
}

#[derive(Clone, Copy)]
struct Rule {
    lesser: i32,
    greater: i32
}

impl Rule {
    fn new(lesser: i32, greater: i32) -> Rule {
        Rule {lesser, greater}
    }

    fn new_from_vec(vec: Vec<i32>) -> Rule {
        Rule {lesser: vec[0], greater: vec[1]}
    }

    fn cmp(&self, a: &i32, b: &i32) -> Ordering {
        if self.lesser == *a && self.greater == *b {
            return Ordering::Less;
        } else if self.lesser == *b && self.greater == *a {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }
}

#[derive(Clone)]
struct RuleList {
    rules: Vec<Rule>
}

impl RuleList {
    fn new() -> RuleList {
        let rules: Vec<Rule> = vec![];
        RuleList { rules }
    }

    fn from_str(string: &str) -> RuleList {
        let mut rules = vec![];
        for line in string.split("\n") {
            let nums: Vec<i32> = line.split("|").map(|str| str.parse::<i32>().unwrap()).collect();
            rules.push(Rule::new_from_vec(nums));
        }
        RuleList { rules }
    }

    fn push(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    fn passes_rules(&self, vec: &Vec<i32>) -> bool {
        let mut a = 0;
        let mut b = 1;

        while b < vec.len() {
            match self.cmp(&vec[a], &vec[b]) {
                Ordering::Less => {}
                _ => return false
            }
            a += 1;
            b += 1;
        }
        return true
    }

    fn cmp(&self, a: &i32, b: &i32) -> Ordering {
        for rule in &self.rules {
            let result = rule.cmp(a, b);
            if result == Ordering::Equal {continue}
            return result;
        }
        return Ordering::Equal
    }
}

#[test]
fn test_input() {
    assert_eq!(Solution::new("inputs/day_05_test.txt").part_one(), 143);
}

#[test]
fn test_part_two() {
    assert_eq!(Solution::new("inputs/day_05_test.txt").part_two(), 123);
}
