use std::collections::HashMap;

#[derive(Debug)]
struct File<'a> {
    name: &'a str,
    size: u64,
}

impl<'a> From<&'a str> for File<'a> {
    fn from(s: &'a str) -> File<'a> {
        let mut file = s.split(" ");
        File {
            size: file.next().unwrap().parse::<u64>().unwrap(),
            name: file.next().unwrap(),
        }
    }
}

impl<'a> File<'a> {
    fn size(&self) -> u64 {
        self.size
    }
}

#[derive(Debug)]
struct Folder<'a> {
    name: &'a str,
    files: Vec<File<'a>>,
    contains: Vec<String>,
}

impl<'a> From<&'a str> for Folder<'a> {
    fn from(s: &'a str) -> Folder<'a> {
        let file = s.split(" ");
        Folder {
            name: file.last().unwrap(),
            files: vec![],
            contains: vec![],
        }
    }
}

const MAX_SIZE: u64 = 100000;

impl<'a> Folder<'a> {
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
            let folder = Folder::from(line);
            let parent = current_path.iter().rev().next().unwrap();
            folders_map
                .entry(current_path.join(""))
                .or_insert(Folder {
                    name: &parent,
                    files: vec![],
                    contains: vec![],
                })
                .contains
                .push(current_path.join("") + &folder.name);

            folders_map.insert(current_path.join("") + &folder.name, folder);
        }
        if line.starts_with(|c: char| c.is_digit(10)) {
            let folder = current_path.iter().last().unwrap();
            folders_map
                .entry(current_path.join(""))
                .or_insert(Folder {
                    name: folder,
                    files: vec![],
                    contains: vec![],
                })
                .files
                .push(File::from(line));
        }
    }
    dbg!(&folders_map
        .iter()
        .map(|(key, folder)| (key, &folder.name, &folder.contains))
        .collect::<Vec<_>>());

    folders_map
        .iter()
        .map(|(_, folder)| folder.size(&folders_map))
        .filter(|size| *size <= MAX_SIZE)
        .sum()
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
}
