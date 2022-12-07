use std::{collections::HashMap, convert::Infallible, str::FromStr};

#[derive(Debug)]
struct File {
    size: u64,
}

impl FromStr for File {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<File, Infallible> {
        let mut file = s.split(" ");
        Ok(File {
            size: file.next().unwrap().parse::<u64>().unwrap(),
        })
    }
}

impl File {
    fn size(&self) -> u64 {
        self.size
    }
}

#[derive(Debug)]
struct Folder {
    files: Vec<File>,
    contains: Vec<String>,
}

impl Folder {
    fn new() -> Self {
        Folder {
            files: vec![],
            contains: vec![],
        }
    }

    fn inner_folders_size(&self, folders_map: &HashMap<String, Folder>) -> u64 {
        self.contains
            .iter()
            .map(|id| folders_map.get(&id.to_string()).unwrap().size(folders_map))
            .sum::<u64>()
    }

    fn size(&self, folders_map: &HashMap<String, Folder>) -> u64 {
        self.files.iter().map(|file| file.size()).sum::<u64>()
            + self.inner_folders_size(folders_map)
    }
}

const MAX_SIZE: u64 = 100000;

pub fn part_1(input: String) -> u64 {
    let mut folders_map: HashMap<String, Folder> = HashMap::new();
    let mut current_path: Vec<&str> = vec!["/"];

    for line in input.lines() {
        if line.contains("$ cd") {
            let path = line.split(" ").last().unwrap();
            match path {
                ".." => {
                    current_path.pop();
                }
                "/" => {
                    current_path = vec!["/"];
                }
                _ => {
                    current_path.push(path);
                }
            }
        }

        if line.starts_with("dir") {
            let folder_name = line.split(" ").last().unwrap();
            folders_map
                .entry(current_path.join(""))
                .or_insert(Folder {
                    files: vec![],
                    contains: vec![],
                })
                .contains
                .push(current_path.join("") + folder_name);

            folders_map.insert(current_path.join("") + folder_name, Folder::new());
        }
        if line.starts_with(|c: char| c.is_digit(10)) {
            folders_map
                .entry(current_path.join(""))
                .or_insert(Folder {
                    files: vec![],
                    contains: vec![],
                })
                .files
                .push(line.parse::<File>().unwrap());
        }
    }

    folders_map
        .iter()
        .map(|(_, folder)| folder.size(&folders_map))
        .filter(|size| *size <= MAX_SIZE)
        .sum()
}

const TOTAL_SPACE: u64 = 70000000;
const REQUIRED_SPACE: u64 = 30000000;

pub fn part_2(input: String) -> u64 {
    let mut folders_map: HashMap<String, Folder> = HashMap::new();
    let mut current_path: Vec<&str> = vec!["/"];

    for line in input.lines() {
        if line.contains("$ cd") {
            let path = line.split(" ").last().unwrap();
            match path {
                ".." => {
                    current_path.pop();
                }
                "/" => {
                    current_path = vec!["/"];
                }
                _ => {
                    current_path.push(path);
                }
            }
        }

        if line.starts_with("dir") {
            let folder_name = line.split(" ").last().unwrap();
            folders_map
                .entry(current_path.join(""))
                .or_insert(Folder {
                    files: vec![],
                    contains: vec![],
                })
                .contains
                .push(current_path.join("") + folder_name);

            folders_map.insert(current_path.join("") + folder_name, Folder::new());
        }
        if line.starts_with(|c: char| c.is_digit(10)) {
            folders_map
                .entry(current_path.join(""))
                .or_insert(Folder {
                    files: vec![],
                    contains: vec![],
                })
                .files
                .push(line.parse::<File>().unwrap());
        }
    }

    let used_space = folders_map.get("/").unwrap().size(&folders_map);
    let free_space = TOTAL_SPACE - used_space;
    let needed_space = REQUIRED_SPACE - free_space;

    folders_map
        .iter()
        .filter_map(|(_, folder)| {
            let folder_size = folder.size(&folders_map);
            if folder_size > needed_space {
                return Some(folder_size);
            }
            None
        })
        .min()
        .unwrap()
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
    fn largest_files_combined() {
        assert_eq!(part_1(INPUT.to_string()), 95437);
    }

    #[test]
    fn smallest_file_to_free_space() {
        assert_eq!(part_2(INPUT.to_string()), 24933642);
    }
}
