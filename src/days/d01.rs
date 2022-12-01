// https://adventofcode.com/2022/day/1

fn replace_total_if_greater(greatest_totals: &mut [u32; 3], running_total: u32) {
    greatest_totals.sort();
    for total in greatest_totals {
        if *total < running_total {
            *total = running_total;
            return;
        }
    }
}

pub fn solve(input: String) -> (String, String) {
    let mut greatest_totals: [u32; 3] = [0, 0, 0];
    let mut accumulator = 0;

    for line in input.lines() {
        if line.is_empty() {
            replace_total_if_greater(&mut greatest_totals, accumulator);
            accumulator = 0;
        }

        match u32::from_str_radix(line, 10) {
            Ok(current) => {
                accumulator += current;
            }
            Err(_) => {}
        }
    }
    replace_total_if_greater(&mut greatest_totals, accumulator);

    return (
        greatest_totals.iter().max().as_deref().unwrap().to_string(),
        greatest_totals.iter().sum::<u32>().to_string(),
    );
}
