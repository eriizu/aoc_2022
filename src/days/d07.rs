// https://adventofcode.com/2022/day/7

mod file;
mod parser;
mod stats;

use file::*;
use parser::*;
use stats::*;

pub fn solve(input: String) -> (String, String) {
    let root = File::new_directory("/");
    let mut step_1_out = 0;
    let mut step_2_out = 0;
    if let File::Dir(directory) = root {
        let mut parser = Parser::new(directory);
        for line in input.lines() {
            parser.parse_line(line);
        }

        let mut stats = StatsMachine::new();
        step_1_out = stats.sum_bellow_max(&parser.root, 100000);
        step_2_out = stats.size_of_smallest_to_remove(&parser.root, 70_000_000, 30_000_000);
    }
    (step_1_out.to_string(), step_2_out.to_string())
}
