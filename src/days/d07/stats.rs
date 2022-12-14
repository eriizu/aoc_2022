use super::file::*;

pub struct StatsMachine {
    accumulator: u32,
    boundary: u32,
}

impl StatsMachine {
    pub fn new() -> Self {
        Self {
            accumulator: 0,
            boundary: 0,
        }
    }

    fn rec_size(&mut self, dir: &Directory) -> u32 {
        let size: u32 = dir
            .children
            .iter()
            .map(|child| match child {
                File::Dir(dir) => self.rec_size(dir),
                File::Reg { name: _, size } => *size,
            })
            .sum();
        if size < self.boundary {
            self.accumulator += size;
        }
        return size;
    }

    pub fn sum_bellow_max(&mut self, dir: &Directory, max: u32) -> u32 {
        self.boundary = max;
        self.rec_size(dir);
        return self.accumulator;
    }
}
