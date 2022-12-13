use std::str::FromStr;

#[derive(Debug)]
enum Line {
    CommandInput(Command),
    CommandOutput(FileSystemObject),
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        if s.starts_with('$') {
            Line::CommandInput(Command::from_str(s).unwrap())
        } else {
            Line::CommandOutput(FileSystemObject::from_str(s).unwrap())
        }
    }
}

#[derive(Debug)]
enum Command {
    ChangeDirectory(String),
    List,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command_parts: Vec<_> = s.split(' ').collect();
        let command = match command_parts[1] {
            "cd" => Command::ChangeDirectory(command_parts[2].to_string()),
            "ls" => Command::List,
            _ => return Err(format!("Unknown command line: {}", s)),
        };
        Ok(command)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NodeId {
    index: usize,
}

impl From<usize> for NodeId {
    fn from(index: usize) -> Self {
        Self { index }
    }
}

#[derive(Debug, Clone)]
struct FileSystemObject {
    _name: String,
    _type: FileSystemObjectType,
    parent: Option<NodeId>,
    size: i32,
    children: Vec<NodeId>,
}

impl FromStr for FileSystemObject {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("dir") {
            let (_, directory_name) = s.split_once(' ').unwrap();
            Ok(FileSystemObject {
                _name: directory_name.to_string(),
                _type: FileSystemObjectType::Directory,
                parent: None,
                size: 0,
                children: vec![],
            })
        } else {
            let (file_size, file_name) = s.split_once(' ').unwrap();
            Ok(FileSystemObject {
                _name: file_name.to_string(),
                _type: FileSystemObjectType::File,
                parent: None,
                size: file_size.parse::<i32>().unwrap(),
                children: vec![],
            })
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FileSystemObjectType {
    Directory,
    File,
}

#[derive(Debug)]
struct FileSystem {
    nodes: Vec<FileSystemObject>,
}

fn build_filesystem(lines: &[Line]) -> FileSystem {
    let mut nodes: Vec<FileSystemObject> = vec![];

    let mut line_iterator = lines.iter();

    // an overly complex way to just start with `$ cd /`
    let _current_directory = {
        let line = line_iterator.next().unwrap();
        match line {
            Line::CommandInput(command) => match command {
                Command::ChangeDirectory(dir) => {
                    let root = FileSystemObject {
                        _name: dir.to_string(),
                        _type: FileSystemObjectType::Directory,
                        parent: None,
                        size: 0,
                        children: vec![],
                    };
                    nodes.push(root.clone());
                    root
                }
                _ => panic!("first line not change directory command: {:?}", &line),
            },
            _ => panic!("first line not a command input! Got: {:?}", &line),
        }
    };

    let mut current_directory_node_id = NodeId { index: 0 }; // we start at `/`
    let mut previous_directories: Vec<NodeId> = vec![];
    previous_directories.push(NodeId { index: 0 });

    for line in line_iterator {
        let next_index = nodes.len();
        match line {
            Line::CommandInput(command) => match command {
                Command::ChangeDirectory(dir) => {
                    if dir == ".." {
                        current_directory_node_id =
                            previous_directories.pop().unwrap_or(NodeId { index: 0 })
                    } else {
                        let new_dir = FileSystemObject {
                            _name: dir.to_string(),
                            _type: FileSystemObjectType::Directory,
                            parent: Some(current_directory_node_id),
                            size: 0,
                            children: vec![],
                        };

                        // set our current directory as the directory we just moved in to
                        // current_directory = new_dir.clone();

                        // remember our previous directories, for when `cd ..` is called an arbitrary amount of times.... r.i.p.
                        previous_directories.push(current_directory_node_id);

                        current_directory_node_id = NodeId { index: next_index };

                        // push onto our file system
                        nodes.push(new_dir);
                    }
                }
                Command::List => {
                    // println!("listing contents of directory: {:?}", &current_directory);
                }
            },
            Line::CommandOutput(object) => {
                // println!(
                //     "Adding child {} to current directory {:?}",
                //     next_index, current_directory
                // );

                // update the parent object to have the listed object as a child
                nodes[current_directory_node_id.index]
                    .children
                    .push(next_index.into());

                // update the size of all parents
                let mut parent_node = Some(current_directory_node_id);
                while let Some(pn) = parent_node {
                    nodes[pn.index].size += object.size;
                    parent_node = nodes[pn.index].parent;
                }

                // set the current object's parent as the current directory
                let mut obj = object.clone();
                obj.parent = Some(current_directory_node_id);
                nodes.push(obj);
            }
        }
    }

    FileSystem { nodes }
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<Line> {
    let lines = input.lines().map(|line| line.into()).collect();
    lines
}

#[aoc(day7, part1)]
fn part1(input: &[Line]) -> i32 {
    // println!("{:#?}", input);
    // println!("---------------------------------------------------");
    let file_system = build_filesystem(input);
    // println!("{:#?}", file_system);
    file_system
        .nodes
        .iter()
        .filter(|node| node._type == FileSystemObjectType::Directory && node.size <= 100000)
        .map(|node| node.size)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[Line]) -> i32 {
    let file_system = build_filesystem(input);

    let total_disk_space = 70_000_000;
    let target_disk_space = 30_000_000;
    let unused_space = total_disk_space - file_system.nodes[0].size;

    let mut directory_sizes: Vec<i32> = file_system
        .nodes
        .iter()
        .filter(|node| {
            node._type == FileSystemObjectType::Directory
                && node.size >= target_disk_space - unused_space
        })
        .map(|node| node.size)
        .collect();

    directory_sizes.sort();

    for size in directory_sizes {
        if size >= target_disk_space - unused_space {
            return size;
        }
    }

    panic!("oh no! upgrade isn't possible");
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "$ cd /
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
7214296 k";

    #[test]
    fn test_part1() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(95437, part1(&parsed_input))
    }

    #[test]
    fn test_part2() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(24933642, part2(&parsed_input))
    }
}
