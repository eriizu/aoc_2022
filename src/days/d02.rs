// https://adventofcode.com/2022/day/2

#[derive(PartialEq, Debug)]
enum MoveType {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Move {
    kind: MoveType,
    beats_kind: MoveType,
    letter_1: char,
    letter_2: char,
    associated_score: u32,
}

const MOVE_TABLE: [Move; 3] = [
    Move {
        kind: MoveType::Rock,
        beats_kind: MoveType::Scissors,
        letter_1: 'A',
        letter_2: 'X',
        associated_score: 1,
    },
    Move {
        kind: MoveType::Paper,
        beats_kind: MoveType::Rock,
        letter_1: 'B',
        letter_2: 'Y',
        associated_score: 2,
    },
    Move {
        kind: MoveType::Scissors,
        beats_kind: MoveType::Paper,
        letter_1: 'C',
        letter_2: 'Z',
        associated_score: 3,
    },
];

fn extract_letters(input: &str) -> Option<(char, char)> {
    let mut letters = input.split(" ");
    let a = letters.next()?.chars().nth(0)?;
    let b = letters.next()?.chars().nth(0)?;

    return Some((a, b));
}

fn round_score(p1_move: &Move, p2_move: &Move) -> u32 {
    let win_score = if p1_move.beats_kind == p2_move.kind {
        0
    } else if p1_move.kind == p2_move.kind {
        3
    } else {
        6
    };

    return win_score + p2_move.associated_score;
}

pub fn solve(input: String) -> (String, String) {
    let mut total_score = 0u32;
    for line in input.lines() {
        if let Some((input_letter_1, input_letter_2)) = extract_letters(&line) {
            let p1_move_match = MOVE_TABLE
                .iter()
                .find(|item| item.letter_1.eq(&input_letter_1));
            let p2_move_match = MOVE_TABLE
                .iter()
                .find(|item| item.letter_2.eq(&input_letter_2));
            match (p1_move_match, p2_move_match) {
                (Some(p1), Some(p2)) => total_score += round_score(p1, p2),
                (Some(_), None) => eprintln!("match missing for p2 {}", input_letter_2),
                (None, Some(_)) => eprintln!("match missing for p1 {}", input_letter_1),
                _ => eprintln!("no match found for {}, {}", input_letter_1, input_letter_2),
            }
        }
    }

    (total_score.to_string(), "".into())
}
