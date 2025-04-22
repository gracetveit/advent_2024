use crate::input_reader::input_reader;

pub struct Solution {
    word_search: Vec<Vec<char>>,
}

impl Solution {
    pub fn new(filename: &str) -> Solution {
        let mut word_search = vec![];
        let raw_data = input_reader(String::from(filename));
        for line in raw_data.split("\n") {
            let mut line_vec = vec![];
            for char in line.chars() {
                line_vec.push(char);
            }
            word_search.push(line_vec);
        }
        word_search.pop();
        return Solution { word_search };
    }

    pub fn part_one(&self) -> u32 {
        let mut pos = vec![0, 0];
        let mut sum = 0;
        while pos[0] < self.word_search.len() {
            while pos[1] < self.word_search[0].len() {
                if self.word_search[pos[0]][pos[1]] == 'X' {
                    for dir in [1, 2, 3, 4, 6, 7, 8, 9] {
                        if self.direct_line_search(&pos, dir, 'X') {
                            sum += 1
                        }
                    }
                }
                pos[1] += 1;
            }
            pos[0] += 1;
            pos[1] = 0;
        }

        return sum;
    }

    pub fn part_two(&self) -> u32 {
        let mut pos = vec![0, 0];
        let mut sum = 0;
        while pos[0] < self.word_search.len() {
            while pos[1] < self.word_search[0].len() {
                if self.word_search[pos[0]][pos[1]] == 'A' {
                    if self.valid_cross(&pos) {
                        sum += 1
                    }
                }
                pos[1] += 1
            }
            pos[0] += 1;
            pos[1] = 0;
        }

        return sum;
    }

    fn valid_cross(&self, pos: &Vec<usize>) -> bool {
        let upper_left = self.new_pos(pos, 7);
        let upper_right = self.new_pos(pos, 9);
        let lower_left = self.new_pos(pos, 1);
        let lower_right = self.new_pos(pos, 3);
        match (upper_left, upper_right, lower_left, lower_right) {
            (Some(valid_ul), Some(valid_ur), Some(valid_ll), Some(valid_lr)) => {
                let ulchar = self.word_search[valid_ul[0]][valid_ul[1]];
                let urchar = self.word_search[valid_ur[0]][valid_ur[1]];
                let llchar = self.word_search[valid_ll[0]][valid_ll[1]];
                let lrchar = self.word_search[valid_lr[0]][valid_lr[1]];
                return Solution::mirror_check(ulchar, lrchar)
                    && Solution::mirror_check(urchar, llchar);
            }
            (_, _, _, _) => return false,
        }
    }

    fn mirror_check(first_char: char, second_char: char) -> bool {
        match (first_char, second_char) {
            ('S', 'M') => return true,
            ('M', 'S') => return true,
            (_, _) => return false,
        }
    }

    fn direct_line_search(&self, current_pos: &Vec<usize>, dir: u8, current_char: char) -> bool {
        let new_pos = self.new_pos(current_pos, dir);
        let new_char = Solution::next_char(current_char);
        match new_pos {
            Some(pos) => {
                if self.word_search[pos[0]][pos[1]] == new_char {
                    if new_char == 'S' {
                        return true;
                    } else {
                        return self.direct_line_search(&pos, dir, new_char);
                    }
                } else {
                    return false;
                }
            }
            None => return false,
        }
    }

    fn new_pos(&self, current_pos: &Vec<usize>, dir: u8) -> Option<Vec<usize>> {
        let mut new_pos = current_pos.clone();
        match dir {
            7 => {
                if new_pos[0] == 0 {
                    return None;
                };
                if new_pos[1] == 0 {
                    return None;
                };
                new_pos[0] -= 1;
                new_pos[1] -= 1;
            }
            8 => {
                if new_pos[0] == 0 {
                    return None;
                };
                new_pos[0] -= 1
            }
            9 => {
                if new_pos[0] == 0 {
                    return None;
                };
                new_pos[0] -= 1;
                new_pos[1] += 1
            }
            4 => {
                if new_pos[1] == 0 {
                    return None;
                };
                new_pos[1] -= 1;
            }
            6 => new_pos[1] += 1,
            1 => {
                if new_pos[1] == 0 {
                    return None;
                };
                new_pos[0] += 1;
                new_pos[1] -= 1
            }
            2 => new_pos[0] += 1,
            3 => {
                new_pos[0] += 1;
                new_pos[1] += 1
            }
            _ => return None,
        }
        if self.word_search.len() <= new_pos[0] || self.word_search[0].len() <= new_pos[1] {
            return None;
        } else {
            return Some(new_pos);
        }
    }

    fn next_char(current_char: char) -> char {
        match current_char {
            'X' => 'M',
            'M' => 'A',
            'A' => 'S',
            _ => {
                panic!("Not a valid character!")
            }
        }
    }
}

#[test]
fn test_input() {
    assert_eq!(Solution::new("inputs/day_04_test.txt").part_one(), 18)
}

#[test]
fn test_part_two() {
    assert_eq!(Solution::new("inputs/day_04_test.txt").part_two(), 9)
}
