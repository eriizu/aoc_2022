// https://adventofcode.com/2022/day/3

fn get_priority(input: char) -> Option<u8> {
    if input.is_ascii_lowercase() {
        Some(input as u8 - 'a' as u8 + 1)
    } else if input.is_ascii_uppercase() {
        Some(input as u8 - 'A' as u8 + 27)
    } else {
        None
    }
}

fn priority_for_line(line: &str) -> Option<u8> {
    let compartments = line.split_at((line.len()) / 2);
    let duplicate_letter =
        compartments
            .0
            .chars()
            .find(|c0| match compartments.1.chars().find(|c1| c1.eq(&c0)) {
                Some(_) => true,
                None => false,
            })?;
    get_priority(duplicate_letter)
}

pub fn solve(input: String) -> (String, String) {
    let mut total_priorities = 0u32;
    for line in input.lines() {
        if !line.is_empty() {
            match priority_for_line(line) {
                Some(priority) => {
                    total_priorities += priority as u32;
                }
                None => {}
            }
        }
    }

    (total_priorities.to_string(), "".into())
}
