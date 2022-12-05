// https://adventofcode.com/2022/day/5

use std::{collections::VecDeque, str::Lines};

struct Stacks {
    stacks: Vec<VecDeque<char>>,
}

impl Stacks {
    fn parse_setup_line(&mut self, line: &str) {
        let mut offset = 0;
        for i in 0..self.stacks.len() {
            let letter = line.chars().nth(offset + 1).unwrap();
            if letter.is_ascii_alphabetic() {
                self.stacks[i].push_back(letter);
            }
            offset += 4;
        }
    }

    pub fn from_lines(lines: &mut Lines) -> Self {
        let mut out = Self { stacks: vec![] };
        let line = lines.next().unwrap();
        let number_of_stacks = (line.len() + 1) / 4;
        out.stacks.resize(number_of_stacks, VecDeque::new());
        out.parse_setup_line(line);
        for line in lines {
            if line.is_empty() {
                return out;
            }
            out.parse_setup_line(line);
        }
        return out;
    }

    fn move_one(&mut self, src: usize, dest: usize) -> Option<()> {
        let value = self.stacks.get_mut(src)?.pop_front()?;
        self.stacks.get_mut(dest)?.push_front(value);
        Some(())
    }

    pub fn do_action_part1(&mut self, action: &MoveAction) {
        for _ in 0..action.quantity {
            self.move_one(action.source, action.destination);
        }
    }

    fn move_all_rec(&mut self, count: usize, src: usize, dest: usize) -> Option<()> {
        let value = self.stacks.get_mut(src)?.pop_front()?;
        if count > 1 {
            self.move_all_rec(count - 1, src, dest);
        }
        self.stacks.get_mut(dest)?.push_front(value);
        Some(())
    }

    pub fn do_action_part2(&mut self, action: &MoveAction) {
        self.move_all_rec(action.quantity, action.source, action.destination);
    }

    pub fn get_top_of_each(&self) -> String {
        let a: String = self
            .stacks
            .iter()
            .map(|item| item.get(0).unwrap_or(&' '))
            .collect();
        return a;
    }
}

struct MoveAction {
    quantity: usize,
    source: usize,
    destination: usize,
}

impl MoveAction {
    pub fn from_str(line: &str) -> Option<Self> {
        let mut split = line.split_ascii_whitespace();
        split.next()?;
        let quantity = usize::from_str_radix(split.next()?, 10).ok()?;
        split.next()?;
        let source = usize::from_str_radix(split.next()?, 10).ok()? - 1;
        split.next()?;
        let destination = usize::from_str_radix(split.next()?, 10).ok()? - 1;
        Some(Self {
            quantity,
            source,
            destination,
        })
    }
}

pub fn solve(input: String) -> (String, String) {
    let mut lines = input.lines();
    let mut stacks_part1 = Stacks::from_lines(&mut lines);
    let mut stacks_part2 = {
        let mut tmp_lines = input.lines();
        Stacks::from_lines(&mut tmp_lines)
    };

    for line in lines {
        match MoveAction::from_str(line) {
            Some(action) => {
                stacks_part1.do_action_part1(&action);
                stacks_part2.do_action_part2(&action);
            }
            None => {
                eprintln!("invalid action '{}'", line)
            }
        }
    }

    (
        stacks_part1.get_top_of_each(),
        stacks_part2.get_top_of_each(),
    )
}
