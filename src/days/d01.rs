// https://adventofcode.com/2022/day/1

fn replace_total_if_greater(greatest_totals: &mut [u32; 3], running_total: u32) {
    for total in greatest_totals {
        if *total < running_total {
            *total = running_total;
            return;
        }
    }
}

pub fn solve(input: String) -> (String, String) {
    let mut greatest_total = 0;
    let mut greatest_totals: [u32; 3] = [0, 0, 0];
    let mut totals: Vec<u32> = Vec::new();
    let mut accumulator = 0;

    for line in input.lines() {
        if line.is_empty() {
            if accumulator > greatest_total {
                greatest_total = accumulator;
            }
            replace_total_if_greater(&mut greatest_totals, accumulator);
            totals.push(accumulator);
            accumulator = 0;
        }

        match u32::from_str_radix(line, 10) {
            Ok(current) => {
                accumulator += current;
            }
            Err(_) => {}
        }
    }

    println!("{:?}", greatest_totals);
    totals.sort_by(|a, b| b.cmp(a));
    let sum = totals.iter().take(3).sum::<u32>();

    // Return the solution to part1 and part2 in a tuple.
    (greatest_total.to_string(), sum.to_string())
}
