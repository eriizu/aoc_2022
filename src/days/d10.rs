// https://adventofcode.com/2022/day/10

enum Instruction {
    Addx(i32),
    Noop,
}

const REPORT_AT: [i32; 6] = [20, 60, 100, 140, 180, 220];

struct Reporter<'a> {
    next_report_iter: std::slice::Iter<'a, i32>,
    next_report: i32,
    accumulator: i32,
}

impl<'a> Reporter<'a> {
    pub fn new() -> Self {
        let mut iter = REPORT_AT.iter();
        let next_report = match iter.next() {
            Some(val) => *val,
            None => 0,
        };
        Self {
            next_report_iter: iter,
            next_report,
            accumulator: 0,
        }
    }
    pub fn report(&mut self, tick: i32, register: i32) {
        if tick == self.next_report {
            self.accumulator += register * tick;
            self.next_report = match self.next_report_iter.next() {
                Some(val) => *val,
                None => 0,
            };
        }
    }
}

struct State<'a, 'b> {
    register: i32,
    tick: i32,
    instruction: Option<Instruction>,
    sleep_cycles: u32,
    reporter: Reporter<'a>,
    commands: std::str::Lines<'b>,
    done: bool,
    screen_line: String,
    sprite_position: usize,
}

impl<'a, 'b> State<'a, 'b> {
    pub fn new(commands: std::str::Lines<'b>) -> Self {
        let screen = ".".repeat(40);
        Self {
            register: 1,
            tick: 0,
            instruction: None,
            sleep_cycles: 0,
            reporter: Reporter::new(),
            commands,
            done: false,
            screen_line: screen,
            sprite_position: 0,
        }
    }

    pub fn reset_line(&mut self) {
        self.screen_line = ".".repeat(40);
    }

    pub fn write_on_screen_line(&mut self) {
        match usize::try_from((self.tick % 40) - 1) {
            Ok(px_idx) if px_idx < self.screen_line.len() - 1 => {
                let replace_with =
                    if px_idx >= self.sprite_position && px_idx < self.sprite_position + 3 {
                        "#"
                    } else {
                        "."
                    };
                self.screen_line
                    .replace_range(px_idx..px_idx + 1, replace_with);
            }
            _ => {}
        }
    }

    pub fn process_line(&mut self, line: &str) {
        if line == "noop" {
            self.instruction = Some(Instruction::Noop);
            self.sleep_cycles = 1;
        } else if let Some(val) = line.strip_prefix("addx ") {
            self.instruction = Some(Instruction::Addx(match val.parse() {
                Ok(val) => val,
                _ => 0,
            }));
            self.sleep_cycles = 2;
        } else {
            eprintln!("failed to processes line: {}", line);
        }
    }

    pub fn next_cycle(&mut self) {
        self.tick += 1;

        if self.tick != 1 && self.tick % 40 == 1 {
            println!("{}", self.screen_line);
            self.reset_line();
            self.sprite_position = 0;
        }
        self.write_on_screen_line();

        match self.instruction {
            None => match self.commands.next() {
                Some(command) => self.process_line(command),
                None => self.done = true,
            },
            _ => {}
        }

        self.reporter.report(self.tick, self.register);

        if self.sleep_cycles != 0 {
            self.sleep_cycles -= 1;
            if self.sleep_cycles == 0 {
                match self.instruction {
                    Some(Instruction::Addx(val)) => {
                        self.instruction = None;
                        self.register += val;
                        self.sprite_position = match usize::try_from(self.register) {
                            Ok(val) => val,
                            Err(_) => 1,
                        } - 1;
                    }
                    Some(Instruction::Noop) => {
                        self.instruction = None;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn run(&mut self) {
        while !self.done {
            self.next_cycle();
        }
    }
}

pub fn solve(input: String) -> (String, String) {
    let lines: std::str::Lines = input.lines();
    let mut state = State::new(lines);
    state.run();
    (state.reporter.accumulator.to_string(), "".into())
}

#[cfg(test)]
mod test {}
