use super::file::*;

#[derive(Debug)]
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

    fn rec_smallest_above(&mut self, dir: &Directory) -> u32 {
        let size: u32 = dir
            .children
            .iter()
            .map(|child| match child {
                File::Dir(dir) => self.rec_smallest_above(dir),
                File::Reg { name: _, size } => *size,
            })
            .sum();
        if size >= self.boundary && self.accumulator > size {
            self.accumulator = size;
        }
        return size;
    }

    pub fn size_of_smallest_to_remove(
        &mut self,
        dir: &Directory,
        total_avail_size: u32,
        target_free_size: u32,
    ) -> u32 {
        self.accumulator = 0;
        let total_used = dir.rec_size();

        let free = total_avail_size - total_used;
        let min_to_remove = target_free_size - free;
        self.boundary = min_to_remove;

        self.accumulator = total_used;
        self.rec_smallest_above(dir);
        return self.accumulator;
    }
}
