// https://adventofcode.com/2022/day/6

fn carry_slide(carry: &mut [char], new_character: char) {
    let mut idx = 1;
    let len = carry.len();
    while idx < len {
        carry[idx - 1] = carry[idx];
        idx += 1;
    }
    carry[len - 1] = new_character;
}

fn carry_are_all_different(carry: &[char]) -> bool {
    for character in carry.iter() {
        if carry.iter().filter(|c| *c == character).count() != 1 {
            return false;
        }
    }
    return true;
}

pub fn solve(input: String) -> (String, String) {
    let mut idx = 5;
    let mut carry: [char; 4] = [' ', ' ', ' ', ' '];
    let mut carry_step_2: [char; 14] = [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ];
    let mut chars = input.chars();
    let mut step_1_out = None;
    let mut step_2_out = None;

    carry[0] = chars.next().unwrap();
    carry[1] = chars.next().unwrap();
    carry[2] = chars.next().unwrap();
    carry[3] = chars.next().unwrap();

    while let Some(character) = chars.next() {
        match step_1_out {
            None => {
                carry_slide(&mut carry, character);
                if carry_are_all_different(&carry) {
                    step_1_out = Some(idx)
                }
            }
            _ => {}
        }
        match step_2_out {
            None if idx >= 14 => {
                carry_slide(&mut carry_step_2, character);
                if carry_are_all_different(&carry_step_2) {
                    step_2_out = Some(idx);
                }
            }
            _ => {}
        }
        match (step_1_out, step_2_out) {
            (Some(_), Some(_)) => break,
            _ => {}
        }
        idx += 1;
    }

    (
        step_1_out.unwrap().to_string(),
        step_2_out.unwrap().to_string(),
    )
}

#[cfg(test)]
mod test {}
