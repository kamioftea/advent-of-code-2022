//! This is my solution for [Advent of Code - Day 7 - _No Space Left On Device_](https://adventofcode.com/2022/day/7)
//!
//!

use std::fs;
use std::num::ParseIntError;
use itertools::Itertools;
use crate::day_7::Command::{AddDir, AddFile, PopDir, PushDir, RootDir};

#[derive(Eq, PartialEq, Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Directory {
    name: String,
    sub_dirs: Vec<Directory>,
    files: Vec<File>,
}

impl From<&str> for Directory {
    fn from(name: &str) -> Self {
        Directory { name: name.to_string(), sub_dirs: Vec::new(), files: Vec::new() }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Command {
    PushDir(String),
    PopDir,
    RootDir,
    AddDir(Directory),
    AddFile(File),
}

impl TryFrom<&str> for Command {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            str if str.starts_with("$ cd /") => Ok(RootDir),
            str if str.starts_with("$ cd ..") => Ok(PopDir),
            str if str.starts_with("$ cd") => {
                let dir = str.split_whitespace().dropping(2).next().unwrap();
                Ok(PushDir(dir.to_string()))
            }
            str if str.starts_with("dir") => {
                let dir = str.split_whitespace().dropping(1).next().unwrap();
                Ok(AddDir(Directory::from(dir)))
            }
            str => {
                let (file_size, name) = str.split_once(" ").unwrap();
                file_size.parse::<usize>().map(|size: usize| AddFile(File { size, name: name.to_string() }))
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct FileSystem {
    root: Directory,
    path: Vec<String>,
}

impl From<Vec<Command>> for FileSystem {
    fn from(commands: Vec<Command>) -> Self {
        let root: Directory = Directory::from("/");
        let mut file_system = FileSystem { root, path: vec![] };

        for command in commands {
            file_system.apply(command)
        }

        file_system.apply(RootDir);

        file_system
    }
}

impl FileSystem {
    fn apply(&mut self, command: Command) {
        match command {
            RootDir => {
                self.path = vec![];
            }
            PopDir => { self.path.pop(); }
            PushDir(dir_name) => {
                let Directory { sub_dirs, .. } = self.current_dir();
                if let Some(_) = sub_dirs.iter().find(|Directory { name, .. }| name == &dir_name) {
                    self.path.push(dir_name)
                }
            }
            AddDir(dir) => {
                self.current_dir().sub_dirs.push(dir)
            }
            AddFile(file) => {
                self.current_dir().files.push(file)
            }
        }
    }

    fn current_dir(&mut self) -> &mut Directory {
        self.path.iter().fold(
            &mut self.root,
            |dir, dir_name|
                dir.sub_dirs.iter_mut().find(|Directory { name, .. }| name == dir_name).unwrap(),
        )
    }
}

impl Directory {
    fn dir_sizes(&self) -> Vec<usize> {
        let mut sizes = Vec::new();
        let mut size = 0;
        for dir in &self.sub_dirs {
            let sub_sizes = dir.dir_sizes();
            sub_sizes.iter().for_each(|&s| sizes.push(s));
            size = size + sizes.last().unwrap_or(&0);
        }

        for file in &self.files {
            size = size + file.size
        }

        sizes.push(size);

        sizes
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-7-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 7.
pub fn run() {
    let contents = fs::read_to_string("res/day-7-input").expect("Failed to read file");

    let file_system = FileSystem::from(parse_commands(&contents));

    println!("The sum of small directory sizes is: {}", get_small_dirs_size_sum(&file_system));

    println!("The size of the directory selected for deletion is: {}", find_directory_size_to_delete(&file_system));
}

fn parse_commands(input: &String) -> Vec<Command> {
    input.lines().flat_map(Command::try_from).collect()
}

fn get_small_dirs_size_sum(fs: &FileSystem) -> usize {
    fs.root.dir_sizes().iter().filter(|&&size| size <= 100000 ).sum()
}

fn find_directory_size_to_delete(fs: &FileSystem) -> usize {
    let sizes = fs.root.dir_sizes();
    let total_used = sizes.last().unwrap_or(&0);
    let total_free = 70_000_000 - total_used;
    let to_free = 30_000_000 - total_free;

    *sizes.iter().filter(|&&size| size >= to_free).min().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use crate::day_7::Command::*;
    use crate::day_7::{Command, Directory, File, FileSystem, find_directory_size_to_delete, get_small_dirs_size_sum, parse_commands};

    fn sample_commands() -> Vec<Command> {
        vec![
            RootDir,
            AddDir(Directory::from("a")),
            AddFile(File { name: "b.txt".to_string(), size: 14848514 }),
            AddFile(File { name: "c.dat".to_string(), size: 8504156 }),
            AddDir(Directory::from("d")),
            PushDir("a".to_string()),
            AddDir(Directory::from("e")),
            AddFile(File { name: "f".to_string(), size: 29116 }),
            AddFile(File { name: "g".to_string(), size: 2557 }),
            AddFile(File { name: "h.lst".to_string(), size: 62596 }),
            PushDir("e".to_string()),
            AddFile(File { name: "i".to_string(), size: 584 }),
            PopDir,
            PopDir,
            PushDir("d".to_string()),
            AddFile(File { name: "j".to_string(), size: 4060174 }),
            AddFile(File { name: "d.log".to_string(), size: 8033020 }),
            AddFile(File { name: "d.ext".to_string(), size: 5626152 }),
            AddFile(File { name: "k".to_string(), size: 7214296 }),
        ]
    }

    #[test]
    fn can_parse_command() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k".to_string();

        assert_eq!(parse_commands(&input), sample_commands())
    }

    fn sample_filesystem<'a>() -> FileSystem {
        let root = Directory {
            name: "/".to_string(),
            sub_dirs: vec![
                Directory {
                    name: "a".to_string(),
                    sub_dirs: vec![
                        Directory {
                            name: "e".to_string(),
                            sub_dirs: vec![],
                            files: vec![File { name: "i".to_string(), size: 584 }],
                        },
                    ],
                    files: vec![
                        File { name: "f".to_string(), size: 29116 },
                        File { name: "g".to_string(), size: 2557 },
                        File { name: "h.lst".to_string(), size: 62596 },
                    ],
                },
                Directory {
                    name: "d".to_string(),
                    sub_dirs: vec![],
                    files: vec![
                        File { name: "j".to_string(), size: 4060174 },
                        File { name: "d.log".to_string(), size: 8033020 },
                        File { name: "d.ext".to_string(), size: 5626152 },
                        File { name: "k".to_string(), size: 7214296 },
                    ],
                },
            ],
            files: vec![
                File { name: "b.txt".to_string(), size: 14848514 },
                File { name: "c.dat".to_string(), size: 8504156 },
            ],
        };

        FileSystem {
            root,
            path: vec![],
        }
    }

    #[test]
    fn can_build_fs_from_commands() {
        assert_eq!(
            FileSystem::from(sample_commands()),
            sample_filesystem()
        )
    }

    #[test]
    fn can_list_dir_sizes() {
        assert_eq!(
            sample_filesystem().root.dir_sizes(),
            vec![
                584,
                94853,
                24933642,
                48381165,
            ]
        )
    }

    #[test]
    fn can_sum_small_dirs() {
        assert_eq!(get_small_dirs_size_sum(&sample_filesystem()), 95437)
    }

    #[test]
    fn can_find_dir_to_delete() {
        assert_eq!(find_directory_size_to_delete(&sample_filesystem()), 24933642)
    }
}
