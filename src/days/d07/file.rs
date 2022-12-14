pub struct Directory {
    pub name: String,
    pub children: Vec<File>,
    size: u32,
}

impl Directory {
    pub fn new(name: String) -> Self {
        Self {
            name,
            children: vec![],
            size: 0,
        }
    }

    pub fn rec_size(&self) -> u32 {
        let size: u32 = self
            .children
            .iter()
            .map(|child| match child {
                File::Dir(dir) => dir.rec_size(),
                File::Reg { name: _, size } => *size,
            })
            .sum();
        return size;
    }

    pub fn rec_size_at_most(&self, max: u32) -> u32 {
        let size: u32 = self
            .children
            .iter()
            .map(|child| match child {
                File::Dir(dir) => dir.rec_size(),
                _ => 0,
            })
            .sum();
        if self.size > max {
            return size;
        } else {
            return self.size;
        }
    }

    pub fn insert_file(&mut self, file: File) {
        self.children.push(file);
    }
}

pub enum File {
    Dir(Directory),
    Reg { name: String, size: u32 },
}

impl File {
    pub fn new_directory(name: &str) -> File {
        File::Dir(Directory::new(name.to_owned()))
    }

    pub fn new_regular_file(name: &str, size: u32) -> File {
        File::Reg {
            name: name.to_owned(),
            size,
        }
    }
}
