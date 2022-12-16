// https://adventofcode.com/2022/day/8

struct Grid {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

fn grid_size(input: &str) -> (usize, usize) {
    let mut height: usize = 0;
    let mut width: usize = 0;
    for line in input.lines() {
        let len = line.len();
        if width == 0 {
            width = len;
        }
        if len != 0 && width != len {
            panic!("line width difference");
        }
        height += 1;
    }
    return (width, height);
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let size = grid_size(&input);

        let numbers: Vec<u32> = input
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        Self {
            data: numbers,
            width: size.0,
            height: size.1,
        }
    }

    pub fn get_at_pos(&self, pos: (usize, usize)) -> u32 {
        let idx = self.width * pos.1 + pos.0;
        *self.data.get(idx).unwrap()
    }

    fn is_visible_from_top(&self, pos: (usize, usize)) -> bool {
        let start_tree_height = self.get_at_pos(pos);
        for y in (0..pos.1).rev() {
            let current_tree_height = self.get_at_pos((pos.0, y));
            if current_tree_height >= start_tree_height {
                return false;
            }
        }
        return true;
    }

    fn is_visible_from_right(&self, pos: (usize, usize)) -> bool {
        let start_tree_height = self.get_at_pos(pos);
        for x in (0..pos.0).rev() {
            let current_tree_height = self.get_at_pos((x, pos.1));
            if current_tree_height >= start_tree_height {
                return false;
            }
        }
        return true;
    }

    fn is_visible_from_bottom(&self, pos: (usize, usize)) -> bool {
        let start_tree_height = self.get_at_pos(pos);
        for y in pos.1 + 1..self.height {
            let current_tree_height = self.get_at_pos((pos.0, y));
            if current_tree_height >= start_tree_height {
                return false;
            }
        }
        return true;
    }
    fn is_visible_from_left(&self, pos: (usize, usize)) -> bool {
        let start_tree_height = self.get_at_pos(pos);
        for x in pos.0 + 1..self.width {
            let current_tree_height = self.get_at_pos((x, pos.1));
            if current_tree_height >= start_tree_height {
                return false;
            }
        }
        return true;
    }

    pub fn get_both_solutions(&self) -> (u32, u32) {
        let mut visible_count = 0;
        let mut best_score = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                let current_score = self.get_tree_score(x, y);
                if current_score > best_score {
                    best_score = current_score;
                }
                if self.is_tree_visible(x, y) {
                    visible_count += 1;
                }
            }
        }
        return (visible_count, best_score);
    }

    fn is_tree_visible(&self, x: usize, y: usize) -> bool {
        self.is_visible_from_bottom((x, y))
            || self.is_visible_from_top((x, y))
            || self.is_visible_from_left((x, y))
            || self.is_visible_from_right((x, y))
    }

    fn get_tree_score(&self, x: usize, y: usize) -> u32 {
        self.count_visible_downward((x, y))
            * self.count_visible_upward((x, y))
            * self.count_visible_leftward((x, y))
            * self.count_visible_rightward((x, y))
    }

    fn count_visible_upward(&self, pos: (usize, usize)) -> u32 {
        let mut visible_count = 0;
        let start_tree_height = self.get_at_pos(pos);
        for y in (0..pos.1).rev() {
            visible_count += 1;
            let current_tree_height = self.get_at_pos((pos.0, y));
            if current_tree_height >= start_tree_height {
                break;
            }
        }
        return visible_count;
    }

    fn count_visible_downward(&self, pos: (usize, usize)) -> u32 {
        let mut visible_count = 0;
        let start_tree_height = self.get_at_pos(pos);
        for y in pos.1 + 1..self.height {
            visible_count += 1;
            let current_tree_height = self.get_at_pos((pos.0, y));
            if current_tree_height >= start_tree_height {
                break;
            }
        }
        return visible_count;
    }

    fn count_visible_leftward(&self, pos: (usize, usize)) -> u32 {
        let mut visible_count = 0;
        let start_tree_height = self.get_at_pos(pos);
        for x in (0..pos.0).rev() {
            visible_count += 1;
            let current_tree_height = self.get_at_pos((x, pos.1));
            if current_tree_height >= start_tree_height {
                break;
            }
        }
        return visible_count;
    }

    fn count_visible_rightward(&self, pos: (usize, usize)) -> u32 {
        let mut visible_count = 0;
        let start_tree_height = self.get_at_pos(pos);
        for x in pos.0 + 1..self.width {
            visible_count += 1;
            let current_tree_height = self.get_at_pos((x, pos.1));
            if current_tree_height >= start_tree_height {
                break;
            }
        }
        return visible_count;
    }
}

pub fn solve(input: String) -> (String, String) {
    let grid = Grid::new(&input);
    let (visible_count, best_score) = grid.get_both_solutions();
    return (visible_count.to_string(), best_score.to_string());
}
