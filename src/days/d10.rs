// https://adventofcode.com/2022/day/10

enum Instruction {
    Addx(u32),
    Noop,
}

const REPORT_AT: [u32; 6] = [20, 60, 100, 140, 180, 220];

struct Reporter<'a> {
    next_report_iter: std::slice::Iter<'a, u32>,
    next_report: u32,
    accumulator: u32,
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
    pub fn report(&mut self, tick: u32, register: u32) {
        if tick == self.next_report {
            self.accumulator += register;
            self.next_report = match self.next_report_iter.next() {
                Some(val) => *val,
                None => 0,
            };
        }
    }
}

struct State<'a, 'b> {
    register: u32,
    tick: u32,
    instruction: Option<Instruction>,
    sleep_cycles: u32,
    reporter: Reporter<'a>,
    commands: std::str::Lines<'b>,
    done: bool,
}

impl<'a, 'b> State<'a, 'b> {
    pub fn new(commands: std::str::Lines<'b>) -> Self {
        Self {
            register: 0,
            tick: 0,
            instruction: None,
            sleep_cycles: 0,
            reporter: Reporter::new(),
            commands,
            done: false,
        }
    }

    pub fn signal_strengh(&self) -> u32 {
        self.register * self.tick
    }

    pub fn process_line(&mut self, line: &str) {
        if line == "noop" {
            self.instruction = Some(Instruction::Noop);
            self.sleep_cycles = 1;
        } else if let Some(val) = line.strip_prefix("addx ") {
            self.instruction = Some(Instruction::Addx(match u32::from_str_radix(val, 10) {
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
        if self.sleep_cycles != 0 {
            self.sleep_cycles -= 1;
        } else {
            match self.instruction {
                Some(Instruction::Addx(value)) => {
                    self.register += value;
                    self.instruction = None;
                }
                Some(Instruction::Noop) => {
                    self.instruction = None;
                }
                _ => {}
            }
            match self.commands.next() {
                Some(command) => self.process_line(command),
                None => self.done = true,
            }
        }
        self.reporter.report(self.tick, self.register);
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
