// https://adventofcode.com/2022/day/9

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Coords {
    pub fn manathan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn highest_diff(&self, other: &Self) -> i32 {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        if x_diff > y_diff {
            x_diff
        } else {
            y_diff
        }
    }
}

#[derive(Clone, Copy)]
struct Iteration {
    head: Coords,
    tail: Coords,
}

impl Iteration {
    pub fn new() -> Self {
        Self {
            head: Coords { x: 0, y: 0 },
            tail: Coords { x: 0, y: 0 },
        }
    }
}

struct State {
    last: Iteration,
    current: Iteration,
    visited: std::collections::HashSet<Coords>,
}

impl State {
    pub fn new() -> Self {
        Self {
            last: Iteration::new(),
            current: Iteration::new(),
            visited: std::collections::HashSet::new(),
        }
    }

    pub fn make_move(&mut self, direction: &str) {
        self.last = self.current;
        match direction {
            "U" => self.current.head.y -= 1,
            "D" => self.current.head.y += 1,
            "R" => self.current.head.x += 1,
            "L" => self.current.head.x -= 1,
            _ => {
                panic!("ouin ouin");
            }
        }
        if self.current.head.highest_diff(&self.current.tail) > 1 {
            self.current.tail = self.last.head;
        }
        self.visited.insert(self.current.tail);
    }

    pub fn make_moves(&mut self, direction: &str, distance: u32) {
        (0..distance).for_each(|_| self.make_move(direction));
    }

    pub fn parse_line(&mut self, line: &str) {
        let split = line.split_once(" ");
        match split {
            Some((direction_str, distance_str)) => {
                let distance = u32::from_str_radix(distance_str, 10).unwrap();
                self.make_moves(direction_str, distance);
            }
            None => {}
        }
    }

    pub fn number_visited(&self) -> usize {
        self.visited.len()
    }
}

pub fn solve(input: String) -> (String, String) {
    let mut state = State::new();
    for line in input.lines() {
        state.parse_line(line);
    }

    (state.number_visited().to_string(), "".into())
}
