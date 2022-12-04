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

fn is_between(a: u32, b: u32, nb: u32) -> bool {
    nb >= a && nb <= b
}

fn has_overlap(range_a: (u32, u32), range_b: (u32, u32)) -> bool {
    // range_a.0.be
    is_between(range_b.0, range_b.1, range_a.0)
        || is_between(range_b.0, range_b.1, range_a.1)
        || is_between(range_a.0, range_a.1, range_b.0)
        || is_between(range_a.0, range_a.1, range_b.1)
}

fn num_from_str_pair(input: (&str, &str)) -> Option<(u32, u32)> {
    Some((
        u32::from_str_radix(input.0, 10).ok()?,
        u32::from_str_radix(input.1, 10).ok()?,
    ))
}

pub fn solve(input: String) -> (String, String) {
    let iter = input
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
        });

    let count_conained = iter
        .clone()
        .filter(|(range_a, range_b)| one_contains_the_other(*range_a, *range_b))
        .count();

    let count_overlap = iter
        .filter(|(range_a, range_b)| has_overlap(*range_a, *range_b))
        .count();

    (count_conained.to_string(), count_overlap.to_string())
}

#[cfg(test)]
mod tests {

    #[test]
    fn no_overlap() {
        let overlaps = super::has_overlap((2, 3), (4, 5));
        assert_eq!(overlaps, false);

        let overlaps = super::has_overlap((2, 4), (6, 8));
        assert_eq!(overlaps, false);

        let overlaps = super::has_overlap((4, 5), (2, 3));
        assert_eq!(overlaps, false);

        let overlaps = super::has_overlap((6, 8), (2, 4));
        assert_eq!(overlaps, false);
    }

    #[test]
    fn overlap() {
        let overlaps = super::has_overlap((5, 7), (7, 9));
        assert_eq!(overlaps, true);

        let overlaps = super::has_overlap((7, 9), (5, 7));
        assert_eq!(overlaps, true);

        let overlaps = super::has_overlap((2, 8), (3, 7));
        assert_eq!(overlaps, true);

        let overlaps = super::has_overlap((3, 7), (2, 8));
        assert_eq!(overlaps, true);

        let overlaps = super::has_overlap((6, 6), (4, 6));
        assert_eq!(overlaps, true);

        let overlaps = super::has_overlap((4, 6), (6, 6));
        assert_eq!(overlaps, true);
    }
}
