// https://adventofcode.com/2022/day/7

struct Directory {
    name: String,
    children: Vec<File>,
}

impl Directory {
    pub fn new(name: String) -> Self {
        Self {
            name,
            children: vec![],
        }
    }

    pub fn rec_size(&self) -> u32 {
        self.children
            .iter()
            .map(|child| match child {
                File::Dir(dir) => dir.rec_size(),
                File::Reg { name: _, size } => *size,
            })
            .sum()
    }

    pub fn insert_file(&mut self, file: File) {
        self.children.push(file);
    }
}

enum File {
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

pub fn solve(_input: String) -> (String, String) {
    let root = File::new_directory("/");
    let test_file = File::new_regular_file("tata", 5000123);
    let mut total_size = 0;
    if let File::Dir(mut directory) = root {
        directory.insert_file(test_file);
        total_size = directory.rec_size()
    }
    (total_size.to_string(), "".into())
}
