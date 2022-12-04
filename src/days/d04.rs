// https://adventofcode.com/2022/day/4

fn one_contains_the_other(range_a: (u32, u32), range_b: (u32, u32)) -> bool {
    if range_a.0 >= range_b.0 && range_a.1 <= range_b.1 {
        true
    } else if range_a.0 <= range_b.0 && range_a.1 >= range_b.1 {
        true
    } else {
        false
    }
}

fn num_from_str_pair(input: (&str, &str)) -> Option<(u32, u32)> {
    Some((
        u32::from_str_radix(input.0, 10).ok()?,
        u32::from_str_radix(input.1, 10).ok()?,
    ))
}

pub fn solve(input: String) -> (String, String) {
    let count_conained = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(right, left): (&str, &str)| {
            (
                right.split_once('-').unwrap(),
                left.split_once('-').unwrap(),
            )
        })
        .map(|(range_a, range_b): ((&str, &str), (&str, &str))| {
            (
                num_from_str_pair(range_a).unwrap(),
                num_from_str_pair(range_b).unwrap(),
            )
        })
        .filter(|(range_a, range_b)| one_contains_the_other(*range_a, *range_b))
        .count();

    (count_conained.to_string(), "".to_owned())
}
