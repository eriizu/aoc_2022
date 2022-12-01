// https://adventofcode.com/2022/day/1

pub fn solve(input: String) -> (String, String) {
    let mut greatest_total = 0;
    let mut accumulator = 0;

    for line in input.lines() {
        if line.is_empty() {
            if accumulator > greatest_total {
                greatest_total = accumulator
            }
            accumulator = 0;
        }

        match i32::from_str_radix(line, 10) {
            Ok(current) => {
                accumulator += current;
            }
            Err(_) => {}
        }
    }

    // Return the solution to part1 and part2 in a tuple.
    (greatest_total.to_string(), "".into())
}
