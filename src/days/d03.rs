// https://adventofcode.com/2022/day/3

use std::mem::needs_drop;

fn priority_for_char(input: char) -> Option<u8> {
    if input.is_ascii_lowercase() {
        Some(input as u8 - 'a' as u8 + 1)
    } else if input.is_ascii_uppercase() {
        Some(input as u8 - 'A' as u8 + 27)
    } else {
        None
    }
}

fn priority_for_line(line: &str) -> Option<u8> {
    let compartments = line.split_at((line.len()) / 2);
    let duplicate_letter =
        compartments
            .0
            .chars()
            .find(|c0| match compartments.1.chars().find(|c1| c1.eq(&c0)) {
                Some(_) => true,
                None => false,
            })?;
    priority_for_char(duplicate_letter)
}

struct Group<'a> {
    rucksacks: [Option<&'a str>; 3],
}

impl<'a> Group<'a> {
    fn new() -> Self {
        Self {
            rucksacks: [None, None, None],
        }
    }

    fn append_line(&mut self, line: &'a str) {
        for item in &mut self.rucksacks {
            match item {
                Some(_) => {}
                None => {
                    *item = Some(line);
                    return;
                }
            }
        }
    }

    fn find_common(rs_a: &str, rs_b: &str, rs_c: &str) -> Option<char> {
        rs_a.chars().find(|needle_a| {
            let res_b = rs_b.chars().find(|needle_b| needle_a.eq(&needle_b));
            let res_c = rs_c.chars().find(|needle_c| needle_a.eq(&needle_c));
            match (res_b, res_c) {
                (Some(_), Some(_)) => true,
                _ => false,
            }
        })
    }

    fn reset(&mut self) {
        for item in &mut self.rucksacks {
            *item = None;
        }
    }

    pub fn process_line(&mut self, line: &'a str) -> Option<u8> {
        self.append_line(line);
        let common_char = match (self.rucksacks[0], self.rucksacks[1], self.rucksacks[2]) {
            (Some(rs_a), Some(rs_b), Some(rs_c)) => Self::find_common(rs_a, rs_b, rs_c),
            _ => None,
        };
        let res = if let Some(common_char) = common_char {
            self.reset();
            priority_for_char(common_char)
        } else {
            None
        };

        return res;
    }
}

pub fn solve(input: String) -> (String, String) {
    let mut total_priorities = 0u32;
    let mut total_badge_priorities = 0u32;
    let mut group = Group::new();
    for line in input.lines() {
        if !line.is_empty() {
            match priority_for_line(line) {
                Some(priority) => {
                    total_priorities += priority as u32;
                }
                None => {}
            }
            match group.process_line(line) {
                Some(priority) => total_badge_priorities += priority as u32,
                None => {}
            }
        }
    }

    (
        total_priorities.to_string(),
        total_badge_priorities.to_string(),
    )
}
