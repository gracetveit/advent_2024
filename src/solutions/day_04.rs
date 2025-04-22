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
