use crate::input_reader::input_reader;

/// Input consists of 'reports' (a line of numbers) that consists of 'levels' that
/// are seperated by a space. The solution must figure out what reports are 'safe'.
///
/// A safe report is defined by:
/// - the levels are either *all increasing* or *all decreasing*.
/// - Any two adjacent levels differ by *at least one* and *at most three*
///
/// For this data structure, we're looking at a 2D 'Vector' of 'i32's. The first
/// thing to do is read the file into memory.
///
/// We now can make x number of functions that test if a report is safe/unsafe
///
pub struct Day02Solution {
    reports: Vec<Vec<i32>>,
}

impl Day02Solution {
    pub fn new(filename: &str) -> Day02Solution {
        let mut reports: Vec<Vec<i32>> = vec![];
        let data = input_reader(String::from(filename));
        for report_data in data.split("\n") {
            if report_data == "" {
                continue;
            }
            let mut report: Vec<i32> = vec![];
            for level in report_data.split(" ").collect::<Vec<&str>>() {
                report.push(level.parse().expect("{level} is not an integer!"))
            }
            reports.push(report);
        }
        return Day02Solution { reports };
    }

    pub fn part_one(&self) -> i32 {
        let mut num = 0;
        for report in &self.reports {
            if Day02Solution::mono_directional_test(report)
                && Day02Solution::number_of_steps_test(report)
            {
                num += 1;
            }
        }
        return num;
    }

    pub fn part_two(&self) -> i32 {
        let mut num = 0;
        for report in &self.reports {
            if Day02Solution::mono_directional_test(report)
                && Day02Solution::number_of_steps_test(report)
            {
                num += 1;
            } else if Day02Solution::test_single_bad_level(report) {
                num += 1
            }
        }
        return num;
    }

    fn mono_directional_test(report: &Vec<i32>) -> bool {
        let mut i = 2;
        let positive = report[1] - report[0] > 0;
        while i < report.len() {
            let difference = (report[i] - report[i - 1]) > 0;
            if difference != positive {
                return false;
            }
            i += 1;
        }
        return true;
    }

    fn number_of_steps_test(report: &Vec<i32>) -> bool {
        let mut i = 1;
        while i < report.len() {
            let difference = (report[i - 1] - report[i]).abs();
            if difference == 0 || difference > 3 {
                return false;
            }
            i += 1;
        }
        return true;
    }

    fn test_single_bad_level(report: &Vec<i32>) -> bool {
        let mut i = 0;
        while i < report.len() {
            let mut edited_report = report.clone();
            edited_report.remove(i);
            if Day02Solution::mono_directional_test(&edited_report)
                && Day02Solution::number_of_steps_test(&edited_report)
            {
                return true;
            }
            i += 1;
        }
        return false;
    }
}

#[test]
fn test_new() {
    Day02Solution::new("inputs/day_02_test.txt");
}

#[test]
fn test_input() {
    assert_eq!(Day02Solution::new("inputs/day_02_test.txt").part_one(), 2)
}

#[test]
fn test_real_input() {
    assert_eq!(Day02Solution::new("inputs/day_02.txt").part_one(), 383)
}

#[test]
fn test_mono() {
    let test_01 = vec![62, 65, 67, 70, 73, 76, 75];
    let test_02 = vec![77, 80, 81, 82, 86];
    assert_eq!(Day02Solution::mono_directional_test(&test_01), false);
    assert_eq!(Day02Solution::mono_directional_test(&test_02), true);
}

#[test]
fn day_two_test_input() {
    assert_eq!(Day02Solution::new("inputs/day_02_test.txt").part_two(), 4)
}
