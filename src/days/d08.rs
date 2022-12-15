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

    pub fn count_visible(&self) -> u32 {
        let mut count = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                if self.is_visible((x, y)) {
                    count += 1;
                }
            }
        }
        return count;
    }

    pub fn is_visible(&self, pos: (usize, usize)) -> bool {
        let mut x = pos.0 + 1;
        let mut y = pos.1;
        let tree_height = self.get_at_pos(pos);

        while x < self.width {
            if self.get_at_pos((x, y)) >= tree_height {
                break;
            }
            x += 1;
        }
        if x == self.width {
            return true;
        }
        x = pos.0;
        y += 1;

        while y < self.height {
            if self.get_at_pos((x, y)) >= tree_height {
                break;
            }
            y += 1;
        }
        if y == self.height {
            return true;
        }
        y = pos.1;

        // x -= 1;
        while x > 0 {
            if self.get_at_pos((x, y)) >= tree_height && x != pos.0 {
                break;
            }

            x -= 1;
        }
        if x == 0 && (self.get_at_pos((x, y)) < tree_height || x == pos.0) {
            return true;
        }
        x = pos.0;

        // y -= 1;
        while y > 0 {
            if self.get_at_pos((x, y)) >= tree_height && y != pos.1 {
                break;
            }

            y -= 1;
        }
        if y == 0 && (self.get_at_pos((x, y)) < tree_height || y == pos.1) {
            return true;
        }

        return false;
    }

    pub fn get_best_score(&self) -> u32 {
        let mut out = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                let tmp = self.get_score((x, y));
                if tmp > out {
                    out = tmp;
                }
            }
        }
        return out;
    }

    pub fn get_score(&self, pos: (usize, usize)) -> u32 {
        let mut x = pos.0 + 1;
        let mut y = pos.1;
        let tree_height = self.get_at_pos(pos);
        let mut total_score = 1;
        let mut visible_trees = 1;

        while x < self.width {
            if self.get_at_pos((x, y)) >= tree_height {
                break;
            }
            visible_trees += 1;
            x += 1;
        }
        if x == self.width {
            total_score *= visible_trees;
        }
        visible_trees = 1;
        x = pos.0;
        y += 1;

        while y < self.height {
            if self.get_at_pos((x, y)) >= tree_height {
                break;
            }
            visible_trees += 1;
            y += 1;
        }
        if y == self.height {
            total_score *= visible_trees;
        }
        visible_trees = 0;
        y = pos.1;

        // x -= 1;
        while x > 0 {
            if self.get_at_pos((x, y)) >= tree_height && x != pos.0 {
                break;
            }
            visible_trees += 1;
            x -= 1;
        }
        if x == 0 && (self.get_at_pos((x, y)) < tree_height || x == pos.0) {
            total_score *= visible_trees;
        }
        visible_trees = 0;
        x = pos.0;

        // y -= 1;
        while y > 0 {
            if self.get_at_pos((x, y)) >= tree_height && y != pos.1 {
                break;
            }
            visible_trees += 1;

            y -= 1;
        }
        if y == 0 && (self.get_at_pos((x, y)) < tree_height || y == pos.1) {
            total_score *= visible_trees;
        }

        return total_score;
    }
}

pub fn solve(input: String) -> (String, String) {
    // println!("{:?}", grid_size(&input));
    let grid = Grid::new(&input);

    (
        grid.count_visible().to_string(),
        grid.get_best_score().to_string(),
    )
}
