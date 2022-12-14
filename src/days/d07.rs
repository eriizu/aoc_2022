// https://adventofcode.com/2022/day/7

mod file;
mod stats;

use file::*;
use stats::*;

struct Parser {
    current_path: Vec<String>,
    pub root: Directory,
}

impl Parser {
    fn new(root: Directory) -> Self {
        Self {
            current_path: vec![],
            root,
        }
    }

    fn parse_command(&mut self, mut split: std::str::Split<&str>) {
        let cmd = split.next();
        let arg = split.next();
        match (cmd, arg) {
            (Some(cmd), Some(arg)) if cmd == "cd" && arg == ".." => {
                self.current_path.pop();
                // todo!("go back");
            }
            (Some(cmd), Some(arg)) if cmd == "cd" && arg == "/" => {
                self.current_path.clear();
                // todo!("go to root");
            }
            (Some(cmd), Some(arg)) if cmd == "cd" => {
                self.current_path.push(arg.to_owned());
                // todo!("go to specific dir");
            }
            (Some(cmd), _) if cmd == "ls" => {
                // todo!();
            }
            _ => {}
        }
    }

    fn create_file(name: &str, size_str: &str) -> Option<File> {
        let size = u32::from_str_radix(size_str, 10).ok()?;
        Some(File::new_regular_file(name, size))
    }

    fn get_dir_mut<'a>(dir: &'a mut Directory, path: &[String]) -> Option<&'a mut Directory> {
        if path.len() == 0 {
            return Some(dir);
        }
        let tmp = dir.children.iter_mut().find(|child| match child {
            File::Dir(child) if child.name == path[0] => true,
            _ => false,
        });
        if let Some(File::Dir(tmp)) = tmp {
            return Self::get_dir_mut(tmp, &path[1..]);
        } else {
            return None;
        }
    }

    fn current_wd_mut(&mut self) -> Option<&mut Directory> {
        Self::get_dir_mut(&mut self.root, &self.current_path)
    }

    pub fn parse_line(&mut self, line: &str) {
        let mut split: std::str::Split<&str> = line.split(" ");
        match split.next() {
            Some(val) if val == "$" => self.parse_command(split),
            Some(val) if val == "dir" => {
                if let Some(dir_name) = split.next() {
                    let tmp = self.current_wd_mut();
                    if let Some(dir) = tmp {
                        dir.insert_file(File::new_directory(dir_name));
                    } else {
                        eprintln!("couldn't find current dir");
                    }
                } else {
                    eprintln!("cannot create dir with a dir_name");
                }
            }
            Some(size) => {
                if let Some(file_name) = split.next() {
                    if let Some(file) = Self::create_file(file_name, size) {
                        let tmp = self.current_wd_mut();
                        if let Some(dir) = tmp {
                            dir.insert_file(file);
                        } else {
                            eprintln!("couldn't find current dir");
                        }
                    } else {
                        eprintln!("failed to create file with args {size} and {file_name}");
                    }
                } else {
                    eprintln!("missing argument to file creating");
                }
            }
            _ => {}
        }
    }
}

pub fn solve(input: String) -> (String, String) {
    let root = File::new_directory("/");
    let mut total_size = 0;
    if let File::Dir(directory) = root {
        let mut parser = Parser::new(directory);
        for line in input.lines() {
            parser.parse_line(line);
        }

        let mut stats = StatsMachine::new();
        total_size = stats.sum_bellow_max(&parser.root, 100000);
    }
    (total_size.to_string(), "".into())
}
