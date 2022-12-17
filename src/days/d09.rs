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
    knots: [Coords; 9],
}

impl Iteration {
    pub fn new() -> Self {
        Self {
            head: Coords { x: 0, y: 0 },
            tail: Coords { x: 0, y: 0 },
            knots: [
                Coords { x: 0, y: 0 },
                Coords { x: 0, y: 0 },
                Coords { x: 0, y: 0 },
                Coords { x: 0, y: 0 },
                Coords { x: 0, y: 0 },
                Coords { x: 0, y: 0 },
                Coords { x: 0, y: 0 },
                Coords { x: 0, y: 0 },
                Coords { x: 0, y: 0 },
            ],
        }
    }
}

struct State {
    last: Iteration,
    current: Iteration,
    visited: std::collections::HashSet<Coords>,
    visited9: std::collections::HashSet<Coords>,
}

impl State {
    pub fn new() -> Self {
        Self {
            last: Iteration::new(),
            current: Iteration::new(),
            visited: std::collections::HashSet::new(),
            visited9: std::collections::HashSet::new(),
        }
    }

    // pub fn

    pub fn make_move(last: &mut Iteration, current: &mut Iteration, direction: &str) -> bool {
        match direction {
            "U" => current.head.y -= 1,
            "D" => current.head.y += 1,
            "R" => current.head.x += 1,
            "L" => current.head.x -= 1,
            _ => {
                panic!("ouin ouin");
            }
        }
        Self::move_knot(&last.head, current.head, &mut current.tail)
    }

    pub fn get_mod_to_do(head: &Coords, tail: &Coords) -> Coords {
        let mut out = Coords { x: 0, y: 0 };
        if head.x > tail.x {
            out.x = 1;
        } else if head.x < tail.x {
            out.x = -1;
        }
        if head.y > tail.y {
            out.y = 1;
        } else if head.y < tail.y {
            out.y = -1;
        }
        return out;
    }

    pub fn move_knot(_last_head: &Coords, current_head: Coords, current_tail: &mut Coords) -> bool {
        if current_head.highest_diff(current_tail) > 1 {
            let the_mod_to_do = Self::get_mod_to_do(&current_head, &current_tail);
            current_tail.x += the_mod_to_do.x;
            current_tail.y += the_mod_to_do.y;
            return the_mod_to_do.x != 0 || the_mod_to_do.y != 0;
        }
        return false;
    }

    pub fn make_moves(&mut self, direction: &str, distance: u32) {
        self.visited.insert(self.current.tail);
        (0..distance).for_each(|_| {
            self.last = self.current;
            if Self::make_move(&mut self.last, &mut self.current, direction) {
                self.visited.insert(self.current.tail);
                self.current.knots[0] = self.current.tail;
                for i in 0..8 {
                    if !Self::move_knot(
                        &self.last.knots[i],
                        self.current.knots[i],
                        &mut self.current.knots[i + 1],
                    ) {
                        break;
                    }
                }
                self.visited9.insert(self.current.knots[8]);
            }
        });
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
    pub fn number_visited9(&self) -> usize {
        self.visited9.len()
    }
}

pub fn solve(input: String) -> (String, String) {
    let mut state = State::new();
    for line in input.lines() {
        state.parse_line(line);
    }

    (
        state.number_visited().to_string(),
        state.number_visited9().to_string(),
    )
}
