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

    pub fn dump(&self) {
        for stack in &self.stacks {
            eprintln!("{:?}", stack);
        }
    }

    fn move_one(&mut self, src: usize, dest: usize) -> Option<()> {
        let value = self.stacks.get_mut(src)?.pop_front()?;
        self.stacks.get_mut(dest)?.push_front(value);
        Some(())
    }

    pub fn do_action(&mut self, action: MoveAction) {
        for _ in 0..action.quantity {
            self.move_one(action.source, action.destination);
        }
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
    let mut stacks = Stacks::from_lines(&mut lines);
    stacks.dump();
    eprintln!("{}", stacks.get_top_of_each());
    for line in lines {
        match MoveAction::from_str(line) {
            Some(action) => stacks.do_action(action),
            None => {
                eprintln!("invalid action '{}'", line)
            }
        }
    }

    (stacks.get_top_of_each(), "".into())
}
