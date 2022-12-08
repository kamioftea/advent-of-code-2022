---
day: 7
tags: post
header: 'Day 7: Tuning Trouble'
---
Continuing to fix the broken comms device given to me by the elves I now need to clear up enough space to install the
latest updates. I have the output of using `cd` and `ls` to describe the current filesystem contents. I need to be
able to convert this into a format I can analyse.

## Parsing the console output

The example data shows a file system with a some directories and files in the root, and onc subdirectory and a bunch of
other files deeper in the structure.

```text
$ cd /
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
7214296 k
```

I need to get these into a structured form, so I first creat an enum to hold the possible types. I can ignore the la 
lines as the useful output there is the output lines that follow that command. I also need to be able to represent a
File and a Directory. I'll show what structure they came to have here, but note that originally I had a single 
`children` property in directory, but I always found I needed to access one or the other so this ended up easier to work
with.

```rust
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

#[derive(Eq, PartialEq, Debug, Clone)]
enum Command {
    PushDir(String),
    PopDir,
    RootDir,
    AddDir(Directory),
    AddFile(File),
}
```

Now I need to be able to turn the console output (puzzle input), into this representation. I can use some pattern 
matching here. Some lines (i.e. `& ls`) will be ignored, so I use the TryFrom trait to allow this. The final case if
the parse to `usize` works I know it's a line representing a file in the format `<size> <name>`, otherwise I can pass on
the parse error.

```rust
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
                file_size
                    .parse::<usize>()
                    .map(|size: usize| 
                        AddFile(File { size, name: name.to_string() })
                    )
            }
        }
    }
}
```

Using `flat_map` over a result type drops the Errors and leaves me with a list of only the valid commands.

```rust
fn parse_commands(input: &String) -> Vec<Command> {
    input.lines().flat_map(Command::try_from).collect()
}
```

Building the test cases for today's puzzle was quite tedious, but having them really helped guide my solution, so it was
definitely worth it.

```rust
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
```

## Building a filesystem

The system is built as a state machine, with the state being the tree of directories and files, and the current working 
directory. I can represent this as a `struct` and use pattern matching to map each command to an update to the internal 
state.

I attempted to have the path be a vector of pointers to the directories within the directory tree, but I wasn't work out
how to prove to the compiler that the directory structure it was pointing to couldn't be deallocated whilst the path 
pointers still existed, so I ended up referencing them by name and navigating to the right node each time. A bit 
inefficient, but I didn't notice a performance problem, so I'm happy to leave it like this. There's also some error
handling missing here, but I know the puzzle input will be valid, so it's not a problem in this context.

```rust
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
            RootDir => { self.path = vec![]; }
            PopDir => { self.path.pop(); }
            PushDir(dir_name) => {
                let Directory { sub_dirs, .. } = self.current_dir();
                let maybe_dir = 
                    sub_dirs.iter()
                            .find(|Directory { name, .. }| name == &dir_name);
                
                if let Some(_) = maybe_dir {
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
                dir.sub_dirs.iter_mut()
                   .find(|Directory { name, .. }| name == dir_name)
                   .unwrap(),
        )
    }
}
```

Again writing the test case was a bit tedious, but worth the effort.

```rust
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
```

## Part 1 - Find small directories

The first task is to find all the small directories. As there is a tree structure involved I immediately reach for a 
recursive function. There is some interesting complexity in that I need to add the directory size to the list of 
directories I'm building as well as pass the total size of the directory up to the next level so that it can be included
in its parent's total. Handily when querying a sub_directory for its list of directory sizes, the size of that 
directory is always going to be last in the list, so that can be used to pass both pieces of needed information in one.

```rust
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
// ...
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
```

The final output needed for the puzzle can then be obtained by calling `dir_sizes` on the root.

```rust
fn get_small_dirs_size_sum(fs: &FileSystem) -> usize {
    fs.root.dir_sizes().iter().filter(|&&size| size <= 100_000).sum()
}

#[test]
fn can_sum_small_dirs() {
    assert_eq!(get_small_dirs_size_sum(&sample_filesystem()), 95437)
}
```

That passing I can now apply everything to the puzzle input.

```rust
pub fn run() {
    let contents = 
        fs::read_to_string("res/day-7-input").expect("Failed to read file");

    let file_system = FileSystem::from(parse_commands(&contents));

    println!(
        "The sum of small directory sizes is: {}", 
        get_small_dirs_size_sum(&file_system)
    );
}
// The sum of small directory sizes is: 1886043 
```

## Part 2 - Deleting a directory

Finally, I now have to solve the original issue. I need to find the smallest possible directory that I can delete to 
free up the needed space. Having modelled the domain, these final steps nicely fall out of what is already implemented.

1. `dir_sizes` on the root gives me the list of candidates, but also the total amount of space used because the last 
   entry in the list is by construction the size of the root folder contents.
2. This can be used to calculate the current free space, and the difference between this and the target free space 
   gives the minimum size I need to delete.
3. I can then filter the sizes, dropping any below that threshold, and take the minimum from the directories remaining.

```rust
fn find_directory_size_to_delete(fs: &FileSystem) -> usize {
    let sizes = fs.root.dir_sizes();
    let total_used = sizes.last().unwrap_or(&0);
    let total_free = 70_000_000 - total_used;
    let to_free = 30_000_000 - total_free;

    *sizes.iter().filter(|&&size| size >= to_free).min().unwrap_or(&0)
}
// ...
#[test]
fn can_find_dir_to_delete() {
    assert_eq!(find_directory_size_to_delete(&sample_filesystem()), 24933642)
}
```

That passes, so I can update the run method and complete the day

```rust
pub fn run() {
    let contents = 
        fs::read_to_string("res/day-7-input").expect("Failed to read file");

    let file_system = FileSystem::from(parse_commands(&contents));

    println!(
        "The sum of small directory sizes is: {}", 
        get_small_dirs_size_sum(&file_system)
    );

    println!(
        "The size of the directory selected for deletion is: {}", 
        find_directory_size_to_delete(&file_system)
    );
}
// The sum of small directory sizes is: 1886043
// The size of the directory selected for deletion is: 3842121
//
// Finished in 1.44ms
```
